.PHONY: build compile run

source=output.ll
full_source=./tests/${source}

compile:
	@clang ${full_source} -o ./tests/out

build:
	@cargo run

run: build compile
	@./tests/out
