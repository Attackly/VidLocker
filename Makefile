# Makefile

.PHONY: build_frontend build_rust build_all

run_frontend:
	cd frontend && npm run dev

build_frontend:
	cd frontend && npm run build -- --emptyOutDir
	rm backend/dist -rf
	mkdir backend/dist -p
	cp frontend/build/* backend/dist -r

# Build Rust backend
build_rust:
	cd backend && cargo build --release

build_all: build_frontend build_rust
	echo "Build complete!"

run_dev: build_frontend
	cd backend && cargo sqlx prepare && cargo run

check:
	cd backend && cargo check
