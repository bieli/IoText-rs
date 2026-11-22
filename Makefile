all:
	make build
	make test

test:
	cargo test --all-features
example:
	cargo run --example basic_usage
check-nostd:
	rustup target add thumbv6m-none-eabi
	cargo build --lib --no-default-features --features serde --target thumbv6m-none-eabi
msrv:
	cargo +1.85 check --lib --all-features
FUZZ_TARGET_TRIPLE ?= x86_64-unknown-linux-gnu
FUZZ_SECONDS ?= 60
fuzz:
	cargo +nightly fuzz run --target $(FUZZ_TARGET_TRIPLE) parse -- -max_total_time=$(FUZZ_SECONDS) -dict=fuzz/iotext.dict -max_len=512
	cargo +nightly fuzz run --target $(FUZZ_TARGET_TRIPLE) builder -- -max_total_time=$(FUZZ_SECONDS) -max_len=2048
fuzz-regressions:
	cargo +nightly fuzz run --target $(FUZZ_TARGET_TRIPLE) parse fuzz/regressions/parse -- -runs=0
doc:
	RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
build:
	cargo clean
	cargo build
ci:
	./3rd-party/reviewdog -efm='<pre><code>%E%f:%l:%c:%t:%m -efm=%E%f:%l:%c:%t:%m' -efm='%Z__END__' -efm='%C%m</code></pre>' -efm='%C%m' -efm='%C' -name=clippy -reporter=github-pr-review -filter-mode=added -fail-on-error=false -level=error -diff="git diff FETCH_HEAD"
ci-local:
	./3rd-party/reviewdog -efm='<pre><code>%E%f:%l:%c:%t:%m -efm=%E%f:%l:%c:%t:%m' -efm='%Z__END__' -efm='%C%m</code></pre>' -efm='%C%m' -efm='%C' -name=clippy -reporter=local -filter-mode=added -fail-on-error=true -level=info -diff="git diff FETCH_HEAD"
coverage:
	cargo tarpaulin --out html
fmt-check:
	cargo fmt --all -- --check
