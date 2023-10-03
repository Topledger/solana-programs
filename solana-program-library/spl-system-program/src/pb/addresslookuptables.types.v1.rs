// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressLookupTables {
    #[prost(message, repeated, tag="1")]
    pub address_lookup_tables: ::prost::alloc::vec::Vec<AddressLookupTable>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressLookupTable {
    #[prost(string, tag="1")]
    pub table_address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// @@protoc_insertion_point(module)
