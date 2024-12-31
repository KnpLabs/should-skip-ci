TEST_FLAGS ?=

ifneq (,$(CI))
    TEST_FLAGS=--release
endif

.PHONY: build
build:
	docker-compose build

.PHONY: shell
shell:
	docker-compose run --rm app \
		/bin/bash

.PHONY: build-release
build-release:
	docker-compose run --rm \
		--env LIBGIT2_NO_VENDOR=1 \
		app \
		cargo build --release

.PHONY: tests
tests:
	docker-compose run --rm app \
		cargo test --tests $(TEST_FLAGS)
