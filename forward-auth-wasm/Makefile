prog :=traefik-auth

build: 
	make release_cleanup
	cargo build --target wasm32-unknown-unknown
	cp .traefik.yml pkg/
	cp target/wasm32-unknown-unknown/debug/forward_auth_plugin.wasm pkg/$(prog).wasm

release:
	make release-cleanup
	cargo build --target wasm32-unknown-unknown --release
	cp .traefik.yml pkg/
	cp target/wasm32-unknown-unknown/release/forward_auth_plugin.wasm pkg/$(prog).wasm

release_cleanup:
	rm -rf pkg/
	mkdir pkg

clean:
	rm -rf pkg/
	rm -rf target/

docker: 
	make build
	docker compose up 

setup:
	rustup target add wasm32-unknown-unknown
	cargo check --target wasm32-unknown-unknown --no-default-features

http:
	curl -sS -D - 127.0.0.1/foo -o /dev/null

debug:
	make build
	wasm2wat pkg/traefik-auth.wasm | less


cargo:
	cargo publish --target wasm32-unknown-unknown --dry-run --allow-dirty