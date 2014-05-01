OUT=build

.PHONY: clean examples

all: clean compile

clean:
	rm -rf $(OUT)

compile: out-dir
	rustc --out-dir $(OUT) src/lib.rs

examples:
	rustc -o examples/examples -L build examples/examples.rs

run: examples
	examples/examples

out-dir:
	mkdir -p $(OUT)
