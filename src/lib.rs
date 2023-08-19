mod pb;

use pb::acme::block_meta::v1::BlockMeta;
use substreams::Hex;
use substreams_ethereum::pb::eth;

#[substreams::handlers::map]
fn map_block(block: eth::v2::Block) -> Result<BlockMeta, substreams::errors::Error> {
    let header = block.header.as_ref().unwrap();

    Ok(BlockMeta {
        number: block.number,
        hash: Hex(&block.hash).to_string(),
        parent_hash: Hex(&header.parent_hash).to_string(),
        timestamp: header.timestamp.as_ref().unwrap().to_string(),
    })
}