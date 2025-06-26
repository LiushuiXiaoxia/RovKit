

clean:
	cargo clean

.PHONY: build
build: clean
	cargo build --release

test: build
	cargo test

package: build test
	cargo package
	cargo publish --dry-run --registry default

publish: package
	cargo publish --registry default