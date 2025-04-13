# Default Solana endpoint (can be overridden)
ENDPOINT ?= mainnet.sol.streamingfast.io:443

# Protobuf generation
.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

# Build the Rust WASM module for Substreams
.PHONY: build
build: protogen # Ensure protos are generated before building
	cargo build --target wasm32-unknown-unknown --release

# Package the Substream into an .spkg file
.PHONY: package
package: build
	substreams pack ./substreams.yaml

# Run the Substream (example: start streaming from a specific block)
# Adjust block number (-s) as needed
.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml dex_fee_tier_map -s 309407516 -t +1

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean 