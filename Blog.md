---
# “Because I love the command line — here’s a tool that does exactly one thing and does it right.”

I’ve built a lot of CLI tools.  
I’ve tuned them, refactored them, forgotten them, rewrote them.  
One pattern keeps surfacing: **I want simplicity**.

I want to say:

> “Whenever **this file** changes, run **this command**.”

No bloated watchers, no huge pipeline, no “why did it run now?”. Just clarity.

Enter **file-watcher** (or `fw` for the keyboard-friendly among us).


---

## The problem

You’re in a project, you’ve got Sass partials named `_Button.style.scss`, `_Layout.style.scss`.  
You run a build script.  
You navigate into `src/components/atoms/Button/`.  
You change `_Button.style.scss`.  
Your watcher doesn’t see it because you’re “too deep”.  
Or it sees everything and now you’re building twice an hour.

You ask yourself:  

- Why is this running now?  
- Did I run it from the root?  
- Did I include the right pattern?  

It ends up being more friction than help.



## The idea behind fw

What if:

- I define a simple glob: `--file-type "*.scss"`  
- I define a command: `--command "yarn scss"`  
- And I drop the tool anywhere in my project: root, sub-folder, whatever  
- The tool lifts itself upward until it finds a **root marker** (`package.json`, `Cargo.toml`)  
- Then watches **only** files matching the glob under that root  
- When one of them changes, it runs the command  

That’s it. No surprises. No overhead.

---

## Why I built it in Rust

- I watch **many** files in many projects. I want speed.  
- For low-overhead watchers, Rust + `notify` + `globset` do the job.  
- My command execution needs to be reliable.  
- I want minimal dependencies and maximal portability.  

It’s not about “Rewrite everything in Rust” (though I do enjoy that).  
It’s about “Make this one job rock-solid”.



## How to use it

Build:

```sh
cargo build --release
```

Repo:
