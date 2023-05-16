build: clean codecheck test
	cargo build
	RUSTFLAGS="-C target-cpu=native" cargo build --release

clean:
	cargo clean

codecheck:
	cargo clippy

test:
	cargo test

fmt:
	cargo fmt

fix:
	cargo fix --allow-dirty