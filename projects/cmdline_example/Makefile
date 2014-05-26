include rust.mk

# Default target
all: bin

# Crates
$(eval $(call RUST_CRATE,BIN,src/cmdline/bin.rs))

# Convenience targets
bin: $(BIN_OUT)

clean: $(BIN_CLEAN)

.PHONY: all bin clean
