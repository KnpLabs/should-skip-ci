.PHONY: shell
shell:
	docker-compose run --rm app \
		/bin/bash

.PHONY: build-release
build-release:
	docker-compose run --rm app \
		cargo build --release
