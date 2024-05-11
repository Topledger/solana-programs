// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMeta {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub parent_hash: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
