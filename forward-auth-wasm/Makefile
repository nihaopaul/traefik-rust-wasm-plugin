prog :=traefik-auth

build: 
	wasm-pack build --no-pack
	rm pkg/*.js
	rm pkg/*.ts
	cp .traefik.yml pkg/
	mv pkg/forwardAuthPlugin_bg.wasm pkg/$(prog).wasm

release:
	wasm-pack build --release --no-pack
	rm pkg/*.js
	rm pkg/*.ts
	cp .traefik.yml pkg/
	mv pkg/forwardAuthPlugin_bg.wasm pkg/$(prog).wasm

clean:
	rm -rf pkg/
	rm -rf target/

run: build
	docker compose up | grep forwardAuth