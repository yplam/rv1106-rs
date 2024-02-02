.PHONY: clean rv1106-sys

BUILD_MODE := --release
VERSION=latest

clean:
	cargo clean

rv1106-sys:
	cargo +rv1106 build -p rv1106-sys --target=armv7-unknown-linux-uclibceabihf $(BUILD_MODE)
	cargo +rv1106 build --examples -p rv1106-sys --target=armv7-unknown-linux-uclibceabihf $(BUILD_MODE)
