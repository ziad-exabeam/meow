all: build build_test docs build_testloop

output_dir: 
	@[ -d ./bin ] || mkdir ./bin

build: output_dir
	@[ -d ./bin ] || mkdir ./bin
	rustc -o ./bin/main --bin --opt-level=3 src/demo.rs

docs: output_dir
	pandoc --standalone -t slidy slides/lightning-2013-12.md -o bin/lightning-2013-12.html
	cp -R ./slides/static ./bin

build_test: output_dir
	rustc -o ./bin/main_test --test src/demo.rs --allow dead_code --opt-level 0

run:
	./bin/main

run_test:
	./bin/main_test --test

run_testbench:
	./bin/main_test --test --bench

build_testloop:
	rustc -o ./bin/testloop-rs --bin --opt-level=0 src/testloop-rs.rs

run_loop:
	./bin/testloop-rs ./src/demo.rs --test

run_loop_bench:
	./bin/testloop-rs ./src/demo.rs --test --bench

clean:
	@rm -rf ./bin

help:
	@echo "Usage: make [build | build_test | docs | clean]"
	@echo "       make [run | run_test | run_testbench]"
	@echo "       make [build_testloop]"
	@echo "       make [run_loop]"
	@echo "       make [help]"
