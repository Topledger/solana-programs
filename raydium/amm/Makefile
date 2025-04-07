# Default Solana endpoint (can be overridden)
ENDPOINT ?= mainnet.sol.streamingfast.io:443

# Build the Rust WASM module for Substreams
.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release

# Package the Substream into an .spkg file
.PHONY: package
package: build
	substreams pack ./substreams.yaml

# Run the Substream (example: start streaming from a specific block)
# Adjust block number (-s) as needed
.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml raydium_amm_instructions -s 313745267 -t +1 > amm_pools.jsonl

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean
	rm -rf src/pb/*