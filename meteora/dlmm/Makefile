.PHONY: all
all: build

.PHONY: build
build: protogen
	cargo build --target wasm32-unknown-unknown --release

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package: build
	substreams pack ./substreams.yaml

.PHONY: stream
stream: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_block -s 335838627 -t +1 > meteora_dlmm_instructions.jsonl

.PHONY: stream_output_to_stdout
stream_output_to_stdout: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_block -s 240000000 -t +10 --output-mode=jsonl

.PHONY: test_stream
test_stream: build
	substreams run -e mainnet.sol.streamingfast.io:443 substreams.yaml map_block -s 338000000 -t +100 --output-mode=json

.PHONY: clean
clean:
	cargo clean
	rm -rf proto/sf
	rm -rf target 