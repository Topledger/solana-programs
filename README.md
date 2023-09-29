# Solana Substreams

Substreams for popular dApps on Solana.
Every project is a seperate substream and can be used independently.

### Generate protos
```bash
make protogen
```

### Build substreams
```bash
make build
```

### Set up token
Visit https://substreams.streamingfast.io/reference-and-specs/authentication to fetch a token or run below command if you have already followed the instructions from the linked documentation page.
```bash
sftoken
```

### Run substreams
```bash
make stream
```