.PHONY: run
run: web/dist
	cargo run

web/dist: $(shell find web/src -type f) web/bun.lock
	@cd web && bun run build
	# @touch web/build
