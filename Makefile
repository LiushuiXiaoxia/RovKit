

clean:
	cargo clean

buildAll: clean
	cargo build --release

test: buildAll
	cargo test

package: buildAll test
	cargo package
	cargo publish --dry-run --registry default

publish: package
	cargo publish --registry default