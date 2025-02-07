extern crate bs58;
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy, Serialize)]
pub struct PubkeyLayout {
    pub value: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMintLayout {
    pub decimals: u8,
    pub mint_authority: PubkeyLayout,
    pub freeze_authority_option: u8,
    pub freeze_authority: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMultisigLayout {
    pub status: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct TransferLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ApproveLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct RevokeLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
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

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct SetAuthorityLayout {
    pub authority_type: AuthorityTypeLayout,
    pub new_authority_option: u8,
    pub new_authority: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct MintToLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct BurnLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct CloseAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct FreezeAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ThawAccountLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct TransferCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ApproveCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct MintToCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct BurnCheckedLayout {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeAccount2Layout {
    pub owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct SyncNativeLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeAccount3Layout {
    pub owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMultisig2Layout {
    pub status: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMint2Layout {
    pub decimals: u8,
    pub mint_authority: PubkeyLayout,
    pub freeze_authority: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct GetAccountDataSizeLayout {
    pub extension_type: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeImmutableOwnerLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct AmountToUiAmountLayout {
    pub amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct UiAmountToAmountLayout {
    pub ui_amount: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMintCloseAuthorityLayout {
    pub owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct TransferFeeExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ConfidentialTransferExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct DefaultAccountStateExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ReallocateLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct MemoTransferExtensionLayout {}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct CreateNativeMintLayout {}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeNonTransferableMintLayout {}
#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InterestBearingMintExtensionLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InstructionAccounts {
    pub mint: String,
    pub rent_sysvar: String,
    pub account: String,
    pub owner: String,
    pub signer_accounts: Vec<String>,
    pub source: String,
    pub destination: String,
    pub delegate: String,
    pub authority: String,
    pub payer: String,
    pub fund_relocation_sys_program: String,
    pub funding_account: String,
    pub mint_funding_sys_program: String,
}

#[derive(Debug, Serialize)]
pub struct Instruction {
    pub joinKey: String,
    pub name: String,
    pub instruction_accounts: InstructionAccounts,
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

pub fn parse_instruction(bytes_stream: Vec<u8>, accounts: Vec<String>) -> Instruction {
    // let mut bytes_stream = bs58::decode(base58_string).into_vec().unwrap();
    let mut instruction_name = String::default();
    let mut instruction_accounts = InstructionAccounts::default();

    let mut initializeMintArgs: InitializeMintLayout = InitializeMintLayout::default();
    let initializeAccountArgs: InitializeAccountLayout = InitializeAccountLayout::default();
    let mut initializeMultisigArgs: InitializeMultisigLayout = InitializeMultisigLayout::default();
    let mut transferArgs: TransferLayout = TransferLayout::default();
    let mut approveArgs: ApproveLayout = ApproveLayout::default();
    let revokeArgs: RevokeLayout = RevokeLayout::default();
    let mut setAuthorityArgs: SetAuthorityLayout = SetAuthorityLayout::default();
    let mut mintToArgs: MintToLayout = MintToLayout::default();
    let mut burnArgs: BurnLayout = BurnLayout::default();
    let closeAccountArgs: CloseAccountLayout = CloseAccountLayout::default();
    let freezeAccountArgs: FreezeAccountLayout = FreezeAccountLayout::default();
    let thawAccountArgs: ThawAccountLayout = ThawAccountLayout::default();
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
    let transferFeeExtensionArgs: TransferFeeExtensionLayout =
        TransferFeeExtensionLayout::default();
    let confidentialTransferExtensionArgs: ConfidentialTransferExtensionLayout =
        ConfidentialTransferExtensionLayout::default();
    let defaultAccountStateExtensionArgs: DefaultAccountStateExtensionLayout =
        DefaultAccountStateExtensionLayout::default();
    let reallocateArgs: ReallocateLayout = ReallocateLayout::default();
    let memoTransferExtensionArgs: MemoTransferExtensionLayout =
        MemoTransferExtensionLayout::default();
    let createNativeMintArgs: CreateNativeMintLayout = CreateNativeMintLayout::default();
    let initializeNonTransferableMintArgs: InitializeNonTransferableMintLayout =
        InitializeNonTransferableMintLayout::default();
    let interestBearingMintExtensionArgs: InterestBearingMintExtensionLayout =
        InterestBearingMintExtensionLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    match discriminator {
        0 => {
            instruction_name = String::from("InitializeMint");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(1).unwrap().to_string();

            initializeMintArgs = InitializeMintLayout::try_from_slice(rest).unwrap_or_default();
        }
        1 => {
            instruction_name = String::from("InitializeAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }
        }
        2 => {
            instruction_name = String::from("InitializeMultisig");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(1).unwrap().to_string();
            if accounts.len() > 2 {
                instruction_accounts.signer_accounts = accounts.split_at(2).1.to_vec();
            }

            initializeMultisigArgs = InitializeMultisigLayout::try_from_slice(rest).unwrap();
        }
        3 => {
            instruction_name = String::from("Transfer");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.destination = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }

            if rest.len() > 8 {
                let (rest_split, _) = rest.split_at(8);
                transferArgs = TransferLayout::try_from_slice(rest_split).unwrap();
            } else {
                transferArgs = TransferLayout::try_from_slice(rest).unwrap();
            }
        }
        4 => {
            instruction_name = String::from("Approve");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.delegate = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }

            if rest.len() > 8 {
                let (rest_split, _) = rest.split_at(8);
                approveArgs = ApproveLayout::try_from_slice(rest_split).unwrap();
            } else {
                approveArgs = ApproveLayout::try_from_slice(rest).unwrap();
            }
        }
        5 => {
            instruction_name = String::from("Revoke");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.owner = accounts.get(1).unwrap().to_string();
            if accounts.len() > 2 {
                instruction_accounts.signer_accounts = accounts.split_at(2).1.to_vec();
            }
        }
        6 => {
            instruction_name = String::from("SetAuthority");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.authority = accounts.get(1).unwrap().to_string();
            if accounts.len() > 2 {
                instruction_accounts.signer_accounts = accounts.split_at(2).1.to_vec();
            }

            setAuthorityArgs = SetAuthorityLayout::try_from_slice(rest).unwrap_or_default();
        }
        7 => {
            instruction_name = String::from("MintTo");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.account = accounts.get(1).unwrap().to_string();
            instruction_accounts.authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }

            if rest.len() > 8 {
                let (rest_split, _) = rest.split_at(8);
                mintToArgs = MintToLayout::try_from_slice(rest_split).unwrap();
            } else {
                mintToArgs = MintToLayout::try_from_slice(rest).unwrap();
            }
        }
        8 => {
            instruction_name = String::from("Burn");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }
            let rest_bytes = &mut rest.clone();

            burnArgs = BurnLayout::deserialize(rest_bytes).unwrap();
        }
        9 => {
            instruction_name = String::from("CloseAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.destination = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }
        }
        10 => {
            instruction_name = String::from("FreezeAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }
        }
        11 => {
            instruction_name = String::from("ThawAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }
        }
        12 => {
            instruction_name = String::from("TransferChecked");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.destination = accounts.get(2).unwrap().to_string();
            instruction_accounts.owner = accounts.get(3).unwrap().to_string();
            if accounts.len() > 4 {
                instruction_accounts.signer_accounts = accounts.split_at(4).1.to_vec();
            }

            transferCheckedArgs = TransferCheckedLayout::deserialize(&mut rest.clone()).unwrap();
        }
        13 => {
            instruction_name = String::from("ApproveChecked");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.delegate = accounts.get(2).unwrap().to_string();
            instruction_accounts.owner = accounts.get(3).unwrap().to_string();
            if accounts.len() > 4 {
                instruction_accounts.signer_accounts = accounts.split_at(4).1.to_vec();
            }

            approveCheckedArgs = ApproveCheckedLayout::try_from_slice(rest).unwrap();
        }
        14 => {
            instruction_name = String::from("MintToChecked");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.account = accounts.get(1).unwrap().to_string();
            instruction_accounts.authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }

            mintToCheckedArgs = MintToCheckedLayout::try_from_slice(rest).unwrap();
        }
        15 => {
            instruction_name = String::from("BurnChecked");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signer_accounts = accounts.split_at(3).1.to_vec();
            }

            burnCheckedArgs = BurnCheckedLayout::try_from_slice(rest).unwrap();
        }
        16 => {
            instruction_name = String::from("InitializeAccount2");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(2).unwrap().to_string();

            initializeAccount2Args = InitializeAccount2Layout::try_from_slice(rest).unwrap();
        }
        17 => {
            instruction_name = String::from("SyncNative");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();

            if rest.len() > 0 {
                let (rest_split, _) = rest.split_at(0);
                syncNativeArgs = SyncNativeLayout::try_from_slice(rest_split).unwrap();
            } else {
                syncNativeArgs = SyncNativeLayout::try_from_slice(rest).unwrap();
            }
        }
        18 => {
            instruction_name = String::from("InitializeAccount3");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();

            initializeAccount3Args =
                InitializeAccount3Layout::deserialize((&mut rest.clone())).unwrap();
        }
        19 => {
            instruction_name = String::from("InitializeMultisig2");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            if accounts.len() > 1 {
                instruction_accounts.signer_accounts = accounts.split_at(1).1.to_vec();
            }

            initializeMultisig2Args = InitializeMultisig2Layout::try_from_slice(rest).unwrap();
        }
        20 => {
            instruction_name = String::from("InitializeMint2");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            initializeMint2Args = InitializeMint2Layout::try_from_slice(rest).unwrap_or_default();
        }
        21 => {
            instruction_name = String::from("GetAccountDataSize");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            if rest.len() > 1 {
                let (rest_split, _) = rest.split_at(1);
                getAccountDataSizeArgs =
                    GetAccountDataSizeLayout::try_from_slice(rest_split).unwrap();
            } else if rest.len() == 0 {
                getAccountDataSizeArgs = GetAccountDataSizeLayout::default();
            } else {
                getAccountDataSizeArgs = GetAccountDataSizeLayout::try_from_slice(rest).unwrap();
            }
        }
        22 => {
            instruction_name = String::from("InitializeImmutableOwner");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();

            initializeImmutableOwnerArgs =
                InitializeImmutableOwnerLayout::try_from_slice(rest).unwrap();
        }
        23 => {
            instruction_name = String::from("AmountToUiAmount");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            amountToUiAmountArgs = AmountToUiAmountLayout::try_from_slice(rest).unwrap();
        }
        24 => {
            instruction_name = String::from("UiAmountToAmount");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            uiAmountToAmountArgs = UiAmountToAmountLayout::try_from_slice(rest).unwrap();
        }
        25 => {
            instruction_name = String::from("InitializeMintCloseAuthority");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

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

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.payer = accounts.get(1).unwrap().to_string();
            instruction_accounts.fund_relocation_sys_program = accounts.get(2).unwrap().to_string();
            instruction_accounts.owner = accounts.get(3).unwrap().to_string();
            if accounts.len() > 4 {
                instruction_accounts.signer_accounts = accounts.split_at(4).1.to_vec();
            }
        }
        30 => {
            instruction_name = String::from("MemoTransferExtension");
        }
        31 => {
            instruction_name = String::from("CreateNativeMint");

            instruction_accounts.funding_account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.mint_funding_sys_program = accounts.get(2).unwrap().to_string();
        }
        32 => {
            instruction_name = String::from("InitializeNonTransferableMint");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
        }
        33 => {
            instruction_name = String::from("InterestBearingMintExtension");
        }
        _ => {}
    }

    let result: Instruction = Instruction {
        joinKey: "".to_string(),
        name: instruction_name,
        instruction_accounts: instruction_accounts,
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
