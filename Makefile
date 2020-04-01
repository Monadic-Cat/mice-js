source_files ::= $(wildcard src/*.rs)
package_files ::= pkg/mice_js.js pkg/mice_js_bg.wasm pkg/mice_js.d.ts pkg/mice_js_bg.d.ts pkg/package.json

$(package_files): Cargo.toml $(source_files)
	wasm-pack build --target nodejs
	python3 package_json_rewrite.py

.PHONY:
publish: $(package_files)
	wasm-pack publish --access=public

.PHONY:
clean:
	rm -r pkg
