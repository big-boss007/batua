.PHONY: check test run fmt generate-types

check:
	cargo check

test:
	cargo test

run:
	cargo run

fmt:
	cargo fmt

generate-types:
	@echo "Type generation placeholder"

seed:
	@bash scripts/seed.sh

reset-db:
	@psql -U chirag -d postgres -c "DROP DATABASE IF EXISTS batua;"
	@psql -U chirag -d postgres -c "CREATE DATABASE batua;"
	@for f in $$(ls migrations/*.sql | sort); do psql -U chirag -d batua -f "$$f" > /dev/null 2>&1; done
	@echo "Database reset complete"

reset-and-seed: reset-db seed
