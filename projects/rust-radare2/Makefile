all:: lib example

CFLAGS = -g

lib:
	@echo "Building basic lib..."
	@mkdir -p build
	@rustc --out-dir build r2extractor/mod.rs $(CFLAGS)
	@echo "done."

test:
	@echo "Building libr2extractor tests..."
	@mkdir -p build
	@rustc --out-dir build --test r2extractor/mod.rs
	@echo "done."
	@echo -n "Running..."
	@build/r2extractor
	@echo "done."

example:
	@echo "Building main..."
	@rustc -L build main.rs $(CFLAGS)
	@echo "done."

clean:
	@echo -n "Cleaning..."
	@rm -rf main main.exe build
	@echo "done."

.PHONY: lib example all clean
