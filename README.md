# Rust for rv1106

Rust bindings for Rockchip rv1106

## Features

- [x] Build Rust toolchain
- [x] rv1106-sys
  - [x] rockit binding
  - [x] rknpu2 binding
  - [ ] move detection
  - [ ] occlusion detection
- [x] common examples
  - [x] rustls client
  - [x] tokio example
  - [ ] https example
  - [ ] http3 example
- [ ] rv1106-rs

## Build rust toolchain from source

``` shell
export PATH=/home/xxx/xxx/tools/linux/toolchain/arm-rockchip830-linux-uclibcgnueabihf/bin:$PATH
git clone https://github.com/rust-lang/rust.git
cd rust
git checkout 1.84.1                     
git submodule update --init --recursive
# ./x setup --target=armv7-unknown-linux-uclibceabihf
./x check --target=armv7-unknown-linux-uclibceabihf
./x build library --target=armv7-unknown-linux-uclibceabihf
./x build --stage 2 --target=armv7-unknown-linux-uclibceabihf
mv build/host/stage2 rust-rv110x
tar -cvJf rust-1.84.1-rv110x.tar.xz rust-rv110x

# use
export PATH=/home/xxx/xxx/tools/linux/toolchain/arm-rockchip830-linux-uclibcgnueabihf/bin:$PATH
rustup toolchain link rv110x /opt/rust-rv110x
```

config.toml

``` toml
change-id = 133207

[build]
docs = false
extended = true

[llvm]
download-ci-llvm = false

[rust]
channel = "nightly"
download-rustc = false

[dist]
compression-profile = "balanced"
src-tarball = false
compression-formats = ["xz"]

[install]
prefix = "/opt/rust-rv110x"
sysconfdir = "etc"

[target.armv7-unknown-linux-uclibceabihf]
linker = "arm-rockchip830-linux-uclibcgnueabihf-gcc"
cc = "arm-rockchip830-linux-uclibcgnueabihf-gcc"

```
