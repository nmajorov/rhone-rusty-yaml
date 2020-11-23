# Needed SHELL since I'm using zsh
SHELL := /bin/bash

ts := $(shell date -u +"%Y-%m-%dT%H:%M:%SZ")

version := 1.0.3

.PHONY: help
help: ## This help message
	@echo -e "$$(grep -hE '^\S+:.*##' $(MAKEFILE_LIST) | sed -e 's/:.*##\s*/:/' -e 's/^\(.\+\):\(.*\)/\\x1b[36m\1\\x1b[m:\2/' | column -c2 -t -s :)"

.PHONY: build
build:  dev-packages ## Builds Rust code and Python modules
	python setup.py bdist_wheel

.PHONY: build-release
build-release: build ## Build  module in release mode
	maturin build -o dist

#.PHONY: nightly
#nightly: ## Set rust compiler to nightly version
#	rustup override set nightly

.PHONY: install
install: dev-packages ## Install  module into current virtualenv
	#poetry run maturin develop --release

.PHONY: publish
publish: ## Publish wheel on my server
	scp ./dist/*.whl nmajorov@majorov.biz:/home/nmajorov/dwn

.PHONY: clean
clean: ## Clean up build artifacts
	python setup.py clean
	$(shell if [ -d "./dist" ] ; then rm -r ./dist; fi)

.PHONY: dev-packages
dev-packages: ## Install Python development packages for project
	python setup.py install

.PHONY: cargo-test
cargo-test: ## Run cargo tests only
	cargo test -- --nocapture

.PHONY: test
test: cargo-test install pytest ## Intall  module and run tests

.PHONY: pytest
pytest: ## Run python tests on already installed  module
	python setup.py test


.PHONY: bench
bench: ## Run benchmarks
	pytest benchmarks
