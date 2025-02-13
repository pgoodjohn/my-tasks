start:
	npm run tauri dev

ios:
	npm run tauri ios dev

build:
	npm run tauri build

test:
	cd src-tauri && cargo test

test-watch:
	cd src-tauri && cargo watch -x test

lint-rust:
	cd src-tauri && cargo clippy