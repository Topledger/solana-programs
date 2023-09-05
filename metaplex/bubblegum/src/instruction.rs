// Bubblegum

extern crate bs58;
use core::fmt;

use borsh::{BorshDeserialize, BorshSerialize};

const CREATETREEDISCRIMINATOR: u64 = u64::from_le_bytes([165, 83, 136, 142, 89, 202, 47, 220]);
const SETTREEDELEGATEDISCRIMINATOR: u64 = u64::from_le_bytes([253, 118, 66, 37, 190, 49, 154, 102]);
const MINTV1DISCRIMINATOR: u64 = u64::from_le_bytes([145, 98, 192, 118, 184, 147, 118, 104]);
const MINTTOCOLLECTIONV1DISCRIMINATOR: u64 = u64::from_le_bytes([153, 18, 178, 47, 197, 158, 86, 15]);
const VERIFYCREATORDISCRIMINATOR: u64 = u64::from_le_bytes([52, 17, 96, 132, 71, 4, 85, 194]);
const UNVERIFYCREATORDISCRIMINATOR: u64 = u64::from_le_bytes([107, 178, 57, 39, 105, 115, 112, 152]);
const VERIFYCOLLECTIONDISCRIMINATOR: u64 = u64::from_le_bytes([56, 113, 101, 253, 79, 55, 122, 169]);
const UNVERIFYCOLLECTIONDISCRIMINATOR: u64 = u64::from_le_bytes([250, 251, 42, 106, 41, 137, 186, 168]);
const SETANDVERIFYCOLLECTIONDISCRIMINATOR: u64 = u64::from_le_bytes([235, 242, 121, 216, 158, 234, 180, 234]);
const TRANSFERDISCRIMINATOR: u64 = u64::from_le_bytes([163, 52, 200, 231, 140, 3, 69, 186]);
const DELEGATEDISCRIMINATOR: u64 = u64::from_le_bytes([90, 147, 75, 178, 85, 88, 4, 137]);
const BURNDISCRIMINATOR: u64 = u64::from_le_bytes([116, 110, 29, 56, 107, 219, 42, 93]);
const REDEEMDISCRIMINATOR: u64 = u64::from_le_bytes([184, 12, 86, 149, 70, 196, 97, 225]);
const CANCELREDEEMDISCRIMINATOR: u64 = u64::from_le_bytes([111, 76, 232, 50, 39, 175, 48, 242]);
const DECOMPRESSV1DISCRIMINATOR: u64 = u64::from_le_bytes([54, 85, 76, 70, 228, 250, 164, 81]);
const COMPRESSDISCRIMINATOR: u64 = u64::from_le_bytes([82, 193, 176, 117, 176, 21, 115, 253]);

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy)]
pub struct PubkeyLayout {
    pub value: [u8; 32]
}


#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Discriminator {
    pub value: u64,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidFormat,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum TokenStandardLayout {
    #[default]
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

impl fmt::Display for TokenStandardLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenStandardLayout::NonFungible => write!(f, "NonFungible"),
            TokenStandardLayout::FungibleAsset => write!(f, "FungibleAsset"),
            TokenStandardLayout::Fungible => write!(f, "Fungible"),
            TokenStandardLayout::NonFungibleEdition => write!(f, "NonFungibleEdition"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum UseMethodLayout {
    #[default]
    Burn,
    Multiple,
    Single,
}

impl fmt::Display for UseMethodLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UseMethodLayout::Burn => write!(f, "Burn"),
            UseMethodLayout::Multiple => write!(f, "Multiple"),
            UseMethodLayout::Single => write!(f, "Single"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Clone, Copy)]
pub struct CollectionLayout {
    pub verified: bool,
    pub key: PubkeyLayout
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Clone, Copy)]
pub struct UsesLayout {
    pub useMethod: UseMethodLayout,
    pub remaining: u64,
    pub total: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum TokenProgramVersionLayout {
    #[default]
    Original,
    Token2022,
}

impl fmt::Display for TokenProgramVersionLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenProgramVersionLayout::Original => write!(f, "Original"),
            TokenProgramVersionLayout::Token2022 => write!(f, "Token2022"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreatorLayout {
    pub address: PubkeyLayout,
    pub verified: bool,
    pub share: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MetadataArgsLayout {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub sellerFeeBasisPoints: u16,
    pub primarySaleHappened: bool,
    pub isMutable: bool,
    pub editionNonce: Option<u8>,
    pub tokenStandard: Option<TokenStandardLayout>,
    pub collection: Option<CollectionLayout>,
    pub uses: Option<UsesLayout>,
    pub tokenProgramVersion: TokenProgramVersionLayout,
    pub creators: Vec<CreatorLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreateTreeLayout {
    pub maxDepth: u32,
    pub maxBufferSize: u32,
    pub public: Option<bool>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetTreeDelegateLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MintV1Layout {
    pub message: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MintToCollectionV1Layout {
    pub metadataArgs: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct VerifyCreatorLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UnverifyCreatorLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct VerifyCollectionLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UnverifyCollectionLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetAndVerifyCollectionLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
    pub message: MetadataArgsLayout,
    pub collection: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TransferLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DelegateLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct BurnLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct RedeemLayout {
    pub root: [u8; 32],
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub nonce: u64,
    pub index: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CancelRedeemLayout {
    pub root: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DecompressV1Layout {
    pub metadata: MetadataArgsLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CompressLayout {}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub createTreeArgs: CreateTreeLayout,
    pub setTreeDelegateArgs: SetTreeDelegateLayout,
    pub mintV1Args: MintV1Layout,
    pub mintToCollectionV1Args: MintToCollectionV1Layout,
    pub verifyCreatorArgs: VerifyCreatorLayout,
    pub unverifyCreatorArgs: UnverifyCreatorLayout,
    pub verifyCollectionArgs: VerifyCollectionLayout,
    pub unverifyCollectionArgs: UnverifyCollectionLayout,
    pub setAndVerifyCollectionArgs: SetAndVerifyCollectionLayout,
    pub transferArgs: TransferLayout,
    pub delegateArgs: DelegateLayout,
    pub burnArgs: BurnLayout,
    pub redeemArgs: RedeemLayout,
    pub cancelRedeemArgs: CancelRedeemLayout,
    pub decompressV1Args: DecompressV1Layout,
    pub compressArgs: CompressLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    // let mut bytes_stream = bs58::decode(base58_string).into_vec().unwrap();
    let mut instruction_name = String::default();

    let mut createTreeArgs: CreateTreeLayout = CreateTreeLayout::default();
    let mut setTreeDelegateArgs: SetTreeDelegateLayout = SetTreeDelegateLayout::default();
    let mut mintV1Args: MintV1Layout = MintV1Layout::default();
    let mut mintToCollectionV1Args: MintToCollectionV1Layout = MintToCollectionV1Layout::default();
    let mut verifyCreatorArgs: VerifyCreatorLayout = VerifyCreatorLayout::default();
    let mut unverifyCreatorArgs: UnverifyCreatorLayout = UnverifyCreatorLayout::default();
    let mut verifyCollectionArgs: VerifyCollectionLayout = VerifyCollectionLayout::default();
    let mut unverifyCollectionArgs: UnverifyCollectionLayout = UnverifyCollectionLayout::default();
    let mut setAndVerifyCollectionArgs: SetAndVerifyCollectionLayout = SetAndVerifyCollectionLayout::default();
    let mut transferArgs: TransferLayout = TransferLayout::default();
    let mut delegateArgs: DelegateLayout = DelegateLayout::default();
    let mut burnArgs: BurnLayout = BurnLayout::default();
    let mut redeemArgs: RedeemLayout = RedeemLayout::default();
    let mut cancelRedeemArgs: CancelRedeemLayout = CancelRedeemLayout::default();
    let mut decompressV1Args: DecompressV1Layout = DecompressV1Layout::default();
    let mut compressArgs: CompressLayout = CompressLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(8);
    let discriminator = Discriminator::try_from_slice(&disc_bytes).unwrap().value;

    // let mutt :T

    match discriminator {
        CREATETREEDISCRIMINATOR => {
            instruction_name = String::from("CreateTree");
            createTreeArgs = CreateTreeLayout::try_from_slice(rest).unwrap();
        }
        SETTREEDELEGATEDISCRIMINATOR => {
            instruction_name = String::from("SetTreeDelegate");
            setTreeDelegateArgs = SetTreeDelegateLayout::try_from_slice(rest).unwrap();
        }
        MINTV1DISCRIMINATOR => {
            instruction_name = String::from("MintV1");
            mintV1Args = MintV1Layout::try_from_slice(rest).unwrap();
        }
        MINTTOCOLLECTIONV1DISCRIMINATOR => {
            instruction_name = String::from("MintToCollectionV1");
            mintToCollectionV1Args = MintToCollectionV1Layout::try_from_slice(rest).unwrap();
        }
        VERIFYCREATORDISCRIMINATOR => {
            instruction_name = String::from("VerifyCreator");
            verifyCreatorArgs = VerifyCreatorLayout::try_from_slice(rest).unwrap();
        }
        UNVERIFYCREATORDISCRIMINATOR => {
            instruction_name = String::from("UnverifyCreator");
            unverifyCreatorArgs = UnverifyCreatorLayout::try_from_slice(rest).unwrap();
        }
        VERIFYCOLLECTIONDISCRIMINATOR => {
            instruction_name = String::from("VerifyCollection");
            verifyCollectionArgs = VerifyCollectionLayout::try_from_slice(rest).unwrap();
        }
        UNVERIFYCOLLECTIONDISCRIMINATOR => {
            instruction_name = String::from("UnverifyCollection");
            unverifyCollectionArgs = UnverifyCollectionLayout::try_from_slice(rest).unwrap();
        }
        SETANDVERIFYCOLLECTIONDISCRIMINATOR => {
            instruction_name = String::from("SetAndVerifyCollection");
            setAndVerifyCollectionArgs =
                SetAndVerifyCollectionLayout::try_from_slice(rest).unwrap();
        }
        TRANSFERDISCRIMINATOR => {
            instruction_name = String::from("Transfer");
            transferArgs = TransferLayout::try_from_slice(rest).unwrap();
        }
        DELEGATEDISCRIMINATOR => {
            instruction_name = String::from("Delegate");
            delegateArgs = DelegateLayout::try_from_slice(rest).unwrap();
        }
        BURNDISCRIMINATOR => {
            instruction_name = String::from("Burn");
            burnArgs = BurnLayout::try_from_slice(rest).unwrap();
        }
        REDEEMDISCRIMINATOR => {
            instruction_name = String::from("Redeem");
            redeemArgs = RedeemLayout::try_from_slice(rest).unwrap();
        }
        CANCELREDEEMDISCRIMINATOR => {
            instruction_name = String::from("CancelRedeem");
            cancelRedeemArgs = CancelRedeemLayout::try_from_slice(rest).unwrap();
        }
        DECOMPRESSV1DISCRIMINATOR => {
            instruction_name = String::from("DecompressV1");
            decompressV1Args = DecompressV1Layout::try_from_slice(rest).unwrap();
        }
        COMPRESSDISCRIMINATOR => {
            instruction_name = String::from("Compress");
            compressArgs = CompressLayout::try_from_slice(rest).unwrap();
        }
        _ => {},
    }

    let result: Instruction = Instruction {
        name: instruction_name,
        createTreeArgs: createTreeArgs,
        setTreeDelegateArgs: setTreeDelegateArgs,
        mintV1Args: mintV1Args,
        mintToCollectionV1Args: mintToCollectionV1Args,
        verifyCreatorArgs: verifyCreatorArgs,
        unverifyCreatorArgs: unverifyCreatorArgs,
        verifyCollectionArgs: verifyCollectionArgs,
        unverifyCollectionArgs: unverifyCollectionArgs,
        setAndVerifyCollectionArgs: setAndVerifyCollectionArgs,
        transferArgs: transferArgs,
        delegateArgs: delegateArgs,
        burnArgs: burnArgs,
        redeemArgs: redeemArgs,
        cancelRedeemArgs: cancelRedeemArgs,
        decompressV1Args: decompressV1Args,
        compressArgs: compressArgs,
    };

    return result;
}
