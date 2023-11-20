// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveUseAuthorityArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub number_of_uses: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="2")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatorLayout {
    #[prost(string, required, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, required, tag="2")]
    pub verified: bool,
    #[prost(uint32, required, tag="3")]
    pub share: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenStandardLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionLayout {
    #[prost(bool, required, tag="1")]
    pub verified: bool,
    #[prost(string, required, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseMethodLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsesLayout {
    #[prost(message, required, tag="1")]
    pub use_method: UseMethodLayout,
    #[prost(uint64, required, tag="2")]
    pub remaining: u64,
    #[prost(uint64, required, tag="3")]
    pub total: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDetailsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="2")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetDataLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag="5")]
    pub creators: ::prost::alloc::vec::Vec<CreatorLayout>,
    #[prost(bool, required, tag="6")]
    pub primary_sale_happened: bool,
    #[prost(bool, required, tag="7")]
    pub is_mutable: bool,
    #[prost(message, required, tag="8")]
    pub token_standard: TokenStandardLayout,
    #[prost(message, optional, tag="9")]
    pub collection: ::core::option::Option<CollectionLayout>,
    #[prost(message, optional, tag="10")]
    pub uses: ::core::option::Option<UsesLayout>,
    #[prost(message, optional, tag="11")]
    pub collection_details: ::core::option::Option<CollectionDetailsLayout>,
    #[prost(string, optional, tag="12")]
    pub rule_set: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrintSupplyLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="2")]
    pub val: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, required, tag="2")]
    pub asset_data: AssetDataLayout,
    #[prost(uint32, optional, tag="3")]
    pub decimals: ::core::option::Option<u32>,
    #[prost(message, optional, tag="4")]
    pub print_supply: ::core::option::Option<PrintSupplyLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMasterEditionArgsLayout {
    #[prost(uint64, optional, tag="1")]
    pub max_supply: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag="5")]
    pub creators: ::prost::alloc::vec::Vec<CreatorLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataV2Layout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag="5")]
    pub creators: ::prost::alloc::vec::Vec<CreatorLayout>,
    #[prost(message, optional, tag="6")]
    pub collection: ::core::option::Option<CollectionLayout>,
    #[prost(message, optional, tag="7")]
    pub uses: ::core::option::Option<UsesLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataAccountArgsLayout {
    #[prost(message, required, tag="1")]
    pub data: DataLayout,
    #[prost(bool, required, tag="2")]
    pub is_mutable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataAccountArgsV2Layout {
    #[prost(message, required, tag="1")]
    pub data: DataV2Layout,
    #[prost(bool, required, tag="2")]
    pub is_mutable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataAccountArgsV3Layout {
    #[prost(message, required, tag="1")]
    pub data: DataV2Layout,
    #[prost(bool, required, tag="2")]
    pub is_mutable: bool,
    #[prost(message, optional, tag="3")]
    pub collection_details: ::core::option::Option<CollectionDetailsLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeedsVecLayout {
    #[prost(uint32, repeated, packed="false", tag="1")]
    pub seeds: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafInfoLayout {
    #[prost(uint32, repeated, packed="false", tag="1")]
    pub leaf: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadTypeLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub val_pub_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub val_seeds_vec: ::core::option::Option<SeedsVecLayout>,
    #[prost(message, optional, tag="4")]
    pub val_leaf_info: ::core::option::Option<LeafInfoLayout>,
    #[prost(uint64, optional, tag="5")]
    pub val_int64: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapItemLayout {
    #[prost(string, required, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, required, tag="2")]
    pub val: PayloadTypeLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadLayout {
    #[prost(message, repeated, tag="1")]
    pub map: ::prost::alloc::vec::Vec<MapItemLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationDataLayout {
    #[prost(message, required, tag="1")]
    pub payload: PayloadLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegateArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
    #[prost(uint64, optional, tag="3")]
    pub amount: ::core::option::Option<u64>,
    #[prost(string, optional, tag="4")]
    pub locked_address: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationTypeLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub migration_type: ::core::option::Option<MigrationTypeLayout>,
    #[prost(string, optional, tag="3")]
    pub rule_set: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="2")]
    pub amount: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintNewEditionFromMasterEditionViaTokenArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub edition: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintPrintingTokensViaTokenArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub supply: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCollectionSizeArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationLayout {
    #[prost(string, required, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="2")]
    pub spots_remaining: u64,
    #[prost(uint64, required, tag="3")]
    pub total_spots: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetReservationListArgsLayout {
    #[prost(message, repeated, tag="1")]
    pub reservations: ::prost::alloc::vec::Vec<ReservationLayout>,
    #[prost(uint64, optional, tag="2")]
    pub total_reservation_spots: ::core::option::Option<u64>,
    #[prost(uint64, required, tag="3")]
    pub offset: u64,
    #[prost(uint64, required, tag="4")]
    pub total_spot_offset: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="2")]
    pub amount: ::core::option::Option<u64>,
    #[prost(message, optional, tag="3")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOutOfEscrowArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionToggleLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub val: ::core::option::Option<CollectionLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDetailsToggleLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub val: ::core::option::Option<CollectionDetailsLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsesToggleLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub val: ::core::option::Option<UsesLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleSetToggleLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub new_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<DataLayout>,
    #[prost(bool, optional, tag="4")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="5")]
    pub is_mutable: ::core::option::Option<bool>,
    #[prost(message, optional, tag="6")]
    pub collection: ::core::option::Option<CollectionToggleLayout>,
    #[prost(message, optional, tag="7")]
    pub collection_details: ::core::option::Option<CollectionDetailsToggleLayout>,
    #[prost(message, optional, tag="8")]
    pub uses: ::core::option::Option<UsesToggleLayout>,
    #[prost(message, optional, tag="9")]
    pub rule_set: ::core::option::Option<RuleSetToggleLayout>,
    #[prost(message, optional, tag="10")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataAccountArgsLayout {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<DataLayout>,
    #[prost(string, optional, tag="2")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataAccountArgsV2Layout {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<DataV2Layout>,
    #[prost(string, optional, tag="2")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub is_mutable: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtilizeArgsLayout {
    #[prost(uint64, required, tag="1")]
    pub number_of_uses: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerificationArgsLayout {
    #[prost(string, required, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Arg {
    #[prost(message, optional, tag="1")]
    pub approve_use_authority_args: ::core::option::Option<ApproveUseAuthorityArgsLayout>,
    #[prost(message, optional, tag="2")]
    pub burn_args: ::core::option::Option<BurnArgsLayout>,
    #[prost(message, optional, tag="3")]
    pub create_args: ::core::option::Option<CreateArgsLayout>,
    #[prost(message, optional, tag="4")]
    pub create_master_edition_args: ::core::option::Option<CreateMasterEditionArgsLayout>,
    #[prost(message, optional, tag="5")]
    pub create_metadata_account_args: ::core::option::Option<CreateMetadataAccountArgsLayout>,
    #[prost(message, optional, tag="6")]
    pub create_metadata_account_args_v2: ::core::option::Option<CreateMetadataAccountArgsV2Layout>,
    #[prost(message, optional, tag="7")]
    pub create_metadata_account_args_v3: ::core::option::Option<CreateMetadataAccountArgsV3Layout>,
    #[prost(message, optional, tag="8")]
    pub delegate_args: ::core::option::Option<DelegateArgsLayout>,
    #[prost(message, optional, tag="9")]
    pub lock_args: ::core::option::Option<LockArgsLayout>,
    #[prost(message, optional, tag="10")]
    pub migrate_args: ::core::option::Option<MigrateArgsLayout>,
    #[prost(message, optional, tag="11")]
    pub mint_args: ::core::option::Option<MintArgsLayout>,
    #[prost(message, optional, tag="12")]
    pub mint_new_edition_from_master_edition_via_token_args: ::core::option::Option<MintNewEditionFromMasterEditionViaTokenArgsLayout>,
    #[prost(message, optional, tag="13")]
    pub mint_printing_tokens_via_token_args: ::core::option::Option<MintPrintingTokensViaTokenArgsLayout>,
    #[prost(message, optional, tag="14")]
    pub revoke_args: ::core::option::Option<RevokeArgsLayout>,
    #[prost(message, optional, tag="15")]
    pub set_collection_size_args: ::core::option::Option<SetCollectionSizeArgsLayout>,
    #[prost(message, optional, tag="16")]
    pub set_reservation_list_args: ::core::option::Option<SetReservationListArgsLayout>,
    #[prost(message, optional, tag="17")]
    pub transfer_args: ::core::option::Option<TransferArgsLayout>,
    #[prost(message, optional, tag="18")]
    pub transfer_out_of_escrow_args: ::core::option::Option<TransferOutOfEscrowArgsLayout>,
    #[prost(message, optional, tag="19")]
    pub unlock_args: ::core::option::Option<UnlockArgsLayout>,
    #[prost(message, optional, tag="20")]
    pub update_args: ::core::option::Option<UpdateArgsLayout>,
    #[prost(message, optional, tag="21")]
    pub update_metadata_account_args: ::core::option::Option<UpdateMetadataAccountArgsLayout>,
    #[prost(message, optional, tag="22")]
    pub update_metadata_account_args_v2: ::core::option::Option<UpdateMetadataAccountArgsV2Layout>,
    #[prost(message, optional, tag="23")]
    pub use_args: ::core::option::Option<UseArgsLayout>,
    #[prost(message, optional, tag="24")]
    pub utilize_args: ::core::option::Option<UtilizeArgsLayout>,
    #[prost(message, optional, tag="25")]
    pub verification_args: ::core::option::Option<VerificationArgsLayout>,
    #[prost(string, required, tag="26")]
    pub instruction_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetadataMeta {
    #[prost(string, required, tag="1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag="2")]
    pub block_time: i64,
    #[prost(string, required, tag="3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="5")]
    pub block_slot: u64,
    #[prost(uint32, required, tag="7")]
    pub instruction_index: u32,
    #[prost(bool, required, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="9")]
    pub inner_instruction_index: u32,
    #[prost(message, required, tag="10")]
    pub arg: Arg,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<TokenMetadataMeta>,
}
// @@protoc_insertion_point(module)
