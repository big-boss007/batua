.PHONY: check test run fmt generate-types dev stop

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

dev:
	@echo "Starting Batua..."
	@cargo run &
	@cd frontend && npm run dev -- --port 5174 &
	@echo ""
	@echo "  Backend:  http://localhost:3000"
	@echo "  Frontend: http://localhost:5174"
	@echo "  Admin:    http://localhost:5174/admin"
	@echo ""
	@wait

stop:
	@pkill -f "target/debug/batua" 2>/dev/null || true
	@pkill -f "vite dev" 2>/dev/null || true
	@echo "Stopped all services"
