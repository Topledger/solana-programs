// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<super::super::super::sf::solana::r#type::v1::ConfirmedTransaction>,
}
// @@protoc_insertion_point(module)
