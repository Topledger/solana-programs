use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbCreateLayout, PbDeleteLayout, PbExtendLayout, PbImmutableOwnerLayout, PbResizeLayout,
    PbTransferLayout, PbUpdateLayout,
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
pub struct CreateLayout {
    pub hashedName: Vec<u8>,
    pub space: u32,
    pub expiresAt: Option<u64>,
}

impl CreateLayout {
    pub fn to_proto_struct(&self) -> PbCreateLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        let mut expires_at: Option<u64> = None;
        if self.expiresAt.is_some() {
            expires_at = Some(self.expiresAt.unwrap_or_default());
        }

        PbCreateLayout {
            hashed_name: hashed_name,
            space: self.space,
            expires_at: expires_at,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdateLayout {
    pub hashedName: Vec<u8>,
    pub offset: u32,
    pub data: Vec<u8>,
}

impl UpdateLayout {
    pub fn to_proto_struct(&self) -> PbUpdateLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        let mut data: Vec<u64> = vec![];
        for x in self.data.as_slice().iter() {
            data.push(*x as u64);
        }

        PbUpdateLayout {
            hashed_name: hashed_name,
            offset: self.offset,
            data: data,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct TransferLayout {
    pub hashedName: Vec<u8>,
    pub nameAccountBump: u8,
    pub newOwner: PubKeyLayout,
}

impl TransferLayout {
    pub fn to_proto_struct(&self) -> PbTransferLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        PbTransferLayout {
            hashed_name: hashed_name,
            name_account_bump: self.nameAccountBump as u32,
            new_owner: self.newOwner.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct DeleteLayout {
    pub hashedName: Vec<u8>,
    pub nameAccountBump: u8,
}

impl DeleteLayout {
    pub fn to_proto_struct(&self) -> PbDeleteLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        PbDeleteLayout {
            hashed_name: hashed_name,
            name_account_bump: self.nameAccountBump as u32,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ResizeLayout {
    pub hashedName: Vec<u8>,
    pub newSize: u32,
    pub nameAccountBump: u8,
}

impl ResizeLayout {
    pub fn to_proto_struct(&self) -> PbResizeLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        PbResizeLayout {
            hashed_name,
            new_size: self.newSize,
            name_account_bump: self.nameAccountBump as u32,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ExtendLayout {
    pub hashedName: Vec<u8>,
    pub expiresAt: u64,
}

impl ExtendLayout {
    pub fn to_proto_struct(&self) -> PbExtendLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        PbExtendLayout {
            hashed_name: hashed_name,
            expires_at: self.expiresAt,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ImmutableOwnerLayout {
    pub hashedName: Vec<u8>,
}

impl ImmutableOwnerLayout {
    pub fn to_proto_struct(&self) -> PbImmutableOwnerLayout {
        let mut hashed_name: Vec<u64> = vec![];
        for x in self.hashedName.as_slice().iter() {
            hashed_name.push(*x as u64);
        }

        PbImmutableOwnerLayout {
            hashed_name: hashed_name,
        }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
