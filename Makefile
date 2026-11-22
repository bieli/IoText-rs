.PHONY: all build test example check-nostd msrv doc fmt-check clippy deny ci coverage fuzz fuzz-regressions

all: build test

build:
	cargo build --all-targets

test:
	cargo test --all-features

example:
	cargo run --example basic_usage

check-nostd:
	rustup target add thumbv6m-none-eabi
	cargo build --lib --no-default-features --features serde --target thumbv6m-none-eabi

msrv:
	cargo +1.85 check --lib --all-features

doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo clippy --all-targets --no-default-features -- -D warnings

deny:
	cargo deny check

# Local equivalent of the main CI jobs (.github/workflows/ci.yml).
ci: fmt-check clippy test doc check-nostd msrv deny
	cargo test --no-default-features

coverage:
	cargo tarpaulin --all-features --out html

FUZZ_TARGET_TRIPLE ?= x86_64-unknown-linux-gnu
FUZZ_SECONDS ?= 60
fuzz:
	cargo +nightly fuzz run --target $(FUZZ_TARGET_TRIPLE) parse -- -max_total_time=$(FUZZ_SECONDS) -dict=fuzz/iotext.dict -max_len=512
	cargo +nightly fuzz run --target $(FUZZ_TARGET_TRIPLE) builder -- -max_total_time=$(FUZZ_SECONDS) -max_len=2048

fuzz-regressions:
	cargo +nightly fuzz run --target $(FUZZ_TARGET_TRIPLE) parse fuzz/regressions/parse -- -runs=0
