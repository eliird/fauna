engine:
	cd engine && wasm-pack build --target web
web:
	cd web && npm run dev
publish:
	rm -rf web/src/engine
	cp -r engine/pkg web/src/engine

run: engine publish web

.PHONY: engine web publish run