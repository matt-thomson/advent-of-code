# just manual: https://github.com/casey/just/#readme

_default:
  @just --list

lint:
	cargo clippy --locked -- -D warnings

test:
	cargo test --locked

run day input:
  cargo run --locked --bin {{day}} {{input}}
