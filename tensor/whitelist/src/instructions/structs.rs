use std::vec;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::pb::sf::solana::block_meta::v1::{
    PbArray, PbInitUpdateAuthorityLayout, PbInitUpdateMintProofLayout, PbInitUpdateWhitelistLayout,
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
pub struct InitUpdateAuthorityLayout {
    pub newCosigner: Option<PubKeyLayout>,
    pub newOwner: Option<PubKeyLayout>,
}

impl InitUpdateAuthorityLayout {
    pub fn to_proto_struct(&self) -> PbInitUpdateAuthorityLayout {
        let mut new_cosigner: Option<String> = None;
        if self.newCosigner.is_some() {
            new_cosigner = Some(self.newCosigner.unwrap().to_proto_struct());
        }

        let mut new_owner: Option<String> = None;
        if self.newOwner.is_some() {
            new_owner = Some(self.newOwner.unwrap().to_proto_struct());
        }

        PbInitUpdateAuthorityLayout {
            new_cosigner: new_cosigner,
            new_owner: new_owner,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitUpdateWhitelistLayout {
    pub uuid: [u8; 32],
    pub rootHash: Option<[u8; 32]>,
    pub name: Option<[u8; 32]>,
    pub voc: Option<PubKeyLayout>,
    pub fvc: Option<PubKeyLayout>,
}

impl InitUpdateWhitelistLayout {
    pub fn to_proto_struct(&self) -> PbInitUpdateWhitelistLayout {
        let mut uuid: Vec<u32> = vec![];
        for x in self.uuid.iter() {
            uuid.push(*x as u32);
        }

        let mut root_hash: Vec<u32> = vec![];
        if self.rootHash.is_some() {
            for x in self.rootHash.unwrap().iter() {
                root_hash.push(*x as u32);
            }
        }

        let mut name: Vec<u32> = vec![];
        if self.name.is_some() {
            for x in self.name.unwrap().iter() {
                name.push(*x as u32);
            }
        }

        let mut voc: Option<String> = None;
        if self.voc.is_some() {
            voc = Some(self.voc.unwrap().to_proto_struct());
        }

        let mut fvc: Option<String> = None;
        if self.fvc.is_some() {
            fvc = Some(self.fvc.unwrap().to_proto_struct());
        }

        PbInitUpdateWhitelistLayout {
            uuid: uuid,
            root_hash: root_hash,
            name: name,
            voc: voc,
            fvc: fvc,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct Array {
    pub value: [u8; 32],
}

impl Array {
    pub fn to_proto_struct(&self) -> PbArray {
        let mut value: Vec<u32> = vec![];
        for x in self.value.iter() {
            value.push(*x as u32);
        }
        PbArray { value: value }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitUpdateMintProofLayout {
    pub proof: Vec<Array>,
}

impl InitUpdateMintProofLayout {
    pub fn to_proto_struct(&self) -> PbInitUpdateMintProofLayout {
        let mut proof: Vec<PbArray> = vec![];
        for x in self.proof.iter() {
            proof.push(x.to_proto_struct());
        }

        PbInitUpdateMintProofLayout { proof: proof }
    }
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
