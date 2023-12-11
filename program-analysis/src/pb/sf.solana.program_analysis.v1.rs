// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<ProgramStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProgramStats {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub program_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="4")]
    pub fee_payers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, required, tag="5")]
    pub fee_lamports: u32,
    #[prost(uint32, required, tag="6")]
    pub base_fee_lamports: u32,
    #[prost(uint32, required, tag="7")]
    pub priority_fee_lamports: u32,
    #[prost(uint32, required, tag="8")]
    pub compute_units_consumed: u32,
    #[prost(uint32, required, tag="9")]
    pub compute_units_allocated: u32,
    #[prost(uint32, required, tag="10")]
    pub successful_txns_count: u32,
    #[prost(uint32, required, tag="11")]
    pub failed_txns_count: u32,
    #[prost(uint32, required, tag="12")]
    pub total_outer_invocation_count: u32,
    #[prost(uint32, required, tag="13")]
    pub total_inner_invocation_count: u32,
    #[prost(uint32, required, tag="14")]
    pub failed_invocation_count: u32,
    #[prost(map="string, uint32", tag="15")]
    pub errors: ::std::collections::HashMap<::prost::alloc::string::String, u32>,
}
// @@protoc_insertion_point(module)
