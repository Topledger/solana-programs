use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbConfigSettingsV0Layout, PbInitializeRewardableEntityConfigArgsV0Layout,
    PbInitializeRewardableEntityConfigV0Layout, PbIotConfigFields, PbMobileConfigFields,
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
pub enum ConfigSettingsV0Layout {
    IotConfig {
        min_gain: i32,
        max_gain: i32,
        full_location_staking_fee: u64,
        dataonly_location_staking_fee: u64,
    },
    MobileConfig {
        full_location_staking_fee: u64,
        dataonly_location_staking_fee: u64,
    },
}

impl Default for ConfigSettingsV0Layout {
    fn default() -> Self {
        ConfigSettingsV0Layout::IotConfig {
            min_gain: 0,
            max_gain: 0,
            full_location_staking_fee: 0,
            dataonly_location_staking_fee: 0,
        }
    }
}

impl ConfigSettingsV0Layout {
    pub fn to_proto_struct(&self) -> PbConfigSettingsV0Layout {
        let mut name: String = "".to_string();
        let mut iot_config_fields: Option<PbIotConfigFields> = None;
        let mut mobile_config_fields: Option<PbMobileConfigFields> = None;

        match self {
            ConfigSettingsV0Layout::IotConfig {
                min_gain: _min_gain,
                max_gain: _max_gain,
                full_location_staking_fee: _full_location_staking_fee,
                dataonly_location_staking_fee: _dataonly_location_staking_fee,
            } => {
                name = "IotConfig".to_string();
                iot_config_fields = Some(PbIotConfigFields {
                    min_gain: *_min_gain,
                    max_gain: *_max_gain,
                    full_location_staking_fee: *_full_location_staking_fee,
                    dataonly_location_staking_fee: *_dataonly_location_staking_fee,
                });
            }
            ConfigSettingsV0Layout::MobileConfig {
                full_location_staking_fee: _full_location_staking_fee,
                dataonly_location_staking_fee: _dataonly_location_staking_fee,
            } => {
                name = "MobileConfig".to_string();
                mobile_config_fields = Some(PbMobileConfigFields {
                    full_location_staking_fee: *_full_location_staking_fee,
                    dataonly_location_staking_fee: *_dataonly_location_staking_fee,
                });
            }
        }

        PbConfigSettingsV0Layout {
            name: name,
            iot_config_fields: iot_config_fields,
            mobile_config_fields: mobile_config_fields,
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeRewardableEntityConfigArgsV0Layout {
    pub symbol: String,
    pub settings: ConfigSettingsV0Layout,
}

impl InitializeRewardableEntityConfigArgsV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeRewardableEntityConfigArgsV0Layout {
        PbInitializeRewardableEntityConfigArgsV0Layout {
            symbol: self.symbol.to_string(),
            settings: self.settings.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Default, Debug)]
pub struct InitializeRewardableEntityConfigV0Layout {
    pub args: InitializeRewardableEntityConfigArgsV0Layout,
}

impl InitializeRewardableEntityConfigV0Layout {
    pub fn to_proto_struct(&self) -> PbInitializeRewardableEntityConfigV0Layout {
        PbInitializeRewardableEntityConfigV0Layout {
            args: self.args.to_proto_struct(),
        }
    }
}
