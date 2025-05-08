ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml map_block -s 231333119 -t +1

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml

.PHONY wasm_pack:
wasm_pack:
	wasm-pack build --no-typescript --target no-modules

.PHONY convert_and_gcloud_upload:
convert_and_gcloud_upload:
	go build utils/convert.go && mv convert utils/convert
	utils/script.sh metaplex_token_metadata