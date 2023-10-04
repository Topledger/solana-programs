extern crate bs58;

use borsh::{maybestd, BorshDeserialize, BorshSerialize};
use maybestd::io::{Read, Result};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct PubKeyLayout {
    pub value: [u8; 32],
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct CreateAccountLayout {
    pub lamports: u64,
    pub space: u64,
    pub owner: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct AssignLayout {
    pub owner: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct TransferLayout {
    pub lamports: u64,
}

#[derive(BorshSerialize, Debug, Clone, Default)]
pub struct SeedLayout {
    pub value: Vec<u8>,
}

impl BorshDeserialize for SeedLayout {
    fn deserialize_reader<R: Read>(reader: &mut R) -> Result<Self> {
        let mut buffer = [0 as u8; 8];
        let _ = reader.read(&mut buffer);

        let size = usize::try_from_slice(&buffer).unwrap();

        let mut consumedValue = vec![0 as u8; size];
        let _ = reader.read(&mut consumedValue);

        return Ok(SeedLayout {
            value: consumedValue,
        });
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct CreateAccountWithSeedLayout {
    pub base: PubKeyLayout,
    pub seed: SeedLayout,
    pub lamports: u64,
    pub space: u64,
    pub owner: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct AdvanceNonceAccountLayout {}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct WithdrawNonceAccountLayout {
    pub lamports: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct InitializeNonceAccountLayout {
    pub authority: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct AuthorizeNonceAccountLayout {
    pub authority: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct AllocateLayout {
    pub space: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct AllocateWithSeedLayout {
    pub base: PubKeyLayout,
    pub seed: SeedLayout,
    pub space: u64,
    pub owner: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct AssignWithSeedLayout {
    pub base: PubKeyLayout,
    pub seed: SeedLayout,
    pub owner: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct TransferWithSeedLayout {
    pub lamports: u64,
    pub from_seed: SeedLayout,
    pub from_owner: PubKeyLayout,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
pub struct UpgradeNonceAccountLayout {}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub createAccountArg: CreateAccountLayout,
    pub assignArg: AssignLayout,
    pub transferArg: TransferLayout,
    pub createAccountWithSeedArg: CreateAccountWithSeedLayout,
    pub advanceNonceAccountArg: AdvanceNonceAccountLayout,
    pub withdrawNonceAccountArg: WithdrawNonceAccountLayout,
    pub initializeNonceAccountArg: InitializeNonceAccountLayout,
    pub authorizeNonceAccountArg: AuthorizeNonceAccountLayout,
    pub allocateArg: AllocateLayout,
    pub allocateWithSeedArg: AllocateWithSeedLayout,
    pub assignWithSeedArg: AssignWithSeedLayout,
    pub transferWithSeedArg: TransferWithSeedLayout,
    pub upgradeNonceAccountArg: UpgradeNonceAccountLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut instruction_name = String::default();

    let mut createAccountArg: CreateAccountLayout = CreateAccountLayout::default();
    let mut assignArg: AssignLayout = AssignLayout::default();
    let mut transferArg: TransferLayout = TransferLayout::default();
    let mut createAccountWithSeedArg: CreateAccountWithSeedLayout =
        CreateAccountWithSeedLayout::default();
    let mut advanceNonceAccountArg: AdvanceNonceAccountLayout =
        AdvanceNonceAccountLayout::default();
    let mut withdrawNonceAccountArg: WithdrawNonceAccountLayout =
        WithdrawNonceAccountLayout::default();
    let mut initializeNonceAccountArg: InitializeNonceAccountLayout =
        InitializeNonceAccountLayout::default();
    let mut authorizeNonceAccountArg: AuthorizeNonceAccountLayout =
        AuthorizeNonceAccountLayout::default();
    let mut allocateArg: AllocateLayout = AllocateLayout::default();
    let mut allocateWithSeedArg: AllocateWithSeedLayout = AllocateWithSeedLayout::default();
    let mut assignWithSeedArg: AssignWithSeedLayout = AssignWithSeedLayout::default();
    let mut transferWithSeedArg: TransferWithSeedLayout = TransferWithSeedLayout::default();
    let mut upgradeNonceAccountArg: UpgradeNonceAccountLayout =
        UpgradeNonceAccountLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(4);
    let discriminator: u32 = u32::try_from_slice(disc_bytes).unwrap();

    match discriminator {
        0 => {
            instruction_name = String::from("CreateAccount");
            createAccountArg = CreateAccountLayout::try_from_slice(rest).unwrap();
        }
        1 => {
            instruction_name = String::from("Assign");
            assignArg = AssignLayout::try_from_slice(rest).unwrap();
        }
        2 => {
            instruction_name = String::from("Transfer");
            transferArg = TransferLayout::try_from_slice(rest).unwrap();
        }
        3 => {
            instruction_name = String::from("CreateAccountWithSeed");
            createAccountWithSeedArg = CreateAccountWithSeedLayout::try_from_slice(rest).unwrap();
        }
        4 => {
            instruction_name = String::from("AdvanceNonceAccount");
            advanceNonceAccountArg = AdvanceNonceAccountLayout::try_from_slice(rest).unwrap();
        }
        5 => {
            instruction_name = String::from("WithdrawNonceAccount");
            withdrawNonceAccountArg = WithdrawNonceAccountLayout::try_from_slice(rest).unwrap();
        }
        6 => {
            instruction_name = String::from("InitializeNonceAccount");
            initializeNonceAccountArg = InitializeNonceAccountLayout::try_from_slice(rest).unwrap();
        }
        7 => {
            instruction_name = String::from("AuthorizeNonceAccount");
            authorizeNonceAccountArg = AuthorizeNonceAccountLayout::try_from_slice(rest).unwrap();
        }
        8 => {
            instruction_name = String::from("Allocate");
            allocateArg = AllocateLayout::try_from_slice(rest).unwrap();
        }
        9 => {
            instruction_name = String::from("AllocateWithSeed");
            allocateWithSeedArg = AllocateWithSeedLayout::try_from_slice(rest).unwrap();
        }
        10 => {
            instruction_name = String::from("AssignWithSeed");
            assignWithSeedArg = AssignWithSeedLayout::try_from_slice(rest).unwrap();
        }
        11 => {
            instruction_name = String::from("TransferWithSeed");
            transferWithSeedArg = TransferWithSeedLayout::try_from_slice(rest).unwrap();
        }
        12 => {
            instruction_name = String::from("UpgradeNonceAccount");
            upgradeNonceAccountArg = UpgradeNonceAccountLayout::try_from_slice(rest).unwrap();
        }

        _ => {}
    }

    let result: Instruction = Instruction {
        name: instruction_name,
        createAccountArg: createAccountArg,
        assignArg: assignArg,
        transferArg: transferArg,
        createAccountWithSeedArg: createAccountWithSeedArg,
        advanceNonceAccountArg: advanceNonceAccountArg,
        withdrawNonceAccountArg: withdrawNonceAccountArg,
        initializeNonceAccountArg: initializeNonceAccountArg,
        authorizeNonceAccountArg: authorizeNonceAccountArg,
        allocateArg: allocateArg,
        allocateWithSeedArg: allocateWithSeedArg,
        assignWithSeedArg: assignWithSeedArg,
        transferWithSeedArg: transferWithSeedArg,
        upgradeNonceAccountArg: upgradeNonceAccountArg,
    };

    return result;
}
