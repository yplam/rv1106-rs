.PHONY: clean rv1106-sys examples

BUILD_MODE := --release
VERSION=latest

clean:
	cargo clean

rv1106-sys:
	cargo +rv110x build -p rv1106-sys --target=armv7-unknown-linux-uclibceabihf $(BUILD_MODE)
	cargo +rv110x build --examples -p rv1106-sys --target=armv7-unknown-linux-uclibceabihf $(BUILD_MODE)

examples:
	cargo +rv110x build -p rv1106-examples --target=armv7-unknown-linux-uclibceabihf $(BUILD_MODE)