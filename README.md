# gmtk-2025

## Compiling for windows

- Follow the [msvc setup instructions](https://bevy-cheatbook.github.io/setup/cross/linux-windows.html#first-time-setup-msvc) to get the toolchain configured.

Had to disable gamepad inputs for now, but I can create a window through wine at least:

```sh
cargo build --target=x86_64-pc-windows-msvc --release
WGPU_BACKEND=vulkan wine target/x86_64-pc-windows-msvc/release/gmtk-2025.exe
```

You can also just run natively for development:
```sh
cargo run
```

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
