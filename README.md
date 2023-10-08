# files-to-clipboard

So, basically, this repo contains a singular function which uses windows API to copy files to the clipboard.

Actually, kinda misleading, technically speaking the files themselves don't get copied to the clipboard, it's the paths that get copied, which point to the files.

## Usage

This isn't on crates.io or anything because I don't think it's in a production ready state (maybe that will change one day who knows).

So just copy everything that's in `src/main.rs`.

AND be sure to put this in your `Cargo.toml`.

```
[dependencies.windows]
version = "0.51.1"
features = [
    "Win32_System_DataExchange",
    "Win32_Foundation",
    "Win32_UI_Shell",
    "Win32_System_Memory"
]
```

Also this only works on windows if that wasn't already clear.
