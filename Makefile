.PHONY: build run-in-firefox

build:
	cargo clean -p rust-web-extension; cargo build

run-in-firefox: build
	web-ext run -s pkg --bc -u 'http://rust-linz.at' -u 'https://github.com/grasegger/oro' -u 'about:debugging#/runtime/this-firefox'