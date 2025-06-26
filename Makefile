

clean:
	cargo clean

buildAll: clean
	cargo build --release
	cargo test

package: buildAll
	cargo package
	cargo publish --dry-run --registry default

publish: package
	cargo publish --registry default