.PHONY: shell
shell:
	docker-compose run --rm app \
		/bin/bash

.PHONY: build-release
build-release:
	docker-compose run --rm app \
		cargo build --release

# https://manpages.debian.org/testing/cargo/cargo-test.1.en.html
.PHONY: test-unit
test-unit:
	docker-compose run --rm app \
		cargo test -p ci --all-targets
