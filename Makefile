.PHONY: shell
shell:
	docker-compose run --rm \
		app \
		/bin/sh
