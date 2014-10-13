# usage:
# make clean
# make

all: run

clean:
	-rm -f *~ bench

run: benchmark

bench: bench.rs
	rustc --test -O bench.rs

benchmark: bench
	./bench --bench
