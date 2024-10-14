use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbCloseMarkerArgsV0Layout, PbCloseMarkerV0Layout, PbCompiledInstructionLayout,
    PbExecuteTransactionArgsV0Layout, PbExecuteTransactionV0Layout,
    PbInitializeLazyTransactionsArgsV0Layout, PbInitializeLazyTransactionsV0Layout,
    PbSetCanopyArgsV0Layout, PbSetCanopyV0Layout, PbUpdateLazyTransactionsArgsV0Layout,
    PbUpdateLazyTransactionsV0Layout,
};

#[derive(BorshDeserialize, Debug, Clone, Default, Copy)]
pub struct PubKeyLayout {
    pub value: [u8; 32],
}

impl PubKeyLayout {
    pub fn to_proto_struct(&self) -> String {
        let result = get_b58_string(self.value);
        result
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeLazyTransactionsArgsV0Layout {
    pub root: [u8; 32],
    pub name: String,
    pub authority: PubKeyLayout,
    pub maxDepth: u32,
}

impl InitializeLazyTransactionsArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeLazyTransactionsArgsV0Layout {
        let mut root: Vec<u32> = vec![];
        for x in self.root.iter() {
            root.push(*x as u32);
        }

        PbInitializeLazyTransactionsArgsV0Layout {
            root: root,
            name: self.name.to_string(),
            authority: self.authority.to_proto_struct(),
            max_depth: self.maxDepth,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct CompiledInstructionLayout {
    pub programIdIndex: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

impl CompiledInstructionLayout {
    pub fn to_proto_struct(&self) -> PbCompiledInstructionLayout {
        let mut accounts: Vec<u32> = vec![];
        for x in self.accounts.iter() {
            accounts.push(*x as u32);
        }

        let mut data: Vec<u32> = vec![];
        for x in self.data.iter() {
            data.push(*x as u32);
        }

        PbCompiledInstructionLayout {
            program_id_index: self.programIdIndex as u32,
            accounts: accounts,
            data: data,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct ExecuteTransactionArgsV0Layout {
    pub instructions: Vec<CompiledInstructionLayout>,
    pub signerSeeds: Vec<u8>,
    pub index: u32,
}

impl ExecuteTransactionArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbExecuteTransactionArgsV0Layout {
        let mut instructions: Vec<PbCompiledInstructionLayout> = vec![];
        for x in self.instructions.iter() {
            instructions.push(x.to_proto_struct());
        }

        let mut signer_seeds: Vec<u32> = vec![];
        for x in self.signerSeeds.iter() {
            signer_seeds.push(*x as u32);
        }

        PbExecuteTransactionArgsV0Layout {
            instructions: instructions,
            signer_seeds: signer_seeds,
            index: self.index,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct CloseMarkerArgsV0Layout {
    pub index: u32,
}

impl CloseMarkerArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbCloseMarkerArgsV0Layout {
        PbCloseMarkerArgsV0Layout { index: self.index }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct UpdateLazyTransactionsArgsV0Layout {
    pub root: Option<[u8; 32]>,
    pub authority: Option<PubKeyLayout>,
}

impl UpdateLazyTransactionsArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbUpdateLazyTransactionsArgsV0Layout {
        let mut root: Vec<u32> = vec![];
        if self.root.is_some() {
            for x in self.root.unwrap().iter() {
                root.push(*x as u32);
            }
        }

        let mut authority: Option<String> = None;
        if self.authority.is_some() {
            authority = Some(self.authority.unwrap().to_proto_struct());
        }

        PbUpdateLazyTransactionsArgsV0Layout {
            root: root,
            authority: authority,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct SetCanopyArgsV0Layout {
    pub offset: u32,
    pub bytes: Vec<u8>,
}

impl SetCanopyArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbSetCanopyArgsV0Layout {
        let mut bytes: Vec<u32> = vec![];
        for x in self.bytes.iter() {
            bytes.push(*x as u32);
        }

        PbSetCanopyArgsV0Layout {
            offset: self.offset,
            bytes: bytes,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeLazyTransactionsV0Layout {
    pub initializeLazyTransactionsV0Args: InitializeLazyTransactionsArgsV0Layout,
}

impl InitializeLazyTransactionsV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeLazyTransactionsV0Layout {
        PbInitializeLazyTransactionsV0Layout {
            initialize_lazy_transactions_v0_args: self
                .initializeLazyTransactionsV0Args
                .to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct ExecuteTransactionV0Layout {
    pub executeTransactionV0Args: ExecuteTransactionArgsV0Layout,
}

impl ExecuteTransactionV0Layout {
    pub fn to_proto_struct(&self) -> PbExecuteTransactionV0Layout {
        PbExecuteTransactionV0Layout {
            execute_transaction_v0_args: self.executeTransactionV0Args.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct CloseMarkerV0Layout {
    pub closeMarkerV0Args: CloseMarkerArgsV0Layout,
}

impl CloseMarkerV0Layout {
    pub fn to_proto_struct(&self) -> PbCloseMarkerV0Layout {
        PbCloseMarkerV0Layout {
            close_marker_v0_args: self.closeMarkerV0Args.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct UpdateLazyTransactionsV0Layout {
    pub updateLazyTransactionsV0Args: UpdateLazyTransactionsArgsV0Layout,
}

impl UpdateLazyTransactionsV0Layout {
    pub fn to_proto_struct(&self) -> PbUpdateLazyTransactionsV0Layout {
        PbUpdateLazyTransactionsV0Layout {
            update_lazy_transactions_v0_args: self.updateLazyTransactionsV0Args.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct SetCanopyV0Layout {
    pub setCanopyV0Args: SetCanopyArgsV0Layout,
}

impl SetCanopyV0Layout {
    pub fn to_proto_struct(&self) -> PbSetCanopyV0Layout {
        PbSetCanopyV0Layout {
            set_canopy_v0_args: self.setCanopyV0Args.to_proto_struct(),
        }
    }
}
