# StreamingFast Substreams Template
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Quick Start (Locally)

Use this quickstart guide to set up your environment to use Substreams locally.

First, [copy this repository](https://github.com/streamingfast/substreams-template/generate) and clone it.

## Quick Start (Gitpod)

Use these steps to conveniently open your repository in a Gitpod.

1. First, [copy this repository](https://github.com/streamingfast/substreams-template/generate)
2. Grab a StreamingFast key from [https://app.streamingfast.io/](https://app.streamingfast.io/)
3. Create a [Gitpod](https://gitpod.io) account
4. Configure a `STREAMINGFAST_KEY` variable in your Gitpod account settings
5. Open your repository as a [Gitpod workspace](https://gitpod.io/workspaces)

## Install Dependencies & Authentication

Follow [Installation Requirements](https://substreams.streamingfast.io/getting-started/installing-the-cli) instructions on official Substreams documentation website.

Also make sure that you grabbed your StreamingFast API key and generated a Substreams API token set to environment `SUBSTREAMS_API_TOKEN`, see [authentication instructions](https://substreams.streamingfast.io/getting-started/quickstart#run-your-first-substreams) for how to do it.

### Validation

Ensure that `substreams` CLI works as expected:

```
substreams -v
version (...)
```

## Generating Protobuf

```bash
substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

> We exclude paths that are not required to have locally.

## Compile

At this point, we're ready to build our WASM binary and Protobuf definitions.

```bash
cargo build --target wasm32-unknown-unknown --release
```

The resulting WASM artifact will be found at `./target/wasm32-unknown-unknown/release/substreams.wasm`

## Run your Substreams

We're now ready to run our example Substreams!

> Don't forget to be at the root of the project to run the following commands

```bash
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml db_out --start-block 12292922 --stop-block +1
```

Let's break down everything happening above.

- `substreams` is our executable
- `-e mainnet.eth.streamingfast.io:443` is the provider going to run our Substreams
- `substream.yaml` is the path where we have defined our Substreams Manifest
- `db_out` this is the module which we want to run, defined in the manifest (must be of `map` kind)
- `--start-block 12292922` start from block `12292922`
- `--stop-block +1` only request a single block (stop block will be manifest's start block + 1)

Here is the example of an output of the `map_transfers` starting at `12292922` block for only `1` block:

 > **Note** Using `[...]` to abbreviate the JSON output

```bash
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml db_out -s 12292922 -t +10
Connected (trace ID fb2646fe50f1cb5430b89ea273b6a6aa)
Progress messages received: 240 (29/sec)
Backprocessing history up to requested target block 12292922:
(hit 'm' to switch mode)

store_transfers            12287507  ::  12287507-12288544 12289000-12289548 12290000-12290542 12291000-12291452 12292000-12292481

# Output above will be different on your machine, what is happening is that we requested block
# 12292922 but the `substreams.yaml` start block is 12287507 which means we have 5 415 blocks to
# catch to build up 'store_transfers' state up to block 12292922. This is done on parallel worker
# and the output above is displaying the advanced of the backward parallel processing.
...

# Once store reconciliation is done, you will start to receive the output of `db_out` module:

----------- BLOCK #12,292,922 (e2d521d11856591b77506a383033cf85e1d46f1669321859154ab38643244293) ---------------
{
  "@module": "db_out",
  "@block": 12292922,
  "@type": "sf.substreams.sink.database.v1.DatabaseChanges",
  "@data": {
    "tableChanges": [
      {
        "table": "transfer",
        "pk": "cfb197f62ec5c7f0e71a11ec0c4a0e394a3aa41db5386e85526f86c84b3f2796-87",
        "operation": "OPERATION_CREATE",
        "fields": [
          {
            "name": "trx_hash",
            "newValue": "cfb197f62ec5c7f0e71a11ec0c4a0e394a3aa41db5386e85526f86c84b3f2796"
          },
          [...]
```

## Next Steps

Congratulations! You've successfully run a Substreams.

- Read the documentation at https://substreams.streamingfast.io.
- Look at [Playground](https://github.com/streamingfast/substreams-playground) for more learning examples.
