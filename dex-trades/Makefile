ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: stream_old
stream_old: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block_before_lookup -s 190599999 -t +10000

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s 67492336 -t +10000

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml
