use serde::Serialize;

// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbApproveUseAuthorityArgsLayout {
    #[prost(string, required, tag = "1")]
    pub number_of_uses: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbBurnArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCreatorLayout {
    #[prost(string, required, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, required, tag = "2")]
    pub verified: bool,
    #[prost(uint32, required, tag = "3")]
    pub share: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCollectionLayout {
    #[prost(bool, required, tag = "1")]
    pub verified: bool,
    #[prost(string, required, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUsesLayout {
    #[prost(string, required, tag = "1")]
    pub use_method: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub remaining: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub total: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCollectionDetailsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub size: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub padding: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbAssetDataLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag = "5")]
    pub creators: ::prost::alloc::vec::Vec<PbCreatorLayout>,
    #[prost(bool, required, tag = "6")]
    pub primary_sale_happened: bool,
    #[prost(bool, required, tag = "7")]
    pub is_mutable: bool,
    #[prost(string, required, tag = "8")]
    pub token_standard: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub collection: ::core::option::Option<PbCollectionLayout>,
    #[prost(message, optional, tag = "10")]
    pub uses: ::core::option::Option<PbUsesLayout>,
    #[prost(message, optional, tag = "11")]
    pub collection_details: ::core::option::Option<PbCollectionDetailsLayout>,
    #[prost(string, optional, tag = "12")]
    pub rule_set: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbPrintSupplyLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCreateArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, required, tag = "2")]
    pub asset_data: PbAssetDataLayout,
    #[prost(uint32, optional, tag = "3")]
    pub decimals: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "4")]
    pub print_supply: ::core::option::Option<PbPrintSupplyLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCreateMasterEditionArgsLayout {
    #[prost(string, optional, tag = "1")]
    pub max_supply: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbDataLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag = "5")]
    pub creators: ::prost::alloc::vec::Vec<PbCreatorLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbDataV2Layout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag = "5")]
    pub creators: ::prost::alloc::vec::Vec<PbCreatorLayout>,
    #[prost(message, optional, tag = "6")]
    pub collection: ::core::option::Option<PbCollectionLayout>,
    #[prost(message, optional, tag = "7")]
    pub uses: ::core::option::Option<PbUsesLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCreateMetadataAccountArgsLayout {
    #[prost(message, required, tag = "1")]
    pub data: PbDataLayout,
    #[prost(bool, required, tag = "2")]
    pub is_mutable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCreateMetadataAccountArgsV2Layout {
    #[prost(message, required, tag = "1")]
    pub data: PbDataV2Layout,
    #[prost(bool, required, tag = "2")]
    pub is_mutable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCreateMetadataAccountArgsV3Layout {
    #[prost(message, required, tag = "1")]
    pub data: PbDataV2Layout,
    #[prost(bool, required, tag = "2")]
    pub is_mutable: bool,
    #[prost(message, optional, tag = "3")]
    pub collection_details: ::core::option::Option<PbCollectionDetailsLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbSeedsVecLayoutInner {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub values: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbSeedsVecLayout {
    #[prost(message, repeated, tag = "1")]
    pub seeds: ::prost::alloc::vec::Vec<PbSeedsVecLayoutInner>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbLeafInfoLayout {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub leaf: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbPayloadTypeLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub val_pub_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub val_seeds_vec: ::core::option::Option<PbSeedsVecLayout>,
    #[prost(message, optional, tag = "4")]
    pub val_leaf_info: ::core::option::Option<PbLeafInfoLayout>,
    #[prost(string, optional, tag = "5")]
    pub val_int64: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbMapItemLayout {
    #[prost(string, required, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, required, tag = "2")]
    pub val: PbPayloadTypeLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbPayloadLayout {
    #[prost(message, repeated, tag = "1")]
    pub map: ::prost::alloc::vec::Vec<PbMapItemLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbAuthorizationDataLayout {
    #[prost(message, required, tag = "1")]
    pub payload: PbPayloadLayout,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbDelegateArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
    #[prost(string, optional, tag = "3")]
    pub amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub locked_address: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbLockArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbMigrationTypeLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbMigrateArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub migration_type: ::core::option::Option<PbMigrationTypeLayout>,
    #[prost(string, optional, tag = "3")]
    pub rule_set: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbMintArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbMintNewEditionFromMasterEditionViaTokenArgsLayout {
    #[prost(string, required, tag = "1")]
    pub edition: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbMintPrintingTokensViaTokenArgsLayout {
    #[prost(string, required, tag = "1")]
    pub supply: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbRevokeArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbSetCollectionSizeArgsLayout {
    #[prost(string, required, tag = "1")]
    pub size: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbReservationLayout {
    #[prost(string, required, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub spots_remaining: ::prost::alloc::string::String,
    #[prost(string, required, tag = "3")]
    pub total_spots: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbSetReservationListArgsLayout {
    #[prost(message, repeated, tag = "1")]
    pub reservations: ::prost::alloc::vec::Vec<PbReservationLayout>,
    #[prost(string, optional, tag = "2")]
    pub total_reservation_spots: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, required, tag = "3")]
    pub offset: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub total_spot_offset: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbTransferArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbTransferOutOfEscrowArgsLayout {
    #[prost(string, required, tag = "1")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUnlockArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCollectionToggleLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub val: ::core::option::Option<PbCollectionLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbCollectionDetailsToggleLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub val: ::core::option::Option<PbCollectionDetailsLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUsesToggleLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub val: ::core::option::Option<PbUsesLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbRuleSetToggleLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub val: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUpdateArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub new_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<PbDataLayout>,
    #[prost(bool, optional, tag = "4")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub is_mutable: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "6")]
    pub collection: ::core::option::Option<PbCollectionToggleLayout>,
    #[prost(message, optional, tag = "7")]
    pub collection_details: ::core::option::Option<PbCollectionDetailsToggleLayout>,
    #[prost(message, optional, tag = "8")]
    pub uses: ::core::option::Option<PbUsesToggleLayout>,
    #[prost(message, optional, tag = "9")]
    pub rule_set: ::core::option::Option<PbRuleSetToggleLayout>,
    #[prost(message, optional, tag = "10")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
    #[prost(string, optional, tag = "11")]
    pub token_standard: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUpdateMetadataAccountArgsLayout {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<PbDataLayout>,
    #[prost(string, optional, tag = "2")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUpdateMetadataAccountArgsV2Layout {
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<PbDataV2Layout>,
    #[prost(string, optional, tag = "2")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "4")]
    pub is_mutable: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUseArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub authorization_data: ::core::option::Option<PbAuthorizationDataLayout>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbUtilizeArgsLayout {
    #[prost(string, required, tag = "1")]
    pub number_of_uses: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct PbVerificationArgsLayout {
    #[prost(string, required, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct Arg {
    #[prost(message, optional, tag = "1")]
    pub approve_use_authority_args: ::core::option::Option<PbApproveUseAuthorityArgsLayout>,
    #[prost(message, optional, tag = "2")]
    pub burn_args: ::core::option::Option<PbBurnArgsLayout>,
    #[prost(message, optional, tag = "3")]
    pub create_args: ::core::option::Option<PbCreateArgsLayout>,
    #[prost(message, optional, tag = "4")]
    pub create_master_edition_args: ::core::option::Option<PbCreateMasterEditionArgsLayout>,
    #[prost(message, optional, tag = "5")]
    pub create_metadata_account_args: ::core::option::Option<PbCreateMetadataAccountArgsLayout>,
    #[prost(message, optional, tag = "6")]
    pub create_metadata_account_args_v2:
        ::core::option::Option<PbCreateMetadataAccountArgsV2Layout>,
    #[prost(message, optional, tag = "7")]
    pub create_metadata_account_args_v3:
        ::core::option::Option<PbCreateMetadataAccountArgsV3Layout>,
    #[prost(message, optional, tag = "8")]
    pub delegate_args: ::core::option::Option<PbDelegateArgsLayout>,
    #[prost(message, optional, tag = "9")]
    pub lock_args: ::core::option::Option<PbLockArgsLayout>,
    #[prost(message, optional, tag = "10")]
    pub migrate_args: ::core::option::Option<PbMigrateArgsLayout>,
    #[prost(message, optional, tag = "11")]
    pub mint_args: ::core::option::Option<PbMintArgsLayout>,
    #[prost(message, optional, tag = "12")]
    pub mint_new_edition_from_master_edition_via_token_args:
        ::core::option::Option<PbMintNewEditionFromMasterEditionViaTokenArgsLayout>,
    #[prost(message, optional, tag = "13")]
    pub mint_printing_tokens_via_token_args:
        ::core::option::Option<PbMintPrintingTokensViaTokenArgsLayout>,
    #[prost(message, optional, tag = "14")]
    pub revoke_args: ::core::option::Option<PbRevokeArgsLayout>,
    #[prost(message, optional, tag = "15")]
    pub set_collection_size_args: ::core::option::Option<PbSetCollectionSizeArgsLayout>,
    #[prost(message, optional, tag = "16")]
    pub set_reservation_list_args: ::core::option::Option<PbSetReservationListArgsLayout>,
    #[prost(message, optional, tag = "17")]
    pub transfer_args: ::core::option::Option<PbTransferArgsLayout>,
    #[prost(message, optional, tag = "18")]
    pub transfer_out_of_escrow_args: ::core::option::Option<PbTransferOutOfEscrowArgsLayout>,
    #[prost(message, optional, tag = "19")]
    pub unlock_args: ::core::option::Option<PbUnlockArgsLayout>,
    #[prost(message, optional, tag = "20")]
    pub update_args: ::core::option::Option<PbUpdateArgsLayout>,
    #[prost(message, optional, tag = "21")]
    pub update_metadata_account_args: ::core::option::Option<PbUpdateMetadataAccountArgsLayout>,
    #[prost(message, optional, tag = "22")]
    pub update_metadata_account_args_v2:
        ::core::option::Option<PbUpdateMetadataAccountArgsV2Layout>,
    #[prost(message, optional, tag = "23")]
    pub use_args: ::core::option::Option<PbUseArgsLayout>,
    #[prost(message, optional, tag = "24")]
    pub utilize_args: ::core::option::Option<PbUtilizeArgsLayout>,
    #[prost(message, optional, tag = "25")]
    pub verification_args: ::core::option::Option<PbVerificationArgsLayout>,
    #[prost(string, required, tag = "26")]
    pub instruction_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct InputAccounts {
    #[prost(string, optional, tag = "1")]
    pub ata_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub attribute_dst: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub attribute_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub attribute_src: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub authorization_rules: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub authorization_rules_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub bubblegum_signer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub burn_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub burner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub collection: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub collection_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub collection_authority_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub collection_master_edition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub collection_master_edition_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub collection_metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub collection_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub creator: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub delegate: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub delegate_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub delegate_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "23")]
    pub destination_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "24")]
    pub destination_token_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "25")]
    pub edition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "26")]
    pub edition_mark_pda: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "27")]
    pub edition_marker: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "28")]
    pub edition_marker_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "29")]
    pub escrow: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "30")]
    pub escrow_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "31")]
    pub escrow_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "32")]
    pub master_edition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "33")]
    pub master_edition_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "34")]
    pub master_edition_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "35")]
    pub master_edition_token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "36")]
    pub master_edition_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "37")]
    pub master_metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "38")]
    pub master_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "39")]
    pub master_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "40")]
    pub metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "41")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "42")]
    pub mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "43")]
    pub new_collection_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "44")]
    pub new_edition: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "45")]
    pub new_metadata: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "46")]
    pub new_metadata_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "47")]
    pub new_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "48")]
    pub new_mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "49")]
    pub one_time_auth: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "50")]
    pub one_time_printing_authorization_mint:
        ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "51")]
    pub one_time_printing_authorization_mint_authority:
        ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "52")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "53")]
    pub owner_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "54")]
    pub owner_token_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "55")]
    pub payer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "56")]
    pub pda_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "57")]
    pub print_edition_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "58")]
    pub print_edition_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "59")]
    pub print_edition_token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "60")]
    pub printing_mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "61")]
    pub printing_mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "62")]
    pub rent: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "63")]
    pub reservation_list: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "64")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "65")]
    pub revoke_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "66")]
    pub safety_deposit_box: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "67")]
    pub safety_deposit_store: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "68")]
    pub spl_ata_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "69")]
    pub spl_token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "70")]
    pub system_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "71")]
    pub sysvar_instructions: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "72")]
    pub token: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "73")]
    pub token_account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "74")]
    pub token_account_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "75")]
    pub token_owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "76")]
    pub token_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "77")]
    pub token_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "78")]
    pub token_vault_program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "79")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "80")]
    pub use_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "81")]
    pub use_authority_record: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "82")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "83")]
    pub vault: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "84")]
    pub vault_authority: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct TokenMetadataMeta {
    #[prost(string, required, tag = "1")]
    pub block_date: ::prost::alloc::string::String,
    #[prost(int64, required, tag = "2")]
    pub block_time: i64,
    #[prost(string, required, tag = "3")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag = "4")]
    pub dapp: ::prost::alloc::string::String,
    #[prost(uint64, required, tag = "5")]
    pub block_slot: u64,
    #[prost(uint32, required, tag = "7")]
    pub instruction_index: u32,
    #[prost(bool, required, tag = "8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag = "9")]
    pub inner_instruction_index: u32,
    #[prost(string, required, tag = "10")]
    pub instruction_type: ::prost::alloc::string::String,
    #[prost(message, required, tag = "11")]
    pub args: Arg,
    #[prost(message, required, tag = "12")]
    pub input_accounts: InputAccounts,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize)]
pub struct Output {
    #[prost(message, repeated, tag = "1")]
    pub data: ::prost::alloc::vec::Vec<TokenMetadataMeta>,
}
// @@protoc_insertion_point(module)
