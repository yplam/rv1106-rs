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
export PATH=/home/xxx/xxx/tools/linux/toolchain/arm-rockchip830-linux-uclibcgnueabihf/bin:$PATH
git clone https://github.com/rust-lang/rust.git
cd rust
git checkout 1.72.1                     
git submodule update --init --recursive
# ./x setup --target=armv7-unknown-linux-uclibceabihf
./x check --target=armv7-unknown-linux-uclibceabihf
./x build library --target=armv7-unknown-linux-uclibceabihf
./x build --stage 2 --target=armv7-unknown-linux-uclibceabihf
mv build/host/stage2 rust-1.72.1-rv1106
tar -cvJf rust-1.72.1-rv1106.tar.xz rust-1.72.1-rv1106
```
config.toml

``` toml
changelog-seen = 2
[build]
target = ["armv7-unknown-linux-uclibceabihf"]
build-stage = 2
test-stage = 2
doc-stage = 2
extended = true

[llvm]
download-ci-llvm = false
[rust]
channel = "nightly"
download-rustc = false

[dist]
compression-profile = "balanced"

[target.armv7-unknown-linux-uclibceabihf]
linker = "arm-rockchip830-linux-uclibcgnueabihf-gcc"
cc = "arm-rockchip830-linux-uclibcgnueabihf-gcc"

```
