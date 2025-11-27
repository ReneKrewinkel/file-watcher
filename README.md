# file-watcher (fw)

I love the command line — and I love my own CLI tools even more.

`file-watcher`, or `fw` for short, is a no-fluff daemon that watches **only the files you care about**, inside **only the project you’re working in**, and executes **exactly the command** you specify when those files change.

---

## 🚀 Why this exists

If you’ve ever:

- watched **_*.scss** files change,  
- executed `yarn scss` in the hope your full build pipeline won’t choke,  
- wandered into sub-folders and wondered why the watcher stopped working…

… then this tool is for you.

---

## ✏️ Features at a glance

- Watch any file pattern with a glob:  
  `--file-type "_*.scss"` or `--file-type "src/**/*.ts"`  
- Run any shell command you like:  
  `--command "yarn scss"` or `--command "php -l"`  
- Detect your project root automatically by locating one of your root files, e.g. `package.json`, `Cargo.toml`  
- Recursively monitor everything under the project root  
- Minimal overhead. Zero surprises.

---

## ⚙️ Installation & build

```sh
cargo build --release
