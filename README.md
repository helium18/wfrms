# Deprecated
This repo is unfortunately no longer being maintained. Writing shell scripts in rust is not a good idea 

## wfrms
Suspend your computer and play an alarm on waking up!

![](https://i.imgur.com/x2QlKGg.png)

Use your PC as an alarm while not worrying about running out of charge. 

Features
1. If the time entered is more than 4 hours then PC hibernates.
2. Else it's suspended (or slept).
3. On waking up, an audio file is played on loop, till the user closes the player.

## Install 

Download the [deb](https://github.com/amd176/wfrms/releases/tag/0.1.0) package from releases. 

## Build 

Dependencies: `cargo` and `vlc`

```
git clone https://github.com/amd176/wfrms.git
cd wfrms/src
cargo build --release
cd ..
cd target
./wfrms
```

`wfrms` can be run from the `~/wfrms/target/release/` directory or you can just paste the rust binary in one of the `$PATH` folders so that it can opened quickly.

## Uninstall 

Built: Just remove the `~/wfrms` directory, and the rust binary which you've pasted in one of the `$PATH` folders. If you're facing a problem, consider raising an issue. 

Deb: Uninstall it via the original deb file.

## Upcoming updates 
- [ ] Possibly a GUI.
- [ ] More functionality.

---
Consider raising an issue if you find bugs. It's still in beta!

