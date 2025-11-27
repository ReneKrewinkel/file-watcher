use anyhow::{Result, anyhow};
use clap::Parser;
use globset::Glob;
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use shell_words;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::mpsc::channel;

mod config;
mod tools;

/// CLI arguments for the watcher
#[derive(Parser, Debug)]
#[command(name = "file-watcher", version)]
struct Args {
    /// Glob pattern to watch, e.g. "_*.scss" or "src/**/*.ts"
    #[arg(long = "file-type")]
    file_pattern: String,

    /// Command to run on change, e.g. "yarn scss"
    #[arg(long = "command")]
    command: String,

    /// Comma-separated list of marker files to identify project root.
    /// e.g. "package.json,Cargo.toml,composer.json"
    #[arg(long = "root-files", default_value = "package.json,Cargo.toml")]
    root_files: String,
}

/// Find project root by walking upward until one of the root marker files is found.
/// Checks the start directory first, then each parent.
fn find_project_root(start: &Path, root_files: &[String]) -> Option<PathBuf> {
    let mut current = Some(start.to_path_buf());

    while let Some(dir) = current {
        for file_name in root_files {
            if dir.join(file_name).exists() {
                return Some(dir.clone());
            }
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }

    None
}

/// Parse the quoted command string into program + arguments
fn split_command(cmd: &str) -> Result<(String, Vec<String>)> {
    let parts =
        shell_words::split(cmd).map_err(|e| anyhow!("Could not parse command '{}': {}", cmd, e))?;

    if parts.is_empty() {
        return Err(anyhow!("Empty command provided"));
    }

    Ok((parts[0].clone(), parts[1..].to_vec()))
}

/// Run the custom command in the given root directory
async fn run_custom_command(root: &Path, program: &str, args: &[String]) -> Result<()> {
    println!("▶️ Running command: {} {:?}", program, args);

    let status = tokio::process::Command::new(program)
        .args(args)
        .current_dir(root)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?
        .wait()
        .await?;

    if !status.success() {
        return Err(anyhow!("Command exited with code {:?}", status.code()));
    }

    println!("✅ Command completed successfully");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = config::app::get_config()?;
    tools::logo::show(&config);

    println!("🚀 file-watcher started…");
    println!("🎯 File pattern: {}", args.file_pattern);
    println!("🛠 Command: {}", args.command);
    println!("📂 Root-marker files: {}", args.root_files);

    let (program, cmd_args) = split_command(&args.command)?;

    let root_files: Vec<String> = args
        .root_files
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let cwd = std::env::current_dir()?;
    let root = find_project_root(&cwd, &root_files).ok_or_else(|| {
        anyhow!(
            "Could not locate project root. Searched for files: {:?}",
            root_files
        )
    })?;

    println!("📁 Project root detected at: {:?}", root);

    let glob = Glob::new(&args.file_pattern)
        .map_err(|e| anyhow!("Invalid glob pattern '{}': {}", args.file_pattern, e))?
        .compile_matcher();

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| tx.send(res).expect("watch event send failed"),
        notify::Config::default(),
    )?;

    watcher.watch(&root, RecursiveMode::Recursive)?;
    println!("👀 Watching files under project root…");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                if matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                ) {
                    for path in &event.paths {
                        // make the path relative to root if possible
                        let relative = path.strip_prefix(&root).unwrap_or(path);
                        if glob.is_match(relative) {
                            println!("🔄 Match detected: {:?}", path);
                            if let Err(e) = run_custom_command(&root, &program, &cmd_args).await {
                                eprintln!("❌ Error during command run: {}", e);
                            }
                            // break out after first matching path
                            break;
                        }
                    }
                }
            }
            Ok(Err(e)) => eprintln!("Watcher error: {:?}", e),
            Err(_) => break,
        }
    }

    Ok(())
}
