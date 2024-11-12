all:
	build
build:
	cargo build --release
test:
	cargo test
format:
	cargo fmt --all
lint:
	cargo clippy -- -D warnings
example:
	cargo run -- parse --file queries/sample.sql
doc:
	cargo doc --open
