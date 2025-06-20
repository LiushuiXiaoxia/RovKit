

clean:
	cargo clean

buildAll: clean
	cargo build --release

package: buildAll
	cargo package
	cargo publish --dry-run

publish: package
	CARGO_REGISTRIES_CRATES_IO_PROTOCOL=https cargo publish