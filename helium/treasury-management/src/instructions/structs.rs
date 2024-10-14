use std::default;

use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbCurveLayout, PbFields, PbInitializeTreasuryManagementArgsV0Layout,
    PbInitializeTreasuryManagementV0Layout, PbRedeemArgsV0Layout, PbRedeemV0Layout,
    PbUpdateTreasuryManagementArgsV0Layout, PbUpdateTreasuryManagementV0Layout,
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

#[derive(BorshDeserialize, Debug)]
pub enum CurveLayout {
    ExponentialCurveV0 { k: u128 },
}

impl Default for CurveLayout {
    fn default() -> Self {
        CurveLayout::ExponentialCurveV0 { k: 0 }
    }
}

impl CurveLayout {
    pub fn to_proto_struct(&self) -> PbCurveLayout {
        let mut name = "".to_string();
        let mut fields;

        match self {
            CurveLayout::ExponentialCurveV0 { k: _k } => {
                name = "ExponentialCurveV0".to_string();
                fields = Some(PbFields { k: _k.to_string() });
            }
        }

        PbCurveLayout {
            name: name,
            fields: fields,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
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

        return result;
    }
}

#[derive(BorshDeserialize, Debug, Default)]
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

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitializeTreasuryManagementArgsV0Layout {
    pub authority: PubKeyLayout,
    pub curve: CurveLayout,
    pub freezeUnixTime: i64,
    pub windowConfig: WindowedCircuitBreakerConfigV0Layout,
}

impl InitializeTreasuryManagementArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeTreasuryManagementArgsV0Layout {
        PbInitializeTreasuryManagementArgsV0Layout {
            authority: self.authority.to_proto_struct(),
            curve: self.curve.to_proto_struct(),
            freeze_unix_time: self.freezeUnixTime,
            window_config: self.windowConfig.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitializeTreasuryManagementV0Layout {
    pub initializeTreasuryManagementV0Args: InitializeTreasuryManagementArgsV0Layout,
}

impl InitializeTreasuryManagementV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeTreasuryManagementV0Layout {
        PbInitializeTreasuryManagementV0Layout {
            initialize_treasury_management_v0_args: self
                .initializeTreasuryManagementV0Args
                .to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdateTreasuryManagementArgsV0Layout {
    pub authority: PubKeyLayout,
    pub curve: CurveLayout,
    pub freezeUnixTime: i64,
}

impl UpdateTreasuryManagementArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbUpdateTreasuryManagementArgsV0Layout {
        PbUpdateTreasuryManagementArgsV0Layout {
            authority: self.authority.to_proto_struct(),
            curve: self.curve.to_proto_struct(),
            freeze_unix_time: self.freezeUnixTime,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct RedeemArgsV0Layout {
    pub amount: u64,
    pub expectedOutputAmount: u64,
}

impl RedeemArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbRedeemArgsV0Layout {
        PbRedeemArgsV0Layout {
            amount: self.amount,
            expected_output_amount: self.expectedOutputAmount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct UpdateTreasuryManagementV0Layout {
    pub updateTreasuryManagementV0Args: UpdateTreasuryManagementArgsV0Layout,
}

impl UpdateTreasuryManagementV0Layout {
    pub fn to_proto_struct(&self) -> PbUpdateTreasuryManagementV0Layout {
        PbUpdateTreasuryManagementV0Layout {
            update_treasury_management_v0_args: self
                .updateTreasuryManagementV0Args
                .to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct RedeemV0Layout {
    pub redeemV0Args: RedeemArgsV0Layout,
}

impl RedeemV0Layout {
    pub fn to_proto_struct(&self) -> PbRedeemV0Layout {
        PbRedeemV0Layout {
            redeem_v0_args: self.redeemV0Args.to_proto_struct(),
        }
    }
}
