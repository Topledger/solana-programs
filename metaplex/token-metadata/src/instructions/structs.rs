use borsh::{BorshDeserialize, BorshSerialize};

// use borsh::{BorshDeserialize, BorshSerialize};

// #[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy)]
// pub struct PubkeyLayout {
//     pub value: [u8; 32],
// }

// #[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
// #[repr(u8)]
// pub enum TokenStandardLayout {
//     #[default]
//     NonFungible,
//     FungibleAsset,
//     Fungible,
//     NonFungibleEdition,
// }

// impl fmt::Display for TokenStandardLayout {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             TokenStandardLayout::NonFungible => write!(f, "NonFungible"),
//             TokenStandardLayout::FungibleAsset => write!(f, "FungibleAsset"),
//             TokenStandardLayout::Fungible => write!(f, "Fungible"),
//             TokenStandardLayout::NonFungibleEdition => write!(f, "NonFungibleEdition"),
//         }
//     }
// }

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DataLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreateMetadataAccountArgsLayout {
    pub data: DataLayout,
    pub isMutable: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreateMetadataAccountLayout {
    pub createMetadataAccountArgs: CreateMetadataAccountArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UpdateMetadataAccountArgsLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreateMasterEditionArgsLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetReservationListArgsLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MintPrintingTokensViaTokenArgsLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MintNewEditionFromMasterEditionViaTokenArgsLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UpdateMetadataAccountArgsV2Layout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreateMetadataAccountArgsV2Layout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UtilizeArgsLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ApproveUseAuthorityArgsLayout {}
