TESTS := $(wildcard tests/*.sh)

build:
	cargo build
	@ln -sf target/debug/linker_rs ld

test: build
	$(MAKE) $(TESTS)
	@printf '\e[32mPassed all tests\n\e[0m'

$(TESTS):
	@echo 'TESTING' $@
	@./$@
	@printf '\e[32mOK\n\e[0m'

clean:
	cargo clean
	rm -rf out

.PHONY: build clean test $(TESTS)