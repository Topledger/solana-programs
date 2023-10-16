extern crate bs58;
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use std::default;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy)]
pub struct PubkeyLayout {
    pub value: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeMintLayout {
    pub decimals: u8,
    pub mint_authority: PubkeyLayout,
    pub freeze_authority_option: u8,
    pub freeze_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeMultisigLayout {
    pub status: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TransferLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ApproveLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct RevokeLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum AuthorityTypeLayout {
    #[default]
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
    TransferFeeConfig,
    WithheldWithdraw,
    CloseMint,
    InterestRate,
}

impl fmt::Display for AuthorityTypeLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthorityTypeLayout::MintTokens => write!(f, "MintTokens"),
            AuthorityTypeLayout::FreezeAccount => write!(f, "FreezeAccount"),
            AuthorityTypeLayout::AccountOwner => write!(f, "AccountOwner"),
            AuthorityTypeLayout::CloseAccount => write!(f, "CloseAccount"),
            AuthorityTypeLayout::TransferFeeConfig => write!(f, "TransferFeeConfig"),
            AuthorityTypeLayout::WithheldWithdraw => write!(f, "WithheldWithdraw"),
            AuthorityTypeLayout::CloseMint => write!(f, "CloseMint"),
            AuthorityTypeLayout::InterestRate => write!(f, "InterestRate"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetAuthorityLayout {
    pub authority_type: AuthorityTypeLayout,
    pub new_authority_option: u8,
    pub new_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MintToLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct BurnLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CloseAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct FreezeAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ThawAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TransferCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ApproveCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MintToCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct BurnCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeAccount2Layout {
    pub owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SyncNativeLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeAccount3Layout {
    pub owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeMultisig2Layout {
    pub status: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeMint2Layout {
    pub decimals: u8,
    pub mint_authority: PubkeyLayout,
    pub freeze_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct GetAccountDataSizeLayout {
    pub extension_type: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeImmutableOwnerLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct AmountToUiAmountLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UiAmountToAmountLayout {
    pub ui_amount: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeMintCloseAuthorityLayout {
    pub owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TransferFeeExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ConfidentialTransferExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DefaultAccountStateExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ReallocateLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct MemoTransferExtensionLayout {}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CreateNativeMintLayout {}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeNonTransferableMintLayout {}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InterestBearingMintExtensionLayout {}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub initializeMintArgs: InitializeMintLayout,
    pub initializeAccountArgs: InitializeAccountLayout,
    pub initializeMultisigArgs: InitializeMultisigLayout,
    pub transferArgs: TransferLayout,
    pub approveArgs: ApproveLayout,
    pub revokeArgs: RevokeLayout,
    pub setAuthorityArgs: SetAuthorityLayout,
    pub mintToArgs: MintToLayout,
    pub burnArgs: BurnLayout,
    pub closeAccountArgs: CloseAccountLayout,
    pub freezeAccountArgs: FreezeAccountLayout,
    pub thawAccountArgs: ThawAccountLayout,
    pub transferCheckedArgs: TransferCheckedLayout,
    pub approveCheckedArgs: ApproveCheckedLayout,
    pub mintToCheckedArgs: MintToCheckedLayout,
    pub burnCheckedArgs: BurnCheckedLayout,
    pub initializeAccount2Args: InitializeAccount2Layout,
    pub syncNativeArgs: SyncNativeLayout,
    pub initializeAccount3Args: InitializeAccount3Layout,
    pub initializeMultisig2Args: InitializeMultisig2Layout,
    pub initializeMint2Args: InitializeMint2Layout,
    pub getAccountDataSizeArgs: GetAccountDataSizeLayout,
    pub initializeImmutableOwnerArgs: InitializeImmutableOwnerLayout,
    pub amountToUiAmountArgs: AmountToUiAmountLayout,
    pub uiAmountToAmountArgs: UiAmountToAmountLayout,
    pub initializeMintCloseAuthorityArgs: InitializeMintCloseAuthorityLayout,
    pub transferFeeExtensionArgs: TransferFeeExtensionLayout,
    pub confidentialTransferExtensionArgs: ConfidentialTransferExtensionLayout,
    pub defaultAccountStateExtensionArgs: DefaultAccountStateExtensionLayout,
    pub reallocateArgs: ReallocateLayout,
    pub memoTransferExtensionArgs: MemoTransferExtensionLayout,
    pub createNativeMintArgs: CreateNativeMintLayout,
    pub initializeNonTransferableMintArgs: InitializeNonTransferableMintLayout,
    pub interestBearingMintExtensionArgs: InterestBearingMintExtensionLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    // let mut bytes_stream = bs58::decode(base58_string).into_vec().unwrap();
    let mut instruction_name = String::default();

    let mut initializeMintArgs: InitializeMintLayout = InitializeMintLayout::default();
    let mut initializeAccountArgs: InitializeAccountLayout = InitializeAccountLayout::default();
    let mut initializeMultisigArgs: InitializeMultisigLayout = InitializeMultisigLayout::default();
    let mut transferArgs: TransferLayout = TransferLayout::default();
    let mut approveArgs: ApproveLayout = ApproveLayout::default();
    let mut revokeArgs: RevokeLayout = RevokeLayout::default();
    let mut setAuthorityArgs: SetAuthorityLayout = SetAuthorityLayout::default();
    let mut mintToArgs: MintToLayout = MintToLayout::default();
    let mut burnArgs: BurnLayout = BurnLayout::default();
    let mut closeAccountArgs: CloseAccountLayout = CloseAccountLayout::default();
    let mut freezeAccountArgs: FreezeAccountLayout = FreezeAccountLayout::default();
    let mut thawAccountArgs: ThawAccountLayout = ThawAccountLayout::default();
    let mut transferCheckedArgs: TransferCheckedLayout = TransferCheckedLayout::default();
    let mut approveCheckedArgs: ApproveCheckedLayout = ApproveCheckedLayout::default();
    let mut mintToCheckedArgs: MintToCheckedLayout = MintToCheckedLayout::default();
    let mut burnCheckedArgs: BurnCheckedLayout = BurnCheckedLayout::default();
    let mut initializeAccount2Args: InitializeAccount2Layout = InitializeAccount2Layout::default();
    let mut syncNativeArgs: SyncNativeLayout = SyncNativeLayout::default();
    let mut initializeAccount3Args: InitializeAccount3Layout = InitializeAccount3Layout::default();
    let mut initializeMultisig2Args: InitializeMultisig2Layout =
        InitializeMultisig2Layout::default();
    let mut initializeMint2Args: InitializeMint2Layout = InitializeMint2Layout::default();
    let mut getAccountDataSizeArgs: GetAccountDataSizeLayout = GetAccountDataSizeLayout::default();
    let mut initializeImmutableOwnerArgs: InitializeImmutableOwnerLayout =
        InitializeImmutableOwnerLayout::default();
    let mut amountToUiAmountArgs: AmountToUiAmountLayout = AmountToUiAmountLayout::default();
    let mut uiAmountToAmountArgs: UiAmountToAmountLayout = UiAmountToAmountLayout::default();
    let mut initializeMintCloseAuthorityArgs: InitializeMintCloseAuthorityLayout =
        InitializeMintCloseAuthorityLayout::default();
    let mut transferFeeExtensionArgs: TransferFeeExtensionLayout =
        TransferFeeExtensionLayout::default();
    let mut confidentialTransferExtensionArgs: ConfidentialTransferExtensionLayout =
        ConfidentialTransferExtensionLayout::default();
    let mut defaultAccountStateExtensionArgs: DefaultAccountStateExtensionLayout =
        DefaultAccountStateExtensionLayout::default();
    let mut reallocateArgs: ReallocateLayout = ReallocateLayout::default();
    let mut memoTransferExtensionArgs: MemoTransferExtensionLayout =
        MemoTransferExtensionLayout::default();
    let mut createNativeMintArgs: CreateNativeMintLayout = CreateNativeMintLayout::default();
    let mut initializeNonTransferableMintArgs: InitializeNonTransferableMintLayout =
        InitializeNonTransferableMintLayout::default();
    let mut interestBearingMintExtensionArgs: InterestBearingMintExtensionLayout =
        InterestBearingMintExtensionLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    match discriminator {
        0 => {
            instruction_name = String::from("InitializeMint");
            initializeMintArgs = InitializeMintLayout::try_from_slice(rest).unwrap_or_default();
        }
        1 => {
            instruction_name = String::from("InitializeAccount");
            initializeAccountArgs = InitializeAccountLayout::try_from_slice(rest).unwrap();
        }
        2 => {
            instruction_name = String::from("InitializeMultisig");
            initializeMultisigArgs = InitializeMultisigLayout::try_from_slice(rest).unwrap();
        }
        3 => {
            instruction_name = String::from("Transfer");
            transferArgs = TransferLayout::try_from_slice(rest).unwrap();
        }
        4 => {
            instruction_name = String::from("Approve");
            approveArgs = ApproveLayout::try_from_slice(rest).unwrap();
        }
        5 => {
            instruction_name = String::from("Revoke");
            revokeArgs = RevokeLayout::try_from_slice(rest).unwrap();
        }
        6 => {
            instruction_name = String::from("SetAuthority");
            setAuthorityArgs = SetAuthorityLayout::try_from_slice(rest).unwrap_or_default();
        }
        7 => {
            instruction_name = String::from("MintTo");
            mintToArgs = MintToLayout::try_from_slice(rest).unwrap();
        }
        8 => {
            instruction_name = String::from("Burn");
            burnArgs = BurnLayout::try_from_slice(rest).unwrap();
        }
        9 => {
            instruction_name = String::from("CloseAccount");
        }
        10 => {
            instruction_name = String::from("FreezeAccount");
        }
        11 => {
            instruction_name = String::from("ThawAccount");
        }
        12 => {
            instruction_name = String::from("TransferChecked");
            transferCheckedArgs = TransferCheckedLayout::try_from_slice(rest).unwrap();
        }
        13 => {
            instruction_name = String::from("ApproveChecked");
            approveCheckedArgs = ApproveCheckedLayout::try_from_slice(rest).unwrap();
        }
        14 => {
            instruction_name = String::from("MintToChecked");
            mintToCheckedArgs = MintToCheckedLayout::try_from_slice(rest).unwrap();
        }
        15 => {
            instruction_name = String::from("BurnChecked");
            burnCheckedArgs = BurnCheckedLayout::try_from_slice(rest).unwrap();
        }
        16 => {
            instruction_name = String::from("InitializeAccount2");
            initializeAccount2Args = InitializeAccount2Layout::try_from_slice(rest).unwrap();
        }
        17 => {
            instruction_name = String::from("SyncNative");
            syncNativeArgs = SyncNativeLayout::try_from_slice(rest).unwrap();
        }
        18 => {
            instruction_name = String::from("InitializeAccount3");
            initializeAccount3Args = InitializeAccount3Layout::try_from_slice(rest).unwrap();
        }
        19 => {
            instruction_name = String::from("InitializeMultisig2");
            initializeMultisig2Args = InitializeMultisig2Layout::try_from_slice(rest).unwrap();
        }
        20 => {
            instruction_name = String::from("InitializeMint2");
            initializeMint2Args = InitializeMint2Layout::try_from_slice(rest).unwrap_or_default();
        }
        21 => {
            instruction_name = String::from("GetAccountDataSize");
            getAccountDataSizeArgs = GetAccountDataSizeLayout::try_from_slice(rest).unwrap();
        }
        22 => {
            instruction_name = String::from("InitializeImmutableOwner");
            initializeImmutableOwnerArgs =
                InitializeImmutableOwnerLayout::try_from_slice(rest).unwrap();
        }
        23 => {
            instruction_name = String::from("AmountToUiAmount");
            amountToUiAmountArgs = AmountToUiAmountLayout::try_from_slice(rest).unwrap();
        }
        24 => {
            instruction_name = String::from("UiAmountToAmount");
            uiAmountToAmountArgs = UiAmountToAmountLayout::try_from_slice(rest).unwrap();
        }
        25 => {
            instruction_name = String::from("InitializeMintCloseAuthority");
            initializeMintCloseAuthorityArgs =
                InitializeMintCloseAuthorityLayout::try_from_slice(rest).unwrap();
        }
        26 => {
            instruction_name = String::from("TransferFeeExtension");
        }
        27 => {
            instruction_name = String::from("ConfidentialTransferExtension");
        }
        28 => {
            instruction_name = String::from("DefaultAccountStateExtension");
        }
        29 => {
            instruction_name = String::from("Reallocate");
        }
        30 => {
            instruction_name = String::from("MemoTransferExtension");
        }
        31 => {
            instruction_name = String::from("CreateNativeMint");
        }
        32 => {
            instruction_name = String::from("InitializeNonTransferableMint");
        }
        33 => {
            instruction_name = String::from("InterestBearingMintExtension");
        }
        _ => {}
    }

    let result: Instruction = Instruction {
        name: instruction_name,
        initializeMintArgs: initializeMintArgs,
        initializeAccountArgs: initializeAccountArgs,
        initializeMultisigArgs: initializeMultisigArgs,
        transferArgs: transferArgs,
        approveArgs: approveArgs,
        revokeArgs: revokeArgs,
        setAuthorityArgs,
        mintToArgs: mintToArgs,
        burnArgs: burnArgs,
        closeAccountArgs: closeAccountArgs,
        freezeAccountArgs: freezeAccountArgs,
        thawAccountArgs: thawAccountArgs,
        transferCheckedArgs: transferCheckedArgs,
        approveCheckedArgs: approveCheckedArgs,
        mintToCheckedArgs: mintToCheckedArgs,
        burnCheckedArgs: burnCheckedArgs,
        initializeAccount2Args: initializeAccount2Args,
        syncNativeArgs: syncNativeArgs,
        initializeAccount3Args: initializeAccount3Args,
        initializeMultisig2Args: initializeMultisig2Args,
        initializeMint2Args: initializeMint2Args,
        getAccountDataSizeArgs: getAccountDataSizeArgs,
        initializeImmutableOwnerArgs: initializeImmutableOwnerArgs,
        amountToUiAmountArgs: amountToUiAmountArgs,
        uiAmountToAmountArgs: uiAmountToAmountArgs,
        initializeMintCloseAuthorityArgs: initializeMintCloseAuthorityArgs,
        transferFeeExtensionArgs: transferFeeExtensionArgs,
        confidentialTransferExtensionArgs: confidentialTransferExtensionArgs,
        defaultAccountStateExtensionArgs: defaultAccountStateExtensionArgs,
        reallocateArgs: reallocateArgs,
        memoTransferExtensionArgs: memoTransferExtensionArgs,
        createNativeMintArgs: createNativeMintArgs,
        initializeNonTransferableMintArgs: initializeNonTransferableMintArgs,
        interestBearingMintExtensionArgs: interestBearingMintExtensionArgs,
    };

    return result;
}
