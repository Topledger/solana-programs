// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMeta {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub number: u64,
    #[prost(string, tag="3")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub timestamp: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
