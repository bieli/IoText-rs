all:
	make build
	make test

test:
	cargo test
build:
	cargo clean
	cargo build
ci:
	./3rd-party/reviewdog -efm='<pre><code>%E%f:%l:%c:%t:%m -efm=%E%f:%l:%c:%t:%m' -efm='%Z__END__' -efm='%C%m</code></pre>' -efm='%C%m' -efm='%C' -name=clippy -reporter=github-pr-review -filter-mode=added -fail-on-error=false -level=error -diff="git diff FETCH_HEAD"
ci-local:
	./3rd-party/reviewdog -efm='<pre><code>%E%f:%l:%c:%t:%m -efm=%E%f:%l:%c:%t:%m' -efm='%Z__END__' -efm='%C%m</code></pre>' -efm='%C%m' -efm='%C' -name=clippy -reporter=local -filter-mode=added -fail-on-error=true -level=info -diff="git diff FETCH_HEAD"
coverage:
	cargo tarpaulin --out html

