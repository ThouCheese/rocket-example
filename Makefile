migrate:
	docker-compose exec app diesel migration run

revert:
	docker-compose exec app diesel migration revert

redo:
	docker-compose exec app diesel migration redo

bash:
	docker-compose exec app bash

psql:
	docker-compose exec database psql -U posts

test:
	docker-compose down && \
		docker-compose -f docker-compose.test.yml up \
		--abort-on-container-exit \
		--exit-code-from test_app && \
		docker-compose down --volumes --rmi local

doc:
	cargo doc --open --document-private-items
