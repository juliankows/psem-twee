dist/index.html: node_modules $(shell find src -type f)
	yarn build

build: dist/index.html
.PHONY: build

node_modules: package.json
	yarn

