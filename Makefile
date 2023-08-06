init:
	touch Storage.txt

help:
	cargo run -q

add:
	cargo run -q add "$(todo)"

todos:
	cargo run -q list

test:
	cargo test -q

lint:
	cargo clippy --fix --bin "rust-example-app"

lint-force:
	cargo clippy --fix --bin "rust-example-app" --allow-dirty --allow-staged