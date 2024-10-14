use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbInitRootLayout, PbSetAdminLayout, PbSetAlternateStakerLayout, PbSetOperatorLayout,
    PbSplitLayout,
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
pub struct InitRootLayout {
    pub admin: PubKeyLayout,
    pub operator: PubKeyLayout,
    pub alternateStaker: PubKeyLayout,
}

impl InitRootLayout {
    pub fn to_proto_struct(&self) -> PbInitRootLayout {
        PbInitRootLayout {
            admin: self.admin.to_proto_struct(),
            operator: self.operator.to_proto_struct(),
            alternate_staker: self.alternateStaker.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetOperatorLayout {
    pub newOperator: PubKeyLayout,
}

impl SetOperatorLayout {
    pub fn to_proto_struct(&self) -> PbSetOperatorLayout {
        PbSetOperatorLayout {
            new_operator: self.newOperator.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetAdminLayout {
    pub newAdmin: PubKeyLayout,
}

impl SetAdminLayout {
    pub fn to_proto_struct(&self) -> PbSetAdminLayout {
        PbSetAdminLayout {
            new_admin: self.newAdmin.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetAlternateStakerLayout {
    pub newAlternateStaker: PubKeyLayout,
}

impl SetAlternateStakerLayout {
    pub fn to_proto_struct(&self) -> PbSetAlternateStakerLayout {
        PbSetAlternateStakerLayout {
            new_alternate_staker: self.newAlternateStaker.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SplitLayout {
    pub amount: u64,
}

impl SplitLayout {
    pub fn to_proto_struct(&self) -> PbSplitLayout {
        PbSplitLayout {
            amount: self.amount,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
