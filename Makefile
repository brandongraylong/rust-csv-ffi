# Build
all: setup clean build copy
setup:
	mkdir -p includes
	mkdir -p lib
clean:
	rm -f target/release/libcsv_ffi.a
build:
	cargo build --release
copy: target/release/libcsv_ffi.a
	cp target/release/libcsv_ffi.a ./lib/

# # Build and run C
# example_build_c:
# 	mkdir -p bin/examples/C
# 	gcc -Iincludes/ examples/C/main.c lib/libcsv_ffi.a -o bin/examples/C/a.out
# example_run_c:
# 	./bin/examples/C/a.out
# example_valgrind_c:
# 	valgrind ./bin/examples/C/a.out

# Build, run, and test C++
example_build_cpp:
	mkdir -p bin/examples/cpp
	gcc -Iincludes/ examples/cpp/main.cpp -lstdc++ lib/libcsv_ffi.a -o bin/examples/cpp/a.out
example_run_cpp:
	./bin/examples/cpp/a.out
example_valgrind_cpp:
	valgrind --trace-children=yes --leak-check=full --show-reachable=yes ./bin/examples/cpp/a.out
