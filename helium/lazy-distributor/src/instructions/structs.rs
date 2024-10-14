use std::{default, vec};

use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbDistributeCompressionRewardsArgsV0Layout, PbDistributeCompressionRewardsV0Layout,
    PbInitializeCompressionRecipientArgsV0Layout, PbInitializeCompressionRecipientV0Layout,
    PbInitializeLazyDistributorArgsV0Layout, PbInitializeLazyDistributorV0Layout,
    PbOracleConfigV0Layout, PbSetCurrentRewardsArgsV0Layout, PbSetCurrentRewardsV0Layout,
    PbUpdateLazyDistributorArgsV0Layout, PbUpdateLazyDistributorV0Layout,
    PbWindowedCircuitBreakerConfigV0Layout,
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
pub enum ThresholdTypeLayout {
    #[default]
    Percent = 0,
    Absolute = 1,
}

impl ThresholdTypeLayout {
    pub fn to_proto_struct(&self) -> String {
        let mut result = "".to_string();
        match self {
            ThresholdTypeLayout::Percent => result = "Percent".to_string(),
            ThresholdTypeLayout::Absolute => result = "Absolute".to_string(),
        }
        result
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct OracleConfigV0Layout {
    pub oracle: PubKeyLayout,
    pub url: String,
}

impl OracleConfigV0Layout {
    pub fn to_proto_struct(&self) -> PbOracleConfigV0Layout {
        PbOracleConfigV0Layout {
            oracle: self.oracle.to_proto_struct(),
            url: self.url.to_string(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct WindowedCircuitBreakerConfigV0Layout {
    pub windowSizeSeconds: u64,
    pub thresholdType: ThresholdTypeLayout,
    pub threshold: u64,
}

impl WindowedCircuitBreakerConfigV0Layout {
    pub fn to_proto_struct(&self) -> PbWindowedCircuitBreakerConfigV0Layout {
        PbWindowedCircuitBreakerConfigV0Layout {
            window_size_seconds: self.windowSizeSeconds,
            threshold_type: self.thresholdType.to_proto_struct(),
            threshold: self.threshold,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeLazyDistributorArgsV0Layout {
    pub oracles: Vec<OracleConfigV0Layout>,
    pub authority: PubKeyLayout,
    pub windowConfig: WindowedCircuitBreakerConfigV0Layout,
    pub approver: Option<PubKeyLayout>,
}

impl InitializeLazyDistributorArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeLazyDistributorArgsV0Layout {
        let mut oracles: Vec<PbOracleConfigV0Layout> = vec![];
        for x in self.oracles.iter() {
            oracles.push(x.to_proto_struct());
        }

        let mut approver: Option<String> = None;
        if self.approver.is_some() {
            approver = Some(self.approver.unwrap().to_proto_struct());
        }

        PbInitializeLazyDistributorArgsV0Layout {
            oracles: oracles,
            authority: self.authority.to_proto_struct(),
            window_config: self.windowConfig.to_proto_struct(),
            approver: approver,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeCompressionRecipientArgsV0Layout {
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub root: [u8; 32],
    pub index: u32,
}

impl InitializeCompressionRecipientArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeCompressionRecipientArgsV0Layout {
        let mut data_hash: Vec<u32> = vec![];
        for x in self.dataHash.iter() {
            data_hash.push(*x as u32);
        }

        let mut creator_hash: Vec<u32> = vec![];
        for x in self.creatorHash.iter() {
            creator_hash.push(*x as u32);
        }

        let mut root: Vec<u32> = vec![];
        for x in self.root.iter() {
            root.push(*x as u32);
        }

        PbInitializeCompressionRecipientArgsV0Layout {
            data_hash: data_hash,
            creator_hash: creator_hash,
            root: root,
            index: self.index,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct SetCurrentRewardsArgsV0Layout {
    pub oracleIndex: u16,
    pub currentRewards: u64,
}

impl SetCurrentRewardsArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbSetCurrentRewardsArgsV0Layout {
        PbSetCurrentRewardsArgsV0Layout {
            oracle_index: self.oracleIndex as u32,
            current_rewards: self.currentRewards,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct DistributeCompressionRewardsArgsV0Layout {
    pub dataHash: [u8; 32],
    pub creatorHash: [u8; 32],
    pub root: [u8; 32],
    pub index: u32,
}

impl DistributeCompressionRewardsArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbDistributeCompressionRewardsArgsV0Layout {
        let mut data_hash: Vec<u32> = vec![];
        for x in self.dataHash.iter() {
            data_hash.push(*x as u32);
        }

        let mut creator_hash: Vec<u32> = vec![];
        for x in self.creatorHash.iter() {
            creator_hash.push(*x as u32);
        }

        let mut root: Vec<u32> = vec![];
        for x in self.root.iter() {
            root.push(*x as u32);
        }

        PbDistributeCompressionRewardsArgsV0Layout {
            data_hash: data_hash,
            creator_hash: creator_hash,
            root: root,
            index: self.index,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct UpdateLazyDistributorArgsV0Layout {
    pub oracles: Vec<OracleConfigV0Layout>,
    pub authority: Option<PubKeyLayout>,
    pub approver: Option<PubKeyLayout>,
}

impl UpdateLazyDistributorArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbUpdateLazyDistributorArgsV0Layout {
        let mut oracles: Vec<PbOracleConfigV0Layout> = vec![];
        for x in self.oracles.iter() {
            oracles.push(x.to_proto_struct());
        }

        let mut authority: Option<String> = None;
        if self.authority.is_some() {
            authority = Some(self.authority.unwrap().to_proto_struct());
        }

        let mut approver: Option<String> = None;
        if self.approver.is_some() {
            approver = Some(self.approver.unwrap().to_proto_struct());
        }

        PbUpdateLazyDistributorArgsV0Layout {
            oracles: oracles,
            authority: authority,
            approver: approver,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeLazyDistributorV0Layout {
    pub initializeLazyDistributorV0Args: InitializeLazyDistributorArgsV0Layout,
}

impl InitializeLazyDistributorV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeLazyDistributorV0Layout {
        PbInitializeLazyDistributorV0Layout {
            initialize_lazy_distributor_v0_args: self
                .initializeLazyDistributorV0Args
                .to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeCompressionRecipientV0Layout {
    pub initializeCompressionRecipientV0Args: InitializeCompressionRecipientArgsV0Layout,
}

impl InitializeCompressionRecipientV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeCompressionRecipientV0Layout {
        PbInitializeCompressionRecipientV0Layout {
            initialize_compression_recipient_v0_args: self
                .initializeCompressionRecipientV0Args
                .to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct SetCurrentRewardsV0Layout {
    pub setCurrentRewardsV0Args: SetCurrentRewardsArgsV0Layout,
}

impl SetCurrentRewardsV0Layout {
    pub fn to_proto_struct(&self) -> PbSetCurrentRewardsV0Layout {
        PbSetCurrentRewardsV0Layout {
            set_current_rewards_v0_args: self.setCurrentRewardsV0Args.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct DistributeCompressionRewardsV0Layout {
    pub distributeCompressionRewardsV0Args: DistributeCompressionRewardsArgsV0Layout,
}

impl DistributeCompressionRewardsV0Layout {
    pub fn to_proto_struct(&self) -> PbDistributeCompressionRewardsV0Layout {
        PbDistributeCompressionRewardsV0Layout {
            distribute_compression_rewards_v0_args: self
                .distributeCompressionRewardsV0Args
                .to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct UpdateLazyDistributorV0Layout {
    pub updateLazyDistributorV0Args: UpdateLazyDistributorArgsV0Layout,
}

impl UpdateLazyDistributorV0Layout {
    pub fn to_proto_struct(&self) -> PbUpdateLazyDistributorV0Layout {
        PbUpdateLazyDistributorV0Layout {
            update_lazy_distributor_v0_args: self.updateLazyDistributorV0Args.to_proto_struct(),
        }
    }
}
