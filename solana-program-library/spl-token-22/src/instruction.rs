extern crate bs58;
use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Serialize, Serializer};

fn u64_to_string<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&num.to_string())
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default, Copy, Serialize)]
pub struct PubkeyLayout {
    value: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMintLayout {
    decimals: u8,
    mint_authority: PubkeyLayout,
    freeze_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMultisigLayout {
    m: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct TransferLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ApproveLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub enum AuthorityTypeLayout {
    #[default]
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
    TransferFeeConfig = 4,
    WithheldWithdraw = 5,
    CloseMint = 6,
    InterestRate = 7,
    PermanentDelegate = 8,
    ConfidentialTransferMint = 9,
    TransferHookProgramId = 10,
    ConfidentialTransferFeeConfig = 11,
    MetadataPointer = 12,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub enum ExtensionTypeLayout {
    #[default]
    Uninitialized = 0,
    TransferFeeConfig = 1,
    TransferFeeAmount = 2,
    MintCloseAuthority = 3,
    ConfidentialTransferMint = 4,
    ConfidentialTransferAccount = 5,
    DefaultAccountState = 6,
    ImmutableOwner = 7,
    MemoTransfer = 8,
    NonTransferable = 9,
    InterestBearingConfig = 10,
    CpiGuard = 11,
    PermanentDelegate = 12,
    NonTransferableAccount = 13,
    TransferHook = 14,
    TransferHookAccount = 15,
    ConfidentialTransferFeeConfig = 16,
    ConfidentialTransferFeeAmount = 17,
    MetadataPointer = 18,
    TokenMetadata = 19,
    VariableLenMintTest = 65533,
    AccountPaddingTest = 65534,
    MintPaddingTest = 65535,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub enum AccountStateLayout {
    #[default]
    Uninitialized = 0,
    Initialized = 1,
    Frozen = 2,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct SourceDecryptHandlesLayout {
    lo: [u8; 32],
    hi: [u8; 32],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct DecryptableBalanceLayout {
    nonce: [u8; 12],
    ciphertext: [u8; 24],
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct SetAuthorityLayout {
    authority_type: AuthorityTypeLayout,
    new_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct MintToLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct BurnLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct TransferCheckedLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
    decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ApproveCheckedLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
    decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct MintToCheckedLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
    decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct BurnCheckedLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
    decimals: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeAccount2Layout {
    owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeAccount3Layout {
    owner: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMultisig2Layout {
    m: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMint2Layout {
    decimals: u8,
    mint_authority: PubkeyLayout,
    freeze_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct GetAccountDataSizeLayout {
    extension_types: Vec<ExtensionTypeLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct AmountToUiAmountLayout {
    #[serde(serialize_with = "u64_to_string")]
    amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct UiAmountToAmountLayout {
    ui_amount: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializeMintCloseAuthorityLayout {
    close_authority: Option<PubkeyLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum TransferFeeExtensionLayout {
    InitializeTransferFeeConfig {
        transfer_fee_config_authority: Option<PubkeyLayout>,
        withdraw_withheld_authority: Option<PubkeyLayout>,
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    },
    TransferCheckedWithFee {
        #[serde(serialize_with = "u64_to_string")]
        amount: u64,
        decimals: u8,
        fee: u64,
    },
    WithdrawWithheldTokensFromMint,
    WithdrawWithheldTokensFromAccounts {
        num_token_accounts: u8,
    },
    HarvestWithheldTokensToMint,
    SetTransferFee {
        transfer_fee_basis_points: u16,
        maximum_fee: u64,
    },
}

impl Default for TransferFeeExtensionLayout {
    fn default() -> Self {
        TransferFeeExtensionLayout::InitializeTransferFeeConfig {
            transfer_fee_config_authority: Some(PubkeyLayout::default()),
            withdraw_withheld_authority: Some(PubkeyLayout::default()),
            transfer_fee_basis_points: 0,
            maximum_fee: 0,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum ConfidentialTransferExtensionLayout {
    InitializeMint {
        authority: PubkeyLayout,
        auto_approve_new_accounts: bool,
        auditor_elgamal_pubkey: PubkeyLayout,
    },
    UpdateMint {
        auto_approve_new_accounts: bool,
        auditor_elgamal_pubkey: PubkeyLayout,
    },
    ConfigureAccount {
        decryptable_zero_balance: DecryptableBalanceLayout,
        maximum_pending_balance_credit_counter: u64,
        proof_instruction_offset: i8,
    },
    ApproveAccount,
    EmptyAccount {
        proof_instruction_offset: i8,
    },
    Deposit {
        #[serde(serialize_with = "u64_to_string")]
        amount: u64,
        decimals: u8,
    },
    Withdraw {
        #[serde(serialize_with = "u64_to_string")]
        amount: u64,
        decimals: u8,
        new_decryptable_available_balance: DecryptableBalanceLayout,
        proof_instruction_offset: i8,
    },
    Transfer {
        new_source_decryptable_available_balance: DecryptableBalanceLayout,
        proof_instruction_offset: i8,
    },
    ApplyPendingBalance {
        expected_pending_balance_credit_counter: u64,
        new_decryptable_available_balance: DecryptableBalanceLayout,
    },
    EnableConfidentialCredits,
    DisableConfidentialCredits,
    EnableNonConfidentialCredits,
    DisableNonConfidentialCredits,
    TransferWithSplitProofs {
        new_source_decryptable_available_balance: DecryptableBalanceLayout,
        no_op_on_uninitialized_split_context_state: bool,
        close_split_context_state_on_execution: bool,
        source_decrypt_handles: SourceDecryptHandlesLayout,
    },
}

impl Default for ConfidentialTransferExtensionLayout {
    fn default() -> Self {
        ConfidentialTransferExtensionLayout::InitializeMint {
            authority: PubkeyLayout::default(),
            auto_approve_new_accounts: false,
            auditor_elgamal_pubkey: PubkeyLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum DefaultAccountStateExtensionLayout {
    Initialize { name: AccountStateLayout },
    Update { name: AccountStateLayout },
}

impl Default for DefaultAccountStateExtensionLayout {
    fn default() -> Self {
        DefaultAccountStateExtensionLayout::Initialize {
            name: AccountStateLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct ReallocateLayout {
    extension_types: Vec<ExtensionTypeLayout>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub enum MemoTransferExtensionLayout {
    #[default]
    Enable,
    Disable,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum InterestBearingMintExtensionLayout {
    Initialize {
        rate_authority: PubkeyLayout,
        rate: [u8; 2],
    },
    UpdateRate {
        rate: [u8; 2],
    },
}

impl Default for InterestBearingMintExtensionLayout {
    fn default() -> Self {
        InterestBearingMintExtensionLayout::Initialize {
            rate_authority: PubkeyLayout::default(),
            rate: [0, 0],
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub enum CpiGuardExtensionLayout {
    #[default]
    Enable,
    Disable,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InitializePermanentDelegateLayout {
    delegate: PubkeyLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum TransferHookExtensionLayout {
    Initialize {
        authority: PubkeyLayout,
        program_id: PubkeyLayout,
    },
    Update {
        program_id: PubkeyLayout,
    },
}

impl Default for TransferHookExtensionLayout {
    fn default() -> Self {
        TransferHookExtensionLayout::Initialize {
            authority: PubkeyLayout::default(),
            program_id: PubkeyLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum ConfidentialTransferFeeExtensionLayout {
    InitializeConfidentialTransferFeeConfig {
        authority: PubkeyLayout,
        withdraw_withheld_authority_elgamal_pubkey: PubkeyLayout,
    },
    WithdrawWithheldTokensFromMint {
        proof_instruction_offset: i8,
        new_decryptable_available_balance: DecryptableBalanceLayout,
    },
    WithdrawWithheldTokensFromAccounts {
        num_token_accounts: u8,
        proof_instruction_offset: i8,
        new_decryptable_available_balance: DecryptableBalanceLayout,
    },
    HarvestWithheldTokensToMint,
    EnableHarvestToMint,
    DisableHarvestToMint,
}

impl Default for ConfidentialTransferFeeExtensionLayout {
    fn default() -> Self {
        ConfidentialTransferFeeExtensionLayout::InitializeConfidentialTransferFeeConfig {
            authority: PubkeyLayout::default(),
            withdraw_withheld_authority_elgamal_pubkey: PubkeyLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum MetadataPointerExtensionLayout {
    Initialize {
        authority: PubkeyLayout,
        metadata_address: PubkeyLayout,
    },
    Update {
        metadata_address: PubkeyLayout,
    },
}

impl Default for MetadataPointerExtensionLayout {
    fn default() -> Self {
        MetadataPointerExtensionLayout::Initialize {
            authority: PubkeyLayout::default(),
            metadata_address: PubkeyLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum GroupPointerExtensionLayout {
    Initialize {
        authority: PubkeyLayout,
        group_address: PubkeyLayout,
    },
    Update {
        group_address: PubkeyLayout,
    },
}

impl Default for GroupPointerExtensionLayout {
    fn default() -> Self {
        GroupPointerExtensionLayout::Initialize {
            authority: PubkeyLayout::default(),
            group_address: PubkeyLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize)]
pub enum GroupMemberPointerExtensionLayout {
    Initialize {
        authority: PubkeyLayout,
        member_address: PubkeyLayout,
    },
    Update {
        member_address: PubkeyLayout,
    },
}

impl Default for GroupMemberPointerExtensionLayout {
    fn default() -> Self {
        GroupMemberPointerExtensionLayout::Initialize {
            authority: PubkeyLayout::default(),
            member_address: PubkeyLayout::default(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default, Serialize)]
pub struct InstructionAccounts {
    pub account: String,
    pub authority: String,
    pub context_VerifyBatchedGroupedCiphertext2HandlesValidityProof: String,
    pub context_VerifyBatchedRangeProofU128: String,
    pub context_VerifyCiphertextCommitmentEqualityProof: String,
    pub current_authority: String,
    pub delegate: String,
    pub destination: String,
    pub destination_account: String,
    pub fee_account_owner: String,
    pub fee_receiver: String,
    pub funding_account: String,
    pub lamports_destination: String,
    pub mint_associated: String,
    pub mint: String,
    pub mint_authority: String,
    pub owner: String,
    pub payer: String,
    pub rent_sysvar: String,
    pub rest_sysvar: String,
    pub signers: Vec<String>,
    pub source: String,
    pub source_account: String,
    pub sources: Vec<String>,
    pub system_program: String,
    pub sysvar: String,
    pub withdraw_from: Vec<String>,
    pub withdraw_withheld_authority: String,
    pub zk_token_proof: String,
}

#[derive(Debug, Serialize)]
pub struct Instruction {
    pub joinKey: String,
    pub name: String,
    pub instruction_accounts: InstructionAccounts,
    pub initializeMintArgs: InitializeMintLayout,
    pub initializeMultisigArgs: InitializeMultisigLayout,
    pub transferArgs: TransferLayout,
    pub approveArgs: ApproveLayout,
    pub setAuthorityArgs: SetAuthorityLayout,
    pub mintToArgs: MintToLayout,
    pub burnArgs: BurnLayout,
    pub transferCheckedArgs: TransferCheckedLayout,
    pub approveCheckedArgs: ApproveCheckedLayout,
    pub mintToCheckedArgs: MintToCheckedLayout,
    pub burnCheckedArgs: BurnCheckedLayout,
    pub initializeAccount2Args: InitializeAccount2Layout,
    pub initializeAccount3Args: InitializeAccount3Layout,
    pub initializeMultisig2Args: InitializeMultisig2Layout,
    pub initializeMint2Args: InitializeMint2Layout,
    pub getAccountDataSizeArgs: GetAccountDataSizeLayout,
    pub amountToUiAmountArgs: AmountToUiAmountLayout,
    pub uiAmountToAmountArgs: UiAmountToAmountLayout,
    pub initializeMintCloseAuthorityArgs: InitializeMintCloseAuthorityLayout,
    pub transferFeeExtensionArgs: TransferFeeExtensionLayout,
    pub confidentialTransferExtensionArgs: ConfidentialTransferExtensionLayout,
    pub defaultAccountStateExtensionArgs: DefaultAccountStateExtensionLayout,
    pub reallocateArgs: ReallocateLayout,
    pub memoTransferExtensionArgs: MemoTransferExtensionLayout,
    pub interestBearingMintExtensionArgs: InterestBearingMintExtensionLayout,
    pub cpiGuardExtensionArgs: CpiGuardExtensionLayout,
    pub initializePermanentDelegateArgs: InitializePermanentDelegateLayout,
    pub transferHookExtensionArgs: TransferHookExtensionLayout,
    pub confidentialTransferFeeExtensionArgs: ConfidentialTransferFeeExtensionLayout,
    pub metadataPointerExtensionArgs: MetadataPointerExtensionLayout,
    pub groupPointerExtensionArgs: GroupPointerExtensionLayout,
    pub groupMemberPointerExtensionArgs: GroupMemberPointerExtensionLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>, accounts: Vec<String>) -> Instruction {
    let mut instruction_name = String::default();
    let mut instruction_accounts = InstructionAccounts::default();

    let mut initializeMintArgs = InitializeMintLayout::default();
    let mut initializeMultisigArgs = InitializeMultisigLayout::default();
    let mut transferArgs = TransferLayout::default();
    let mut approveArgs = ApproveLayout::default();
    let mut setAuthorityArgs = SetAuthorityLayout::default();
    let mut mintToArgs = MintToLayout::default();
    let mut burnArgs = BurnLayout::default();
    let mut transferCheckedArgs = TransferCheckedLayout::default();
    let mut approveCheckedArgs = ApproveCheckedLayout::default();
    let mut mintToCheckedArgs = MintToCheckedLayout::default();
    let mut burnCheckedArgs = BurnCheckedLayout::default();
    let mut initializeAccount2Args = InitializeAccount2Layout::default();
    let mut initializeAccount3Args = InitializeAccount3Layout::default();
    let mut initializeMultisig2Args = InitializeMultisig2Layout::default();
    let mut initializeMint2Args = InitializeMint2Layout::default();
    let mut getAccountDataSizeArgs = GetAccountDataSizeLayout::default();
    let mut amountToUiAmountArgs = AmountToUiAmountLayout::default();
    let mut uiAmountToAmountArgs = UiAmountToAmountLayout::default();
    let mut initializeMintCloseAuthorityArgs = InitializeMintCloseAuthorityLayout::default();
    let mut transferFeeExtensionArgs = TransferFeeExtensionLayout::default();
    let mut confidentialTransferExtensionArgs = ConfidentialTransferExtensionLayout::default();
    let mut defaultAccountStateExtensionArgs = DefaultAccountStateExtensionLayout::default();
    let mut reallocateArgs = ReallocateLayout::default();
    let mut memoTransferExtensionArgs = MemoTransferExtensionLayout::default();
    let mut interestBearingMintExtensionArgs = InterestBearingMintExtensionLayout::default();
    let mut cpiGuardExtensionArgs = CpiGuardExtensionLayout::default();
    let mut initializePermanentDelegateArgs = InitializePermanentDelegateLayout::default();
    let mut transferHookExtensionArgs = TransferHookExtensionLayout::default();
    let mut confidentialTransferFeeExtensionArgs =
        ConfidentialTransferFeeExtensionLayout::default();
    let mut metadataPointerExtensionArgs = MetadataPointerExtensionLayout::default();
    let mut groupPointerExtensionArgs = GroupPointerExtensionLayout::default();
    let mut groupMemberPointerExtensionArgs = GroupMemberPointerExtensionLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    match discriminator {
        0 => {
            instruction_name = String::from("InitializeMint");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(1).unwrap().to_string();

            initializeMintArgs =
                InitializeMintLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        1 => {
            instruction_name = String::from("InitializeAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint_associated = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(3).unwrap().to_string();
        }
        2 => {
            instruction_name = String::from("InitializeMultisig");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(1).unwrap().to_string();
            if accounts.len() >= 2 {
                instruction_accounts.signers = accounts.split_at(2).1.to_vec();
            }

            initializeMultisigArgs =
                InitializeMultisigLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        3 => {
            instruction_name = String::from("Transfer");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.destination = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }

            transferArgs = TransferLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        4 => {
            instruction_name = String::from("Approve");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.delegate = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }

            approveArgs = ApproveLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        5 => {
            instruction_name = String::from("Revoke");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.owner = accounts.get(1).unwrap().to_string();
            if accounts.len() > 2 {
                instruction_accounts.signers = accounts.split_at(2).1.to_vec();
            }
        }
        6 => {
            instruction_name = String::from("SetAuthority");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.current_authority = accounts.get(1).unwrap().to_string();
            if accounts.len() > 2 {
                instruction_accounts.signers = accounts.split_at(2).1.to_vec();
            }

            setAuthorityArgs =
                SetAuthorityLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        7 => {
            instruction_name = String::from("MintTo");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.account = accounts.get(1).unwrap().to_string();
            instruction_accounts.mint_authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }

            mintToArgs = MintToLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        8 => {
            instruction_name = String::from("Burn");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }

            burnArgs = BurnLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        9 => {
            instruction_name = String::from("CloseAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.destination = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }
        }
        10 => {
            instruction_name = String::from("FreezeAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }
        }
        11 => {
            instruction_name = String::from("ThawAccount");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }
        }
        12 => {
            instruction_name = String::from("TransferChecked");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.destination = accounts.get(2).unwrap().to_string();
            instruction_accounts.owner = accounts.get(3).unwrap().to_string();
            if accounts.len() > 4 {
                instruction_accounts.signers = accounts.split_at(4).1.to_vec();
            }

            transferCheckedArgs =
                TransferCheckedLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        13 => {
            instruction_name = String::from("ApproveChecked");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.delegate = accounts.get(2).unwrap().to_string();
            instruction_accounts.owner = accounts.get(3).unwrap().to_string();
            if accounts.len() > 4 {
                instruction_accounts.signers = accounts.split_at(4).1.to_vec();
            }

            approveCheckedArgs =
                ApproveCheckedLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        14 => {
            instruction_name = String::from("MintToChecked");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.account = accounts.get(1).unwrap().to_string();
            instruction_accounts.mint_authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }

            mintToCheckedArgs =
                MintToCheckedLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        15 => {
            instruction_name = String::from("BurnChecked");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.owner = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }

            burnCheckedArgs = BurnCheckedLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        16 => {
            instruction_name = String::from("InitializeAccount2");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.rent_sysvar = accounts.get(2).unwrap().to_string();

            initializeAccount2Args =
                InitializeAccount2Layout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        17 => {
            instruction_name = String::from("SyncNative");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
        }
        18 => {
            instruction_name = String::from("InitializeAccount3");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();

            initializeAccount3Args =
                InitializeAccount3Layout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        19 => {
            instruction_name = String::from("InitializeMultisig2");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
            if accounts.len() > 1 {
                instruction_accounts.signers = accounts.split_at(1).1.to_vec();
            }

            initializeMultisig2Args =
                InitializeMultisig2Layout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        20 => {
            instruction_name = String::from("InitializeMint2");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            initializeMint2Args =
                InitializeMint2Layout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        21 => {
            instruction_name = String::from("GetAccountDataSize");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            getAccountDataSizeArgs =
                GetAccountDataSizeLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        22 => {
            instruction_name = String::from("InitializeImmutableOwner");

            instruction_accounts.account = accounts.get(0).unwrap().to_string();
        }
        23 => {
            instruction_name = String::from("AmountToUiAmount");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            amountToUiAmountArgs =
                AmountToUiAmountLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        24 => {
            instruction_name = String::from("UiAmountToAmount");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            uiAmountToAmountArgs =
                UiAmountToAmountLayout::deserialize(&mut rest.clone()).unwrap_or_default();
        }
        25 => {
            instruction_name = String::from("InitializeMintCloseAuthority");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();

            initializeMintCloseAuthorityArgs =
                InitializeMintCloseAuthorityLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();
        }
        26 => {
            instruction_name = String::from("TransferFeeExtension");
            transferFeeExtensionArgs =
                TransferFeeExtensionLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            match transferFeeExtensionArgs {
                TransferFeeExtensionLayout::InitializeTransferFeeConfig { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                TransferFeeExtensionLayout::TransferCheckedWithFee { .. } => {
                    instruction_accounts.source = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.destination = accounts.get(2).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(3).unwrap().to_string();
                    if accounts.len() > 4 {
                        instruction_accounts.signers = accounts.split_at(4).1.to_vec();
                    }
                }
                TransferFeeExtensionLayout::WithdrawWithheldTokensFromMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.fee_receiver = accounts.get(1).unwrap().to_string();
                    instruction_accounts.withdraw_withheld_authority =
                        accounts.get(2).unwrap().to_string();
                    if accounts.len() > 3 {
                        instruction_accounts.signers = accounts.split_at(3).1.to_vec();
                    }
                }
                TransferFeeExtensionLayout::WithdrawWithheldTokensFromAccounts { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.fee_receiver = accounts.get(1).unwrap().to_string();
                    instruction_accounts.withdraw_withheld_authority =
                        accounts.get(2).unwrap().to_string();
                    if accounts.len() > 3 {
                        instruction_accounts.sources = accounts.split_at(3).1.to_vec();
                    }
                }
                TransferFeeExtensionLayout::HarvestWithheldTokensToMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    if accounts.len() > 1 {
                        instruction_accounts.sources = accounts.split_at(1).1.to_vec();
                    }
                }
                TransferFeeExtensionLayout::SetTransferFee { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.fee_account_owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        27 => {
            instruction_name = String::from("ConfidentialTransferExtension");

            confidentialTransferExtensionArgs =
                ConfidentialTransferExtensionLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();

            match confidentialTransferExtensionArgs {
                ConfidentialTransferExtensionLayout::InitializeMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                ConfidentialTransferExtensionLayout::UpdateMint { .. } => {
                    instruction_accounts.source = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                }
                ConfidentialTransferExtensionLayout::ConfigureAccount { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.sysvar = accounts.get(2).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(3).unwrap().to_string();
                    if accounts.len() > 4 {
                        instruction_accounts.signers = accounts.split_at(4).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::ApproveAccount { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(2).unwrap().to_string();
                }
                ConfidentialTransferExtensionLayout::EmptyAccount { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.sysvar = accounts.get(1).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(2).unwrap().to_string();
                    if accounts.len() > 3 {
                        instruction_accounts.signers = accounts.split_at(3).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::Deposit { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(2).unwrap().to_string();
                    if accounts.len() > 3 {
                        instruction_accounts.signers = accounts.split_at(3).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::Withdraw { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.sysvar = accounts.get(2).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(3).unwrap().to_string();
                    if accounts.len() > 4 {
                        instruction_accounts.signers = accounts.split_at(4).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::Transfer { .. } => {
                    instruction_accounts.source_account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.destination_account = accounts.get(2).unwrap().to_string();
                    instruction_accounts.sysvar = accounts.get(3).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(4).unwrap().to_string();
                    if accounts.len() > 5 {
                        instruction_accounts.signers = accounts.split_at(5).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::ApplyPendingBalance { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::EnableConfidentialCredits { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::DisableConfidentialCredits { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::EnableNonConfidentialCredits { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::DisableNonConfidentialCredits { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                ConfidentialTransferExtensionLayout::TransferWithSplitProofs { .. } => {
                    instruction_accounts.source = accounts.get(0).unwrap().to_string();
                    instruction_accounts.mint = accounts.get(1).unwrap().to_string();
                    instruction_accounts.destination = accounts.get(2).unwrap().to_string();

                    instruction_accounts.context_VerifyCiphertextCommitmentEqualityProof =
                        accounts.get(3).unwrap().to_string();
                    instruction_accounts
                        .context_VerifyBatchedGroupedCiphertext2HandlesValidityProof =
                        accounts.get(4).unwrap().to_string();
                    instruction_accounts.context_VerifyBatchedRangeProofU128 =
                        accounts.get(5).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(6).unwrap().to_string();
                    instruction_accounts.lamports_destination =
                        accounts.get(7).unwrap().to_string();
                    instruction_accounts.zk_token_proof = accounts.get(8).unwrap().to_string();
                }
            }
        }
        28 => {
            instruction_name = String::from("DefaultAccountStateExtension");
            defaultAccountStateExtensionArgs =
                DefaultAccountStateExtensionLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();

            match defaultAccountStateExtensionArgs {
                DefaultAccountStateExtensionLayout::Initialize { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                DefaultAccountStateExtensionLayout::Update { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        29 => {
            instruction_name = String::from("Reallocate");
            reallocateArgs = ReallocateLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
            instruction_accounts.payer = accounts.get(1).unwrap().to_string();
            instruction_accounts.system_program = accounts.get(2).unwrap().to_string();
            instruction_accounts.owner = accounts.get(3).unwrap().to_string();
            if accounts.len() > 4 {
                instruction_accounts.signers = accounts.split_at(4).1.to_vec();
            }
        }
        30 => {
            instruction_name = String::from("MemoTransferExtension");
            memoTransferExtensionArgs =
                MemoTransferExtensionLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            match memoTransferExtensionArgs {
                MemoTransferExtensionLayout::Enable { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                MemoTransferExtensionLayout::Disable { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        31 => {
            instruction_name = String::from("CreateNativeMint");

            instruction_accounts.funding_account = accounts.get(0).unwrap().to_string();
            instruction_accounts.mint = accounts.get(1).unwrap().to_string();
            instruction_accounts.system_program = accounts.get(2).unwrap().to_string();
        }
        32 => {
            instruction_name = String::from("InitializeNonTransferableMint");

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
        }
        33 => {
            instruction_name = String::from("InterestBearingMintExtension");
            interestBearingMintExtensionArgs =
                InterestBearingMintExtensionLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();

            match interestBearingMintExtensionArgs {
                InterestBearingMintExtensionLayout::Initialize { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                InterestBearingMintExtensionLayout::UpdateRate { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        34 => {
            instruction_name = String::from("CpiGuardExtension");
            cpiGuardExtensionArgs =
                CpiGuardExtensionLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            match cpiGuardExtensionArgs {
                CpiGuardExtensionLayout::Enable { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                CpiGuardExtensionLayout::Disable { .. } => {
                    instruction_accounts.account = accounts.get(0).unwrap().to_string();
                    instruction_accounts.owner = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        35 => {
            instruction_name = String::from("InitializePermanentDelegate");
            initializePermanentDelegateArgs =
                InitializePermanentDelegateLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();

            instruction_accounts.mint = accounts.get(0).unwrap().to_string();
        }
        36 => {
            instruction_name = String::from("TransferHookExtension");
            transferHookExtensionArgs =
                TransferHookExtensionLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            match transferHookExtensionArgs {
                TransferHookExtensionLayout::Initialize { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                TransferHookExtensionLayout::Update { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        37 => {
            instruction_name = String::from("ConfidentialTransferFeeExtension");
            confidentialTransferFeeExtensionArgs =
                ConfidentialTransferFeeExtensionLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();

            match confidentialTransferFeeExtensionArgs {
                ConfidentialTransferFeeExtensionLayout::InitializeConfidentialTransferFeeConfig { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                ConfidentialTransferFeeExtensionLayout::WithdrawWithheldTokensFromMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.fee_receiver = accounts.get(1).unwrap().to_string();
                    instruction_accounts.sysvar = accounts.get(2).unwrap().to_string();
                    instruction_accounts.withdraw_withheld_authority = accounts.get(3).unwrap().to_string();
                    if accounts.len() > 4 {
                        instruction_accounts.signers = accounts.split_at(4).1.to_vec();
                    }
                }
                ConfidentialTransferFeeExtensionLayout::WithdrawWithheldTokensFromAccounts { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.fee_receiver = accounts.get(1).unwrap().to_string();
                    instruction_accounts.sysvar = accounts.get(2).unwrap().to_string();
                    instruction_accounts.withdraw_withheld_authority = accounts.get(3).unwrap().to_string();
                    if accounts.len() > 4 {
                        instruction_accounts.withdraw_from = accounts.split_at(4).1.to_vec();
                    }
                }
                ConfidentialTransferFeeExtensionLayout::HarvestWithheldTokensToMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    if accounts.len() > 1 {
                        instruction_accounts.sources = accounts.split_at(1).1.to_vec();
                    }
                }
                ConfidentialTransferFeeExtensionLayout::EnableHarvestToMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
                ConfidentialTransferFeeExtensionLayout::DisableHarvestToMint { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        38 => {
            instruction_name = String::from("WithdrawExcessLamports");

            instruction_accounts.source = accounts.get(0).unwrap().to_string();
            instruction_accounts.destination = accounts.get(1).unwrap().to_string();
            instruction_accounts.authority = accounts.get(2).unwrap().to_string();
            if accounts.len() > 3 {
                instruction_accounts.signers = accounts.split_at(3).1.to_vec();
            }
        }
        39 => {
            instruction_name = String::from("MetadataPointerExtension");
            metadataPointerExtensionArgs =
                MetadataPointerExtensionLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            match metadataPointerExtensionArgs {
                MetadataPointerExtensionLayout::Initialize { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                MetadataPointerExtensionLayout::Update { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        40 => {
            instruction_name = String::from("GroupPointerExtension");
            groupPointerExtensionArgs =
                GroupPointerExtensionLayout::deserialize(&mut rest.clone()).unwrap_or_default();

            match groupPointerExtensionArgs {
                GroupPointerExtensionLayout::Initialize { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                GroupPointerExtensionLayout::Update { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        41 => {
            instruction_name = String::from("GroupMemberPointerExtension");
            groupMemberPointerExtensionArgs =
                GroupMemberPointerExtensionLayout::deserialize(&mut rest.clone())
                    .unwrap_or_default();
            match groupMemberPointerExtensionArgs {
                GroupMemberPointerExtensionLayout::Initialize { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                }
                GroupMemberPointerExtensionLayout::Update { .. } => {
                    instruction_accounts.mint = accounts.get(0).unwrap().to_string();
                    instruction_accounts.authority = accounts.get(1).unwrap().to_string();
                    if accounts.len() > 2 {
                        instruction_accounts.signers = accounts.split_at(2).1.to_vec();
                    }
                }
            }
        }
        _ => {}
    }

    let result: Instruction = Instruction {
        joinKey: "".to_string(),
        name: instruction_name,
        instruction_accounts: instruction_accounts,
        initializeMintArgs: initializeMintArgs,
        initializeMultisigArgs: initializeMultisigArgs,
        transferArgs: transferArgs,
        approveArgs: approveArgs,
        setAuthorityArgs: setAuthorityArgs,
        mintToArgs: mintToArgs,
        burnArgs: burnArgs,
        transferCheckedArgs: transferCheckedArgs,
        approveCheckedArgs: approveCheckedArgs,
        mintToCheckedArgs: mintToCheckedArgs,
        burnCheckedArgs: burnCheckedArgs,
        initializeAccount2Args: initializeAccount2Args,
        initializeAccount3Args: initializeAccount3Args,
        initializeMultisig2Args: initializeMultisig2Args,
        initializeMint2Args: initializeMint2Args,
        getAccountDataSizeArgs: getAccountDataSizeArgs,
        amountToUiAmountArgs: amountToUiAmountArgs,
        uiAmountToAmountArgs: uiAmountToAmountArgs,
        initializeMintCloseAuthorityArgs: initializeMintCloseAuthorityArgs,
        transferFeeExtensionArgs: transferFeeExtensionArgs,
        confidentialTransferExtensionArgs: confidentialTransferExtensionArgs,
        defaultAccountStateExtensionArgs: defaultAccountStateExtensionArgs,
        reallocateArgs: reallocateArgs,
        memoTransferExtensionArgs: memoTransferExtensionArgs,
        interestBearingMintExtensionArgs: interestBearingMintExtensionArgs,
        cpiGuardExtensionArgs: cpiGuardExtensionArgs,
        initializePermanentDelegateArgs: initializePermanentDelegateArgs,
        transferHookExtensionArgs: transferHookExtensionArgs,
        confidentialTransferFeeExtensionArgs: confidentialTransferFeeExtensionArgs,
        metadataPointerExtensionArgs: metadataPointerExtensionArgs,
        groupPointerExtensionArgs: groupPointerExtensionArgs,
        groupMemberPointerExtensionArgs: groupMemberPointerExtensionArgs,
    };

    return result;
}
