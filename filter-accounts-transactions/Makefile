ENDPOINT ?= mainnet.sol.streamingfast.io:443

.PHONY: build
build:
	LDFLAGS="-Wl,-no_compact_unwind" cargo build --target wasm32-unknown-unknown --release

.PHONY: stream
stream: build
	substreams run -e $(ENDPOINT) substreams.yaml jsonl_out -p jsonl_out="addresses[]=CMFEEAmEnn1UZ9PFeNV4t49ffir2qnu8y6wFb6YZnevq&addresses[]=FPSFEEeDdXJfrowCM4kJdnUyxdXmHFa7Xvmz63BkjetS&addresses[]=FeesFishZGMrvUMZbUzRzuNwcYNKxNYrLFuMzvJeBMFm&addresses[]=FeesFSmtpqXJcaxGL9Ws58ixoqDtAckvF4CeUMxhp8g6&addresses[]=AiRFEEwT37TH8SAick3JcVU7mkJiy4sdvySWCCiFVXsP&addresses[]=BURNbcXSyfSXMWBvtqSxsBrP6FMKzXJvrgpatNJpC8c2&addresses[]=FEejsRXqs5HUpeeUdK285dmhJKA6aZsiywn5Gag4gVhC" -s 180830047 -t +1

.PHONY: protogen
protogen:
	substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"

.PHONY: package
package:
	substreams pack ./substreams.yaml