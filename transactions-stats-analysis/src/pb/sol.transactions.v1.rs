// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instructions {
    #[prost(message, repeated, tag="1")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(string, tag="1")]
    pub program_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
}
// @@protoc_insertion_point(module)
