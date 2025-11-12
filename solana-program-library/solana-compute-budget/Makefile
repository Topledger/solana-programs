ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s 252842623 -t +1

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml


# 2024-04-10T20:36:00.696Z ERRO (cli) application shutdown unexpectedly, quitting: stream invalid: rpc error: code = InvalidArgument desc = rpc error: code = InvalidArgument desc = step new irr: handler step new: execute modules: applying executor results "map_block": execute: maps wasm call: block 252842623: module "map_block": general wasm execution panicked: wasm execution failed deterministically: panic in the wasm: "called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidInput, error: \"Unexpected length of input\" }" at src/instruction.rs:61:89