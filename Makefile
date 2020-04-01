.PHONY:
build:
	wasm-pack build --target nodejs
	python3 package_json_rewrite.py

.PHONY:
publish: build
	wasm-pack publish --access=public

.PHONY:
clean:
	rm -r pkg
