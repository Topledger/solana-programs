# Substreams sink files

## Description

`substreams-sink-files` is a tool that allows developers to pipe data extracted from a blockchain into various types of local or Cloud files-based persistence solutions.

## Prerequisites

- A Substreams module prepared for a files-sink
- Cloud-based file storage mechanism (optional)

## Installation

Install `substreams-sink-files` by using the pre-built binary release [available in the official GitHub repository](https://github.com/streamingfast/substreams-sink-files/releases).

Extract `substreams-sink-files` into a folder and ensure this folder is referenced globally via your `PATH` environment variable.

## Using the `substreams-sink-files` tool

The `run` command is the primary way to work with the `substreams-sink-files` tool. The command for your project will resemble the following:

```bash
substreams-sink-files run \
    mainnet.eth.streamingfast.io:443 \
    https://github.com/streamingfast/substreams-eth-token-transfers/releases/download/v0.4.0/substreams-eth-token-transfers-v0.4.0.spkg \
    jsonl_out \
    ./localdata/out \
    --encoder=lines \
    --file-working-dir="./localdata/working" \
    --state-store=./localdata/working/state.yaml \
    10_000_000:+100_000
```

> **Note** We use a custom range here `10_000_000:+100_000` because there is no ERC20/ERC721/ERC1155 until a long time in the chain.

Output resembling the following will be printed to the terminal window for properly issued commands and a properly set up and configured Substreams module.

```bash
2023-06-16T11:23:52.342-0400 INFO (substreams-sink-files) starting prometheus metrics server {"listen_addr": "localhost:9102"}
2023-06-16T11:23:52.343-0400 INFO (substreams-sink-files) sink to files {"file_output_path": "./localdata/out", "file_working_dir": "./localdata/working", "encoder_type": "lines", "state_store": "./localdata/working/state.yaml", "blocks_per_file": 10000, "buffer_max_size": 67108864}
2023-06-16T11:23:52.343-0400 INFO (substreams-sink-files) sinker from CLI {"endpoint": "mainnet.eth.streamingfast.io:443", "manifest_path": "https://github.com/streamingfast/substreams-eth-token-transfers/releases/download/v0.4.0/substreams-eth-token-transfers-v0.4.0.spkg", "output_module_name": "jsonl_out", "expected_module_type": "<Ignored>", "block_range": "10_000_000:+100_000"}
2023-06-16T11:23:52.343-0400 INFO (substreams-sink-files) reading substreams manifest {"manifest_path": "https://github.com/streamingfast/substreams-eth-token-transfers/releases/download/v0.4.0/substreams-eth-token-transfers-v0.4.0.spkg"}
2023-06-16T11:23:52.343-0400 INFO (substreams-sink-files) starting pprof server {"listen_addr": "localhost:6060"}
2023-06-16T11:23:52.660-0400 INFO (substreams-sink-files) validating output module {"module_name": "jsonl_out"}
2023-06-16T11:23:52.660-0400 INFO (substreams-sink-files) validating output module type {"module_name": "jsonl_out", "module_type": "proto:substreams.sink.files.v1.Lines"}
2023-06-16T11:23:52.663-0400 INFO (substreams-sink-files) sinker configured {"mode": "Production", "module_count": 3, "output_module_name": "jsonl_out", "output_module_type": "proto:substreams.sink.files.v1.Lines", "output_module_hash": "0d94c2c7662fbe04923c43d9f8732e0858f7af37", "client_config": "mainnet.eth.streamingfast.io:443 (insecure: false, plaintext: false, JWT present: true)", "buffer": true, "block_range": "[10000000, 10100000)", "infinite_retry": false, "final_blocks_only": false, "liveness_checker": true}
2023-06-16T11:23:52.666-0400 INFO (substreams-sink-files) ready, waiting for signal to quit
2023-06-16T11:23:52.667-0400 INFO (substreams-sink-files) starting new file boundary {"boundary": "[10000000, 10010000)"}
2023-06-16T11:23:52.684-0400 INFO (substreams-sink-files) boundary started {"boundary": "[10000000, 10010000)"}
2023-06-16T11:23:52.684-0400 INFO (substreams-sink-files) starting file sink {"restarting_at": "#10009999 (601b697795b7435dcb3f661aeea877fae4e3b534044a5940497b0a04a8845621)"}
2023-06-16T11:23:52.684-0400 INFO (substreams-sink-files) starting sinker {"stats_refresh_each": "15s", "restarting_at": "#10009999 (601b697795b7435dcb3f661aeea877fae4e3b534044a5940497b0a04a8845621)", "end_at": "#1374390772024"}
2023-06-16T11:23:52.801-0400 INFO (substreams-sink-files) session initialized with remote endpoint {"trace_id": "8fd18de5b6acb648867f4b2828bee602"}
2023-06-16T11:23:53.054-0400 INFO (substreams-sink-files) block_num is not in active boundary {"active_boundary": "[10000000, 10010000)", "boundaries_to_skip": 0, "block_num": 10010000}
2023-06-16T11:23:53.054-0400 INFO (substreams-sink-files) stopping file boundary
2023-06-16T11:23:53.054-0400 INFO (substreams-sink-files) all data from range is in memory, no need to flush
2023-06-16T11:23:53.055-0400 INFO (substreams-sink-files) queuing boundary upload {"boundary": "[10000000, 10010000)"}
2023-06-16T11:23:53.056-0400 INFO (substreams-sink-files) bundler stats {"file_count": 1, "boundary": "[10000000, 10010000)", "boundary_process_duration": "371.596208ms", "upload_duration": "0s", "data_process_duration": "0s", "avg_upload_dur": 0, "total_upload_dur": 0, "avg_boundary_process_dur": 0.371596208, "total_boundary_process_dur": 0.371596208, "avg_data_process_dur": 0, "total_data_process_dur": 0}
2023-06-16T11:23:53.056-0400 INFO (substreams-sink-files) starting new file boundary {"boundary": "[10010000, 10020000)"}
2023-06-16T11:23:53.058-0400 INFO (substreams-sink-files) boundary uploaded {"boundary": "[10000000, 10010000)", "output_path": "localdata/out/0010000000-0010010000.jsonl"}
2023-06-16T11:23:53.060-0400 INFO (substreams-sink-files) boundary started {"boundary": "[10010000, 10020000)"}
```

### Cursors

When you use Substreams, it sends back a block to a consumer using an opaque cursor. This cursor points to the exact location within the blockchain where the block is. In case your connection terminates or the process restarts, upon re-connection, Substreams sends back the cursor of the last written bundle in the request so that the stream of data can be resumed exactly where it left off and data integrity is maintained.

You will find that the cursor is saved in a file on disk. The location of this file is specified by the flag `--state-store` which points to a local folder. You must ensure that this file is properly saved to a persistent location. If the file is lost, the `substreams-sink-files` tool will restart from the beginning of the chain, redoing all the previous processing.

Therefore, It is crucial that this file is properly persisted and follows your deployment of `substreams-sink-files` to avoid any data loss.

### High Performance

If you are looking for the fastest performance possible, we suggest that your destination source is able to handle heavy traffic. Also, to speed up things, you can allocate a lot of RAM to the process and increase the flag `--buffer-max-size` to a point where you are able to hold a full batch of N blocks in memory (checking the size of the final file is a good indicator of the size to keep stuff in memory).

A lot of I/O operations is avoid if the buffer can hold everything in memory greatly speeding up the process of writing blocks bundle to its final destination.

### Cloud-based storage

You can use the `substreams-sink-files` tool to route data to files on your local file system and cloud-based storage solutions. To use a cloud-based solution such as Google Cloud Storage bucket, S3 compatible bucket, or Azure bucket, you need to make sure it is set up properly. Then, instead of referencing a local file in the `substreams-sink-files run` command, use the path to the bucket. The paths resemble `gs://<bucket>/<path>`, `s3://<bucket>/<path>`, and `az://<bucket>/<path>` respectively. Be sure to update the values according to your account and provider.

### Limitations

When you use the `substreams-sink-files` tool, you will find that it syncs up to the most recent "final" block of the chain. This means it is not real-time. Additionally, the tool writes bundles to disk when it has seen 10,000 blocks. As a result, the latency of the last available bundle can be delayed by around 10,000 blocks. How many blocks per batch can be controlled by changing the flag `--file-block-count`

## Contributing

For additional information, [refer to the general StreamingFast contribution guide](https://github.com/streamingfast/streamingfast/blob/master/CONTRIBUTING.md).

## License

The `substreams-sink-files` tool [uses the Apache 2.0 license](https://github.com/streamingfast/substreams/blob/develop/LICENSE/README.md).
