use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbApproveUseAuthorityArgsLayout, PbAssetDataLayout, PbAuthorizationDataLayout,
    PbBurnArgsLayout, PbCollectionDetailsLayout, PbCollectionDetailsToggleLayout,
    PbCollectionLayout, PbCollectionToggleLayout, PbCreateArgsLayout,
    PbCreateMasterEditionArgsLayout, PbCreateMetadataAccountArgsLayout,
    PbCreateMetadataAccountArgsV2Layout, PbCreateMetadataAccountArgsV3Layout, PbCreatorLayout,
    PbDataLayout, PbDataV2Layout, PbDelegateArgsLayout, PbLeafInfoLayout, PbLockArgsLayout,
    PbMapItemLayout, PbMigrateArgsLayout, PbMigrationTypeLayout, PbMintArgsLayout,
    PbMintNewEditionFromMasterEditionViaTokenArgsLayout, PbMintPrintingTokensViaTokenArgsLayout,
    PbPayloadLayout, PbPayloadTypeLayout, PbPrintSupplyLayout, PbReservationLayout,
    PbRevokeArgsLayout, PbRuleSetToggleLayout, PbSeedsVecLayout, PbSeedsVecLayoutInner,
    PbSetCollectionSizeArgsLayout, PbSetReservationListArgsLayout, PbTransferArgsLayout,
    PbTransferOutOfEscrowArgsLayout, PbUnlockArgsLayout, PbUpdateArgsLayout,
    PbUpdateMetadataAccountArgsLayout, PbUpdateMetadataAccountArgsV2Layout, PbUseArgsLayout,
    PbUsesLayout, PbUsesToggleLayout, PbUtilizeArgsLayout, PbVerificationArgsLayout,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy)]
pub struct PubKeyLayout {
    pub value: [u8; 32],
}

impl PubKeyLayout {
    pub fn to_proto_struct(&self) -> String {
        let result = get_b58_string(self.value);
        result
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreatorLayout {
    pub address: PubKeyLayout,
    pub verified: bool,
    pub share: u8,
}

impl CreatorLayout {
    pub fn to_proto_struct(&self) -> PbCreatorLayout {
        PbCreatorLayout {
            address: self.address.to_proto_struct(),
            verified: self.verified,
            share: self.share as u32,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum TokenStandardLayout {
    #[default]
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
    ProgrammableNonFungible,
    ProgrammableNonFungibleEdition,
}

impl TokenStandardLayout {
    pub fn to_proto_struct(&self) -> String {
        let mut result = "NonFungible".to_string();

        match self {
            TokenStandardLayout::NonFungible => {
                result = "NonFungible".to_string();
            }
            TokenStandardLayout::FungibleAsset => {
                result = "FungibleAsset".to_string();
            }
            TokenStandardLayout::Fungible => {
                result = "Fungible".to_string();
            }
            TokenStandardLayout::NonFungibleEdition => {
                result = "NonFungibleEdition".to_string();
            }
            TokenStandardLayout::ProgrammableNonFungible => {
                result = "ProgrammableNonFungible".to_string();
            }
            TokenStandardLayout::ProgrammableNonFungibleEdition => {
                result = "ProgrammableNonFungibleEdition".to_string();
            }
        }

        return result;
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CollectionLayout {
    pub verified: bool,
    pub key: PubKeyLayout,
}

impl CollectionLayout {
    pub fn to_proto_struct(&self) -> PbCollectionLayout {
        PbCollectionLayout {
            verified: self.verified,
            key: self.key.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum UseMethodLayout {
    #[default]
    Burn,
    Multiple,
    Single,
}

impl UseMethodLayout {
    pub fn to_proto_struct(&self) -> String {
        let mut result = "".to_string();

        match self {
            UseMethodLayout::Burn => result = "Burn".to_string(),
            UseMethodLayout::Multiple => result = "Multiple".to_string(),
            UseMethodLayout::Single => result = "Single".to_string(),
        }

        result
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UsesLayout {
    pub useMethod: UseMethodLayout,
    pub remaining: u64,
    pub total: u64,
}

impl UsesLayout {
    pub fn to_proto_struct(&self) -> PbUsesLayout {
        PbUsesLayout {
            use_method: self.useMethod.to_proto_struct(),
            remaining: self.remaining.to_string(),
            total: self.total.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum CollectionDetailsLayout {
    V1 { size: u64 },
    V2 { padding: [u8; 8] },
}

impl Default for CollectionDetailsLayout {
    fn default() -> Self {
        CollectionDetailsLayout::V1 { size: 0 }
    }
}

impl CollectionDetailsLayout {
    pub fn to_proto_struct(&self) -> PbCollectionDetailsLayout {
        let mut name: String = "".to_string();
        let mut size: Option<String> = None;
        let mut padding: Vec<u32> = vec![];

        match self {
            CollectionDetailsLayout::V1 { size: _size } => {
                name = "V1".to_string();
                size = Some(_size.to_string());
            }
            CollectionDetailsLayout::V2 { padding: _padding } => {
                name = "V2".to_string();
                padding = _padding.iter().map(|x| *x as u32).collect();
            }
        }

        PbCollectionDetailsLayout {
            name: name,
            size: size,
            padding: padding.to_vec(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct AssetDataLayout {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub sellerFeeBasisPoints: u16,
    pub creators: Option<Vec<CreatorLayout>>,
    pub primarySaleHappened: bool,
    pub isMutable: bool,
    pub tokenStandard: TokenStandardLayout,
    pub collection: Option<CollectionLayout>,
    pub uses: Option<UsesLayout>,
    pub collectionDetails: Option<CollectionDetailsLayout>,
    pub ruleSet: Option<PubKeyLayout>,
}

impl AssetDataLayout {
    pub fn to_proto_struct(&self) -> PbAssetDataLayout {
        let mut creators: Vec<PbCreatorLayout> = vec![];
        if self.creators.is_some() {
            for x in self.creators.as_ref().unwrap().iter() {
                creators.push(x.to_proto_struct());
            }
        }

        let mut collection: Option<PbCollectionLayout> = None;
        if self.collection.is_some() {
            collection = Some(self.collection.as_ref().unwrap().to_proto_struct());
        }

        let mut uses: Option<PbUsesLayout> = None;
        if self.uses.is_some() {
            uses = Some(self.uses.as_ref().unwrap().to_proto_struct());
        }

        let mut collection_details: Option<PbCollectionDetailsLayout> = None;
        if self.collectionDetails.is_some() {
            collection_details = Some(self.collectionDetails.as_ref().unwrap().to_proto_struct());
        }

        let mut rule_set: Option<String> = None;
        if self.ruleSet.is_some() {
            rule_set = Some(self.ruleSet.unwrap().to_proto_struct());
        }

        PbAssetDataLayout {
            name: self.name.to_string(),
            symbol: self.symbol.to_string(),
            uri: self.uri.to_string(),
            seller_fee_basis_points: self.sellerFeeBasisPoints as u32,
            creators: creators,
            primary_sale_happened: self.primarySaleHappened,
            is_mutable: self.isMutable,
            token_standard: self.tokenStandard.to_proto_struct(),
            collection: collection,
            uses: uses,
            collection_details: collection_details,
            rule_set: rule_set,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SeedsVecLayoutInner {
    pub values: Vec<u8>,
}

impl SeedsVecLayoutInner {
    fn to_proto_struct(&self) -> PbSeedsVecLayoutInner {
        let mut values: Vec<u32> = vec![];
        for x in self.values.as_slice().iter() {
            values.push(*x as u32);
        }

        PbSeedsVecLayoutInner { values: values }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SeedsVecLayout {
    pub seeds: Vec<SeedsVecLayoutInner>,
}

impl SeedsVecLayout {
    pub fn to_proto_struct(&self) -> PbSeedsVecLayout {
        let mut seeds: Vec<PbSeedsVecLayoutInner> = vec![];
        for x in self.seeds.as_slice().iter() {
            seeds.push(x.to_proto_struct());
        }

        PbSeedsVecLayout { seeds: seeds }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct LeafInfoLayout {
    pub leaf: [u8; 32],
}

impl LeafInfoLayout {
    pub fn to_proto_struct(&self) -> PbLeafInfoLayout {
        let mut leaf: Vec<u32> = vec![];
        for x in self.leaf {
            leaf.push(x as u32);
        }
        PbLeafInfoLayout { leaf: leaf }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum PayloadTypeLayoutName {
    Pubkey { val: PubKeyLayout },
    Seeds { val: SeedsVecLayout },
    MerkleProof { val: LeafInfoLayout },
    Number { val: u64 },
}

impl Default for PayloadTypeLayoutName {
    fn default() -> Self {
        PayloadTypeLayoutName::Pubkey {
            val: PubKeyLayout::default(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct PayloadTypeLayout {
    pub name: PayloadTypeLayoutName,
}

impl PayloadTypeLayout {
    pub fn to_proto_struct(&self) -> PbPayloadTypeLayout {
        let mut name: String = "Pubkey".to_string();
        let mut val_pub_key = None;
        let mut val_seeds_vec = None;
        let mut val_leaf_info = None;
        let mut val_int64: Option<String> = None;

        match &self.name {
            PayloadTypeLayoutName::Pubkey { val: value } => {
                name = "Pubkey".to_string();
                val_pub_key = Some(value.to_proto_struct());
            }
            PayloadTypeLayoutName::Seeds { val: value } => {
                name = "Seeds".to_string();
                val_seeds_vec = Some(value.to_proto_struct());
            }
            PayloadTypeLayoutName::MerkleProof { val: value } => {
                name = "MerkleProof".to_string();
                val_leaf_info = Some(value.to_proto_struct());
            }
            PayloadTypeLayoutName::Number { val: value } => {
                name = "Number".to_string();
                val_int64 = Some(value.to_string());
            }
        }

        PbPayloadTypeLayout {
            name: name,
            val_pub_key: val_pub_key,
            val_seeds_vec: val_seeds_vec,
            val_leaf_info: val_leaf_info,
            val_int64: val_int64,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MapItemLayout {
    pub key: String,
    pub value: PayloadTypeLayout,
}

impl MapItemLayout {
    pub fn to_proto_struct(&self) -> PbMapItemLayout {
        PbMapItemLayout {
            key: self.key.clone().to_string(),
            val: self.value.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct PayloadLayout {
    pub map: Vec<MapItemLayout>,
}

impl PayloadLayout {
    pub fn to_proto_struct(&self) -> PbPayloadLayout {
        let mut map: Vec<PbMapItemLayout> = vec![];
        for x in self.map.iter() {
            map.push(x.to_proto_struct());
        }

        PbPayloadLayout { map: map }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct AuthorizationDataLayout {
    pub payload: PayloadLayout,
}

impl AuthorizationDataLayout {
    pub fn to_proto_struct(&self) -> PbAuthorizationDataLayout {
        PbAuthorizationDataLayout {
            payload: self.payload.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct DataLayout {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub sellerFeeBasisPoints: u16,
    pub creators: Option<Vec<CreatorLayout>>,
}

impl DataLayout {
    pub fn to_proto_struct(&self) -> PbDataLayout {
        let mut creators: Vec<PbCreatorLayout> = vec![];
        if self.creators.is_some() {
            for x in self.creators.as_ref().unwrap().iter() {
                creators.push(x.to_proto_struct());
            }
        }

        PbDataLayout {
            name: self.name.to_string(),
            symbol: self.symbol.to_string(),
            uri: self.uri.to_string(),
            seller_fee_basis_points: self.sellerFeeBasisPoints as u32,
            creators: creators,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum CreateArgsLayoutName {
    #[default]
    V1,
}

impl CreateArgsLayoutName {
    pub fn to_proto_struct(&self) -> String {
        let mut result = "".to_string();

        match self {
            CreateArgsLayoutName::V1 => {
                result = "V1".to_string();
            }
        }

        result
    }
}

#[derive(BorshDeserialize, Debug, Default, Clone)]
pub enum PrintSupplyLayoutName {
    #[default]
    Zero,
    Limited {
        val: u64,
    },
    Unlimited,
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct PrintSupplyLayout {
    pub name: PrintSupplyLayoutName,
}

impl PrintSupplyLayout {
    pub fn to_proto_struct(&self) -> PbPrintSupplyLayout {
        let mut name: String = "Zero".to_string();
        let mut val: Option<String> = None;

        match &self.name {
            PrintSupplyLayoutName::Zero => {
                name = "Zero".to_string();
            }
            PrintSupplyLayoutName::Limited { val: value } => {
                name = "Limited".to_string();
                val = Some(value.to_string());
            }
            PrintSupplyLayoutName::Unlimited => {
                name = "Unlimited".to_string();
            }
        }

        PbPrintSupplyLayout {
            name: name,
            val: val,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum CollectionToggleLayoutName {
    #[default]
    Blank,
    Clear,
    Set {
        val: CollectionLayout,
    },
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CollectionToggleLayout {
    pub name: CollectionToggleLayoutName,
}

impl CollectionToggleLayout {
    pub fn to_proto_struct(&self) -> PbCollectionToggleLayout {
        let mut name = "".to_string();
        let mut val = None;
        match &self.name {
            CollectionToggleLayoutName::Blank => {
                name = "Blank".to_string();
            }
            CollectionToggleLayoutName::Clear => {
                name = "Clear".to_string();
            }
            CollectionToggleLayoutName::Set { val: _val } => {
                name = "Set".to_string();
                val = Some(_val.to_proto_struct());
            }
        }
        PbCollectionToggleLayout { name, val }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum CollectionDetailsToggleLayoutName {
    #[default]
    Blank,
    Clear,
    Set {
        val: CollectionDetailsLayout,
    },
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CollectionDetailsToggleLayout {
    pub name: CollectionDetailsToggleLayoutName,
}

impl CollectionDetailsToggleLayout {
    pub fn to_proto_struct(&self) -> PbCollectionDetailsToggleLayout {
        let mut name = "".to_string();
        let mut val = None;

        match &self.name {
            CollectionDetailsToggleLayoutName::Blank => {
                name = "Blank".to_string();
            }
            CollectionDetailsToggleLayoutName::Clear => {
                name = "Clear".to_string();
            }
            CollectionDetailsToggleLayoutName::Set { val: _val } => {
                name = "Set".to_string();
                val = Some(_val.to_proto_struct());
            }
        }

        PbCollectionDetailsToggleLayout { name, val }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum UsesToggleLayoutName {
    #[default]
    Blank,
    Clear,
    Set {
        val: UsesLayout,
    },
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UsesToggleLayout {
    pub name: UsesToggleLayoutName,
}

impl UsesToggleLayout {
    pub fn to_proto_struct(&self) -> PbUsesToggleLayout {
        let mut name = "".to_string();
        let mut val = None;

        match &self.name {
            UsesToggleLayoutName::Blank => {
                name = "Blank".to_string();
            }
            UsesToggleLayoutName::Clear => {
                name = "Clear".to_string();
            }
            UsesToggleLayoutName::Set { val: _val } => {
                name = "Set".to_string();
                val = Some(_val.to_proto_struct());
            }
        }

        PbUsesToggleLayout { name, val }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum RuleSetToggleLayoutName {
    #[default]
    Blank,
    Clear,
    Set {
        val: PubKeyLayout,
    },
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct RuleSetToggleLayout {
    pub name: RuleSetToggleLayoutName,
}

impl RuleSetToggleLayout {
    pub fn to_proto_struct(&self) -> PbRuleSetToggleLayout {
        let mut name = "".to_string();
        let mut val = None;

        match self.name {
            RuleSetToggleLayoutName::Blank => {
                name = "Blank".to_string();
            }
            RuleSetToggleLayoutName::Clear => {
                name = "Clear".to_string();
            }
            RuleSetToggleLayoutName::Set { val: _val } => {
                name = "Set".to_string();
                val = Some(_val.to_proto_struct());
            }
        }

        PbRuleSetToggleLayout { name, val }
    }
}

//
// Instruction Layouts
//

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateMetadataAccountArgsLayout {
    pub data: DataLayout,
    pub isMutable: bool,
}

impl CreateMetadataAccountArgsLayout {
    pub fn to_proto_struct(&self) -> PbCreateMetadataAccountArgsLayout {
        PbCreateMetadataAccountArgsLayout {
            data: self.data.to_proto_struct(),
            is_mutable: self.isMutable,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdateMetadataAccountArgsLayout {
    pub data: Option<DataLayout>,
    pub updateAuthority: Option<PubKeyLayout>,
    pub primarySaleHappened: Option<bool>,
}

impl UpdateMetadataAccountArgsLayout {
    pub fn to_proto_struct(&self) -> PbUpdateMetadataAccountArgsLayout {
        let mut data = None;
        if self.data.is_some() {
            data = Some(self.data.as_ref().unwrap().to_proto_struct());
        }

        let mut update_authority = None;
        if self.updateAuthority.is_some() {
            update_authority = Some(self.updateAuthority.unwrap().to_proto_struct());
        }

        PbUpdateMetadataAccountArgsLayout {
            data: data,
            update_authority: update_authority,
            primary_sale_happened: self.primarySaleHappened,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateMasterEditionArgsLayout {
    pub maxSupply: Option<u64>,
}

impl CreateMasterEditionArgsLayout {
    pub fn to_proto_struct(&self) -> PbCreateMasterEditionArgsLayout {
        let mut max_supply: Option<String> = None;
        if self.maxSupply.is_some() {
            max_supply = Some(self.maxSupply.unwrap().to_string());
        }
        PbCreateMasterEditionArgsLayout {
            max_supply: max_supply,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ReservationLayout {
    pub address: PubKeyLayout,
    pub spotsRemaining: u64,
    pub totalSpots: u64,
}

impl ReservationLayout {
    pub fn to_proto_struct(&self) -> PbReservationLayout {
        PbReservationLayout {
            address: self.address.to_proto_struct(),
            spots_remaining: self.spotsRemaining.to_string(),
            total_spots: self.totalSpots.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetReservationListArgsLayout {
    pub reservations: Vec<ReservationLayout>,
    pub totalReservationSpots: Option<u64>,
    pub offset: u64,
    pub totalSpotOffset: u64,
}

impl SetReservationListArgsLayout {
    pub fn to_proto_struct(&self) -> PbSetReservationListArgsLayout {
        let mut reservations: Vec<PbReservationLayout> = vec![];
        if self.reservations.len() > 0 {
            for x in self.reservations.iter() {
                reservations.push(x.to_proto_struct());
            }
        }
        let mut total_reservation_spots: Option<String> = None;
        if self.totalReservationSpots.is_some() {
            total_reservation_spots = Some(self.totalReservationSpots.unwrap().to_string());
        }

        PbSetReservationListArgsLayout {
            reservations: reservations,
            total_reservation_spots: total_reservation_spots,
            offset: self.offset.to_string(),
            total_spot_offset: self.totalSpotOffset.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MintPrintingTokensViaTokenArgsLayout {
    pub supply: u64,
}

impl MintPrintingTokensViaTokenArgsLayout {
    pub fn to_proto_struct(&self) -> PbMintPrintingTokensViaTokenArgsLayout {
        PbMintPrintingTokensViaTokenArgsLayout {
            supply: self.supply.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MintNewEditionFromMasterEditionViaTokenArgsLayout {
    edition: u64,
}

impl MintNewEditionFromMasterEditionViaTokenArgsLayout {
    pub fn to_proto_struct(&self) -> PbMintNewEditionFromMasterEditionViaTokenArgsLayout {
        PbMintNewEditionFromMasterEditionViaTokenArgsLayout {
            edition: self.edition.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct DataV2Layout {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub sellerFeeBasisPoints: u16,
    pub creators: Option<Vec<CreatorLayout>>,
    pub collection: Option<CollectionLayout>,
    pub uses: Option<UsesLayout>,
}

impl DataV2Layout {
    pub fn to_proto_struct(&self) -> PbDataV2Layout {
        let mut creators: Vec<PbCreatorLayout> = vec![];
        if self.creators.is_some() {
            for x in self.creators.as_ref().unwrap().iter() {
                creators.push(x.to_proto_struct());
            }
        }

        let mut collection: Option<PbCollectionLayout> = None;
        if self.collection.is_some() {
            collection = Some(self.collection.as_ref().unwrap().to_proto_struct());
        }

        let mut uses: Option<PbUsesLayout> = None;
        if self.uses.is_some() {
            uses = Some(self.uses.as_ref().unwrap().to_proto_struct());
        }

        PbDataV2Layout {
            name: self.name.to_string(),
            symbol: self.symbol.to_string(),
            uri: self.uri.to_string(),
            seller_fee_basis_points: self.sellerFeeBasisPoints as u32,
            creators: creators,
            collection: collection,
            uses: uses,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdateMetadataAccountArgsV2Layout {
    pub data: Option<DataV2Layout>,
    pub updateAuthority: Option<PubKeyLayout>,
    pub primarySaleHappened: Option<bool>,
    pub isMutable: Option<bool>,
}

impl UpdateMetadataAccountArgsV2Layout {
    pub fn to_proto_struct(&self) -> PbUpdateMetadataAccountArgsV2Layout {
        let mut data: Option<PbDataV2Layout> = None;
        if self.data.is_some() {
            data = Some(self.data.as_ref().unwrap().to_proto_struct());
        }

        let mut update_authority: Option<String> = None;
        if self.updateAuthority.is_some() {
            update_authority = Some(self.updateAuthority.unwrap().to_proto_struct());
        }

        PbUpdateMetadataAccountArgsV2Layout {
            data: data,
            update_authority: update_authority,
            primary_sale_happened: self.primarySaleHappened,
            is_mutable: self.isMutable,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateMetadataAccountArgsV2Layout {
    pub data: DataV2Layout,
    pub isMutable: bool,
}

impl CreateMetadataAccountArgsV2Layout {
    pub fn to_proto_struct(&self) -> PbCreateMetadataAccountArgsV2Layout {
        PbCreateMetadataAccountArgsV2Layout {
            data: self.data.to_proto_struct(),
            is_mutable: self.isMutable,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UtilizeArgsLayout {
    pub numberOfUses: u64,
}

impl UtilizeArgsLayout {
    pub fn to_proto_struct(&self) -> PbUtilizeArgsLayout {
        PbUtilizeArgsLayout {
            number_of_uses: self.numberOfUses.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ApproveUseAuthorityArgsLayout {
    pub numberOfUses: u64,
}

impl ApproveUseAuthorityArgsLayout {
    pub fn to_proto_struct(&self) -> PbApproveUseAuthorityArgsLayout {
        PbApproveUseAuthorityArgsLayout {
            number_of_uses: self.numberOfUses.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateMetadataAccountArgsV3Layout {
    pub data: DataV2Layout,
    pub isMutable: bool,
    pub collectionDetails: Option<CollectionDetailsLayout>,
}

impl CreateMetadataAccountArgsV3Layout {
    pub fn to_proto_struct(&self) -> PbCreateMetadataAccountArgsV3Layout {
        let mut collection_details: Option<PbCollectionDetailsLayout> = None;
        if self.collectionDetails.is_some() {
            collection_details = Some(self.collectionDetails.as_ref().unwrap().to_proto_struct());
        }

        PbCreateMetadataAccountArgsV3Layout {
            data: self.data.to_proto_struct(),
            is_mutable: self.isMutable,
            collection_details: collection_details,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetCollectionSizeArgsLayout {
    pub size: u64,
}

impl SetCollectionSizeArgsLayout {
    pub fn to_proto_struct(&self) -> PbSetCollectionSizeArgsLayout {
        PbSetCollectionSizeArgsLayout {
            size: self.size.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct TransferOutOfEscrowArgsLayout {
    pub amount: u64,
}

impl TransferOutOfEscrowArgsLayout {
    pub fn to_proto_struct(&self) -> PbTransferOutOfEscrowArgsLayout {
        PbTransferOutOfEscrowArgsLayout {
            amount: self.amount.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum BurnArgsLayoutName {
    V1 { amount: u64 },
}

impl Default for BurnArgsLayoutName {
    fn default() -> Self {
        BurnArgsLayoutName::V1 { amount: 0 }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct BurnArgsLayout {
    pub name: BurnArgsLayoutName,
}

impl BurnArgsLayout {
    pub fn to_proto_struct(&self) -> PbBurnArgsLayout {
        let mut name = "".to_string();
        let mut amount = None;

        match self.name {
            BurnArgsLayoutName::V1 { amount: amt } => {
                name = "V1".to_string();
                amount = Some(amt);
            }
        }

        PbBurnArgsLayout {
            name: name,
            amount: amount.unwrap_or_default().to_string(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct CreateArgsLayout {
    pub name: CreateArgsLayoutName,
    pub asset_data: AssetDataLayout,
    pub decimals: Option<u8>,
    pub print_supply: Option<PrintSupplyLayout>,
}

impl CreateArgsLayout {
    pub fn to_proto_struct(&mut self) -> PbCreateArgsLayout {
        let mut decimals: Option<u32> = None;
        if self.decimals.is_some() {
            decimals = Some(self.decimals.unwrap() as u32);
        }

        let mut print_supply: Option<PbPrintSupplyLayout> = None;
        if self.print_supply.is_some() {
            print_supply = Some(self.print_supply.as_ref().unwrap().to_proto_struct());
        }

        PbCreateArgsLayout {
            name: self.name.to_proto_struct(),
            asset_data: self.asset_data.to_proto_struct(),
            decimals: decimals,
            print_supply: print_supply,
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum MintArgsLayoutName {
    V1 {
        amount: u64,
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for MintArgsLayoutName {
    fn default() -> Self {
        MintArgsLayoutName::V1 {
            amount: 0,
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MintArgsLayout {
    pub name: MintArgsLayoutName,
}

impl MintArgsLayout {
    pub fn to_proto_struct(&self) -> PbMintArgsLayout {
        let mut name = "".to_string();
        let mut amount: Option<String> = None;
        let mut authorization_data = None;

        match &self.name {
            MintArgsLayoutName::V1 {
                amount: amt,
                authorization_data: auth_data,
            } => {
                name = "V1".to_string();
                amount = Some(amt.to_string());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbMintArgsLayout {
            name: name,
            amount: amount,
            authorization_data: authorization_data,
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum DelegateArgsLayoutName {
    CollectionV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    SaleV1 {
        amount: u64,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    TransferV1 {
        amount: u64,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    UpdateV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    UtilityV1 {
        amount: u64,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    StakingV1 {
        amount: u64,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    StandardV1 {
        amount: u64,
    },
    LockedTransferV1 {
        amount: u64,
        locked_address: PubKeyLayout,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    ProgrammableConfigV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AuthorityItemV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    DataItemV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    CollectionItemV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    ProgrammableConfigItemV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
    PrintDelegateV1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for DelegateArgsLayoutName {
    fn default() -> Self {
        DelegateArgsLayoutName::CollectionV1 {
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct DelegateArgsLayout {
    pub name: DelegateArgsLayoutName,
}

impl DelegateArgsLayout {
    pub fn to_proto_struct(&self) -> PbDelegateArgsLayout {
        let mut name = "".to_string();
        let mut authorization_data = None;
        let mut amount: Option<String> = None;
        let mut locked_address = None;

        match &self.name {
            DelegateArgsLayoutName::CollectionV1 {
                authorization_data: auth_data,
            } => {
                name = "CollectionV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::SaleV1 {
                amount: amt,
                authorization_data: auth_data,
            } => {
                name = "SaleV1".to_string();
                amount = Some(amt.to_string());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::TransferV1 {
                amount: amt,
                authorization_data: auth_data,
            } => {
                name = "TransferV1".to_string();
                amount = Some(amt.to_string());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::UpdateV1 {
                authorization_data: auth_data,
            } => {
                name = "UpdateV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::UtilityV1 {
                amount: amt,
                authorization_data: auth_data,
            } => {
                name = "UtilityV1".to_string();
                amount = Some(amt.to_string());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::StakingV1 {
                amount: amt,
                authorization_data: auth_data,
            } => {
                name = "StakingV1".to_string();
                amount = Some(amt.to_string());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::StandardV1 { amount: amt } => {
                name = "StandardV1".to_string();
                amount = Some(amt.to_string());
            }
            DelegateArgsLayoutName::LockedTransferV1 {
                amount: amt,
                locked_address: locked_addr,
                authorization_data: auth_data,
            } => {
                name = "LockedTransferV1".to_string();
                locked_address = Some(locked_addr.to_proto_struct());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::ProgrammableConfigV1 {
                authorization_data: auth_data,
            } => {
                name = "ProgrammableConfigV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::AuthorityItemV1 {
                authorization_data: auth_data,
            } => {
                name = "AuthorityItemV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::DataItemV1 {
                authorization_data: auth_data,
            } => {
                name = "DataItemV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::CollectionItemV1 {
                authorization_data: auth_data,
            } => {
                name = "CollectionItemV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::ProgrammableConfigItemV1 {
                authorization_data: auth_data,
            } => {
                name = "ProgrammableConfigItemV1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
            DelegateArgsLayoutName::PrintDelegateV1 {
                authorization_data: auth_date,
            } => {
                name = "PrintDelegateV1".to_string();
                if auth_date.is_some() {
                    authorization_data = Some(auth_date.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbDelegateArgsLayout {
            name,
            authorization_data,
            amount,
            locked_address,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum RevokeArgsLayoutName {
    #[default]
    CollectionV1,
    SaleV1,
    TransferV1,
    DataV1,
    UtilityV1,
    StakingV1,
    StandardV1,
    LockedTransferV1,
    ProgrammableConfigV1,
    MigrationV1,
    AuthorityItemV1,
    DataItemV1,
    CollectionItemV1,
    ProgrammableConfigItemV1,
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct RevokeArgsLayout {
    pub name: RevokeArgsLayoutName,
}

impl RevokeArgsLayout {
    pub fn to_proto_struct(&self) -> PbRevokeArgsLayout {
        let mut result = "".to_string();
        match self.name {
            RevokeArgsLayoutName::CollectionV1 => {
                result = "CollectionV1".to_string();
            }
            RevokeArgsLayoutName::SaleV1 => {
                result = "SaleV1".to_string();
            }
            RevokeArgsLayoutName::TransferV1 => {
                result = "TransferV1".to_string();
            }
            RevokeArgsLayoutName::DataV1 => {
                result = "DataV1".to_string();
            }
            RevokeArgsLayoutName::UtilityV1 => {
                result = "UtilityV1".to_string();
            }
            RevokeArgsLayoutName::StakingV1 => {
                result = "StakingV1".to_string();
            }
            RevokeArgsLayoutName::StandardV1 => {
                result = "StandardV1".to_string();
            }
            RevokeArgsLayoutName::LockedTransferV1 => {
                result = "LockedTransferV1".to_string();
            }
            RevokeArgsLayoutName::ProgrammableConfigV1 => {
                result = "ProgrammableConfigV1".to_string();
            }
            RevokeArgsLayoutName::MigrationV1 => {
                result = "MigrationV1".to_string();
            }
            RevokeArgsLayoutName::AuthorityItemV1 => {
                result = "AuthorityItemV1".to_string();
            }
            RevokeArgsLayoutName::DataItemV1 => {
                result = "DataItemV1".to_string();
            }
            RevokeArgsLayoutName::CollectionItemV1 => {
                result = "CollectionItemV1".to_string();
            }
            RevokeArgsLayoutName::ProgrammableConfigItemV1 => {
                result = "ProgrammableConfigItemV1".to_string();
            }
        }

        PbRevokeArgsLayout { name: result }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum LockArgsLayoutName {
    V1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for LockArgsLayoutName {
    fn default() -> Self {
        LockArgsLayoutName::V1 {
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct LockArgsLayout {
    pub name: LockArgsLayoutName,
}

impl LockArgsLayout {
    pub fn to_proto_struct(&self) -> PbLockArgsLayout {
        let mut name = "".to_string();
        let mut authorization_data = None;

        match &self.name {
            LockArgsLayoutName::V1 {
                authorization_data: auth_data,
            } => {
                name = "V1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbLockArgsLayout {
            name: name,
            authorization_data: authorization_data,
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum UnlockArgsLayoutName {
    V1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for UnlockArgsLayoutName {
    fn default() -> Self {
        UnlockArgsLayoutName::V1 {
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UnlockArgsLayout {
    pub name: UnlockArgsLayoutName,
}

impl UnlockArgsLayout {
    pub fn to_proto_struct(&self) -> PbUnlockArgsLayout {
        let mut name = "".to_string();
        let mut authorization_data = None;

        match &self.name {
            UnlockArgsLayoutName::V1 {
                authorization_data: auth_data,
            } => {
                name = "V1".to_string();
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbUnlockArgsLayout {
            name: name,
            authorization_data: authorization_data,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum MigrationTypeLayout {
    #[default]
    CollectionV1,
    ProgrammableV1,
}

impl MigrationTypeLayout {
    pub fn to_proto_struct(&self) -> PbMigrationTypeLayout {
        let mut result: PbMigrationTypeLayout = PbMigrationTypeLayout::default();
        match self {
            MigrationTypeLayout::CollectionV1 => {
                result = PbMigrationTypeLayout {
                    name: "CollectionV1".to_string(),
                };
            }
            MigrationTypeLayout::ProgrammableV1 => {
                result = PbMigrationTypeLayout {
                    name: "ProgrammableV1".to_string(),
                };
            }
        }
        result
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum MigrateArgsLayoutName {
    V1 {
        migration_type: MigrationTypeLayout,
        rule_set: Option<PubKeyLayout>,
    },
}

impl Default for MigrateArgsLayoutName {
    fn default() -> Self {
        MigrateArgsLayoutName::V1 {
            migration_type: MigrationTypeLayout::CollectionV1,
            rule_set: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct MigrateArgsLayout {
    pub name: MigrateArgsLayoutName,
}

impl MigrateArgsLayout {
    pub fn to_proto_struct(&self) -> PbMigrateArgsLayout {
        let mut name = "".to_string();
        let mut migration_type = None;
        let mut rule_set = None;

        match &self.name {
            MigrateArgsLayoutName::V1 {
                migration_type: mig_type,
                rule_set: r_set,
            } => {
                name = "V1".to_string();
                migration_type = Some(mig_type.to_proto_struct());
                if r_set.is_some() {
                    rule_set = Some(r_set.unwrap().to_proto_struct());
                }
            }
        }

        PbMigrateArgsLayout {
            name: name,
            migration_type: migration_type,
            rule_set: rule_set,
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum TransferArgsLayoutName {
    V1 {
        amount: u64,
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for TransferArgsLayoutName {
    fn default() -> Self {
        TransferArgsLayoutName::V1 {
            amount: 0 as u64,
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct TransferArgsLayout {
    pub name: TransferArgsLayoutName,
}

impl TransferArgsLayout {
    pub fn to_proto_struct(&self) -> PbTransferArgsLayout {
        let mut name = "".to_string();
        let mut amount: Option<String> = None;
        let mut authorization_data = None;

        match &self.name {
            TransferArgsLayoutName::V1 {
                amount: amt,
                authorization_data: auth_data,
            } => {
                name = "V1".to_string();
                amount = Some(amt.to_string());
                if auth_data.is_some() {
                    authorization_data = Some(auth_data.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbTransferArgsLayout {
            name,
            amount,
            authorization_data,
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum UpdateArgsLayoutName {
    V1 {
        new_update_authority: Option<PubKeyLayout>,
        data: Option<DataLayout>,
        primary_sale_happened: Option<bool>,
        is_mutable: Option<bool>,
        collection: CollectionToggleLayout,
        collection_details: CollectionDetailsToggleLayout,
        uses: UsesToggleLayout,
        rule_set: RuleSetToggleLayout,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsUpdateAuthorityV2 {
        new_update_authority: Option<PubKeyLayout>,
        data: Option<DataLayout>,
        primary_sale_happened: Option<bool>,
        is_mutable: Option<bool>,
        collection: CollectionToggleLayout,
        collection_details: CollectionDetailsToggleLayout,
        uses: UsesToggleLayout,
        rule_set: RuleSetToggleLayout,
        token_standard: Option<TokenStandardLayout>,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsAuthorityItemDelegateV2 {
        new_update_authority: Option<PubKeyLayout>,
        primary_sale_happened: Option<bool>,
        is_mutable: Option<bool>,
        token_standard: Option<TokenStandardLayout>,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsCollectionDelegateV2 {
        collection: CollectionToggleLayout,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsDataDelegateV2 {
        data: Option<DataLayout>,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsProgrammableConfigDelegateV2 {
        rule_set: RuleSetToggleLayout,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsDataItemDelegateV2 {
        data: Option<DataLayout>,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsCollectionItemDelegateV2 {
        collection: CollectionToggleLayout,
        authorization_data: Option<AuthorizationDataLayout>,
    },
    AsProgrammableConfigItemDelegateV2 {
        rule_set: RuleSetToggleLayout,
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for UpdateArgsLayoutName {
    fn default() -> Self {
        UpdateArgsLayoutName::V1 {
            new_update_authority: None,
            data: None,
            primary_sale_happened: None,
            is_mutable: None,
            collection: CollectionToggleLayout::default(),
            collection_details: CollectionDetailsToggleLayout::default(),
            uses: UsesToggleLayout::default(),
            rule_set: RuleSetToggleLayout::default(),
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdateArgsLayout {
    pub name: UpdateArgsLayoutName,
}

impl UpdateArgsLayout {
    pub fn to_proto_struct(&self) -> PbUpdateArgsLayout {
        let mut name = "".to_string();
        let mut new_update_authority = None;
        let mut data: Option<PbDataLayout> = None;
        let mut primary_sale_happened = None;
        let mut is_mutable = None;
        let mut collection = None;
        let mut collection_details = None;
        let mut uses = None;
        let mut rule_set = None;
        let mut authorization_data = None;
        let mut token_standard = None;

        match &self.name {
            UpdateArgsLayoutName::V1 {
                new_update_authority: _new_update_authority,
                data: _data,
                primary_sale_happened: _primary_sale_happened,
                is_mutable: _is_mutable,
                collection: _collection,
                collection_details: _collection_details,
                uses: _uses,
                rule_set: _rule_set,
                authorization_data: _authorization_data,
            } => {
                name = "V1".to_string();

                if _new_update_authority.is_some() {
                    new_update_authority = Some(_new_update_authority.unwrap().to_proto_struct());
                }

                if _data.is_some() {
                    data = Some(_data.as_ref().unwrap().to_proto_struct());
                }

                if _primary_sale_happened.is_some() {
                    primary_sale_happened = Some(_primary_sale_happened.unwrap());
                }

                if _is_mutable.is_some() {
                    is_mutable = Some(_is_mutable.unwrap());
                }

                collection = Some(_collection.to_proto_struct());
                collection_details = Some(_collection_details.to_proto_struct());
                uses = Some(_uses.to_proto_struct());
                rule_set = Some(_rule_set.to_proto_struct());

                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsUpdateAuthorityV2 {
                new_update_authority: _new_update_authority,
                data: _data,
                primary_sale_happened: _primary_sale_happened,
                is_mutable: _is_mutable,
                collection: _collection,
                collection_details: _collection_details,
                uses: _uses,
                rule_set: _rule_set,
                token_standard: _token_standard,
                authorization_data: _authorization_data,
            } => {
                name = "AsUpdateAuthorityV2".to_string();

                if _new_update_authority.is_some() {
                    new_update_authority = Some(_new_update_authority.unwrap().to_proto_struct());
                }

                if _data.is_some() {
                    data = Some(_data.as_ref().unwrap().to_proto_struct());
                }

                if _primary_sale_happened.is_some() {
                    primary_sale_happened = Some(_primary_sale_happened.unwrap());
                }

                if _is_mutable.is_some() {
                    is_mutable = Some(_is_mutable.unwrap());
                }

                collection = Some(_collection.to_proto_struct());
                collection_details = Some(_collection_details.to_proto_struct());
                uses = Some(_uses.to_proto_struct());
                rule_set = Some(_rule_set.to_proto_struct());

                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }

                if _token_standard.is_some() {
                    token_standard = Some(_token_standard.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsAuthorityItemDelegateV2 {
                new_update_authority: _new_update_authority,
                primary_sale_happened: _primary_sale_happened,
                is_mutable: _is_mutable,
                token_standard: _token_standard,
                authorization_data: _authorization_data,
            } => {
                name = "AsAuthorityItemDelegateV2".to_string();

                if _new_update_authority.is_some() {
                    new_update_authority = Some(_new_update_authority.unwrap().to_proto_struct());
                }

                if _primary_sale_happened.is_some() {
                    primary_sale_happened = Some(_primary_sale_happened.unwrap());
                }

                if _is_mutable.is_some() {
                    is_mutable = Some(_is_mutable.unwrap());
                }

                if _token_standard.is_some() {
                    token_standard = Some(_token_standard.as_ref().unwrap().to_proto_struct());
                }

                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsCollectionDelegateV2 {
                collection: _collection,
                authorization_data: _authorization_data,
            } => {
                name = "AsCollectionDelegateV2".to_string();

                collection = Some(_collection.to_proto_struct());
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsDataDelegateV2 {
                data: _data,
                authorization_data: _authorization_data,
            } => {
                name = "AsDataDelegateV2".to_string();

                if _data.is_some() {
                    data = Some(_data.as_ref().unwrap().to_proto_struct());
                }
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsProgrammableConfigDelegateV2 {
                rule_set: _rule_set,
                authorization_data: _authorization_data,
            } => {
                name = "AsProgrammableConfigDelegateV2".to_string();
                rule_set = Some(_rule_set.to_proto_struct());
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsDataItemDelegateV2 {
                data: _data,
                authorization_data: _authorization_data,
            } => {
                name = "AsDataItemDelegateV2".to_string();
                if _data.is_some() {
                    data = Some(_data.as_ref().unwrap().to_proto_struct());
                }
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsCollectionItemDelegateV2 {
                collection: _collection,
                authorization_data: _authorization_data,
            } => {
                name = "AsCollectionItemDelegateV2".to_string();
                collection = Some(_collection.to_proto_struct());
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
            UpdateArgsLayoutName::AsProgrammableConfigItemDelegateV2 {
                rule_set: _rule_set,
                authorization_data: _authorization_data,
            } => {
                name = "AsProgrammableConfigItemDelegateV2".to_string();
                rule_set = Some(_rule_set.to_proto_struct());
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbUpdateArgsLayout {
            name,
            new_update_authority,
            data,
            primary_sale_happened,
            is_mutable,
            collection,
            collection_details,
            uses,
            rule_set,
            authorization_data,
            token_standard,
        }
    }
}

#[derive(BorshDeserialize, Debug)]
pub enum UseArgsLayoutName {
    V1 {
        authorization_data: Option<AuthorizationDataLayout>,
    },
}

impl Default for UseArgsLayoutName {
    fn default() -> Self {
        UseArgsLayoutName::V1 {
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UseArgsLayout {
    pub name: UseArgsLayoutName,
}

impl UseArgsLayout {
    pub fn to_proto_struct(&self) -> PbUseArgsLayout {
        let mut name = "".to_string();
        let mut authorization_data = None;

        match &self.name {
            UseArgsLayoutName::V1 {
                authorization_data: _authorization_data,
            } => {
                name = "V1".to_string();
                if _authorization_data.is_some() {
                    authorization_data =
                        Some(_authorization_data.as_ref().unwrap().to_proto_struct());
                }
            }
        }

        PbUseArgsLayout {
            name,
            authorization_data,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub enum VerificationArgsLayout {
    #[default]
    CreatorV1,
    CollectionV1,
}

impl VerificationArgsLayout {
    pub fn to_proto_struct(&self) -> PbVerificationArgsLayout {
        let mut result = "".to_string();
        match self {
            VerificationArgsLayout::CreatorV1 => {
                result = "CreatorV1".to_string();
            }
            VerificationArgsLayout::CollectionV1 => {
                result = "CollectionV1".to_string();
            }
        }
        PbVerificationArgsLayout { name: result }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
