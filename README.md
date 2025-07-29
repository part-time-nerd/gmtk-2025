# gmtk-2025

run natively for development:
```sh
cargo run
```

## Compiling for windows

- Follow the [msvc setup instructions](https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#first-time-setup-msvc) to get the toolchain configured.

Had to disable gamepad inputs for now, but I can create a window through wine at least:

```sh
cargo build --target=x86_64-pc-windows-msvc --release
WGPU_BACKEND=vulkan wine target/x86_64-pc-windows-msvc/release/gmtk-2025.exe
```

NOTE: There's some weirdness with keychron keyboard being recognized as a gamepad. If its connected when running the windows build through wine it will panic and crash. So unplug it and use a different keyboard. Also gamepads just don't work with it + wine -- it will crash if you have one plugged in. Not sure about bluetooth. Make sure to unplug it before testing. Should probably verify that it doesn't crash on an actual windows PC.

## Compiling for web

- <https://bevy-cheatbook.github.io/platforms/wasm.html>

You will need to install the runner and configure `~/.cargo/config.toml`:
```sh
cargo install wasm-server-runner
```

Then you can run the app in the browser:
```sh
cargo run --target wasm32-unknown-unknown
```

TODO: consider using [trunk](https://github.com/trunk-rs/trunk) for deployment.
