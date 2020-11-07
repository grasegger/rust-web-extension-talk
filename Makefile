.PHONY: build run-in-firefox

build:
	cargo clean -p rust-web-extension; cargo build

run-in-firefox:
	web-ext run -s pkg --bc -u 'http://rust-linz.at' -u 'https://github.com/grasegger/rust-web-extension-talk' -u 'about:debugging#/runtime/this-firefox'