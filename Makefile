rcc:
	cargo build
	cp ./target/debug/rcc .

test: rcc
	./test.sh

clean:
	rm -f rcc *.o tmp*

.PHONY: test clean
