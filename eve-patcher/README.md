# Client patcher

This tool is used to modify the client's blue.dll, refer to <https://github.com/stschake/blue_patcher> and `utils/dev/patcher.cpp` of the [original project](https://github.com/EvEmu-Project/evemu_Crucible)

NOTICE: In general, this tool should be cross-built for Windows. Otherwise, you may need to manually copy `blue.dll` to the Linux device for patching.

```bash
pacman -S mingw-w64
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

## Usage

`./eve-patcher.exe <path to blue.dll>`

Or you can just drag `blue.dll` onto this executable file.
