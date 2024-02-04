# Rust for rv1106

Rust bindings for Rockchip rv1106

## Features

- [x] Build Rust toolchain and docker image.
- [x] rv1106-sys
  - [x] rockit binding
  - [x] rknpu2 binding
  - [ ] move detection
  - [ ] occlusion detection
- [ ] rv1106-rs


## Build rust toolchain from source

``` shell
git checkout df871fbf053de3a855398964cd05fadbe91cf4fd
./x check --target=armv7-unknown-linux-uclibceabihf
./x build --stage 2 --target=armv7-unknown-linux-uclibceabihf
```
