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
	substreams run -e $(ENDPOINT) substreams.yaml raydium_clmm_instructions -s 331627783 -t +1

# Clean build artifacts
.PHONY: clean
clean:
	cargo clean

# (Optional) Run verification binary if/when created
#.PHONY: verify
#verify:
#	cargo run --bin verify_discriminators --target=aarch64-apple-darwin -- verify 