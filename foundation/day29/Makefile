.PHONY: fmt clippy test bench nextest

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

bench:
	cargo bench --bench vec_capacity

nextest:
	cargo nextest run
