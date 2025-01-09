
build-transfer: 
	RUSTFLAGS="-C target-feature=+multivalue" cargo build --target wasm32-unknown-unknown --release --no-default-features --features transfer
build-view:
	RUSTFLAGS="-C target-feature=+multivalue" cargo build --target wasm32-unknown-unknown --release --no-default-features --features view

ci: build-transfer build-view
	@echo "build success"
