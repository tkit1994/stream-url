all: fix fmt
fmt:
	cargo fmt

fix:
	cargo clippy --fix --allow-dirty --allow-staged


.PHONY: fmt fix
