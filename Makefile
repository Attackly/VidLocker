# Makefile

.PHONY: build_frontend build_rust build_all

build_frontend:
	cd frontend && npm run build --emptyOutDir

# Build Rust backend
build_rust:
	cd backend && cargo build --release

build_all: build_frontend build_rust
	echo "Build complete!"
