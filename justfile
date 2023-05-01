default:
	@just --list

server:
	cargo watch -q -c -w src/ -x run

client:
	cargo watch -q -c -w tests/ -x "test -q -- --nocapture --color=always"
