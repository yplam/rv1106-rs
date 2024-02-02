# Rust for rv1106

## Build rust toolchain from source

``` shell
git checkout df871fbf053de3a855398964cd05fadbe91cf4fd
./x check --target=armv7-unknown-linux-uclibceabihf
./x build --stage 2 --target=armv7-unknown-linux-uclibceabihf
```
