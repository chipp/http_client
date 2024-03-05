patch: VERSION=$(shell cat Cargo.toml | grep "version = " | sed -e 's,version = "\(.*\..*\..*\)",\1,' | tr -d '\n')
patch: MAJOR_VERSION=$(shell printf $(VERSION) | sed -e 's,\(.*\)\.\(.*\)\.\(.*\),\1,')
patch: MINOR_VERSION=$(shell printf $(VERSION) | sed -e 's,\(.*\)\.\(.*\)\.\(.*\),\2,')
patch: PATCH_VERSION=$(shell printf $(VERSION) | sed -e 's,\(.*\)\.\(.*\)\.\(.*\),\3,')
patch: NEXT_VERSION=$(shell printf $(MAJOR_VERSION).$(MINOR_VERSION).$$(( $(PATCH_VERSION) + 1 )))
patch:
ifneq ($(shell git diff --stat --staged | wc -c | tr -d ' ' | tr -d '\n'), 0)
	$(error Some changes are staged. Stash changes before patching version)
endif

ifneq ($(shell git diff --stat Cargo.toml | wc -c | tr -d ' ' | tr -d '\n'), 0)
	$(error Stash changes in Cargo.toml before patching version)
endif

	cargo test

	@sed -i '' 's,^version = "$(VERSION)",version = "$(NEXT_VERSION)",' Cargo.toml

	@git add Cargo.toml
	@git commit -m "Bump version to $(NEXT_VERSION)"
	@git tag $(NEXT_VERSION)
	@git push origin main
	@git push origin $(NEXT_VERSION)
