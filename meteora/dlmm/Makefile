.PHONY: all
all: build

.PHONY: build
build: 
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams pack ./substreams.yaml

.PHONY: stream
stream: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_block -s 331472828 -t +1

.PHONY: stream_output_to_stdout
stream_output_to_stdout: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_block -s 240000000 -t +10 --output-mode=jsonl

.PHONY: debug_build
debug_build: clean protogen
	RUSTFLAGS="--cfg debug_assertions" cargo build --target wasm32-unknown-unknown --release -v

.PHONY: clean
clean:
	cargo clean
	rm -rf proto/sf
	rm -rf target 