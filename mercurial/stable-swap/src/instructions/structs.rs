use borsh::BorshDeserialize;

use crate::pb::sf::solana::block_meta::v1::{
    PbAddLiquidityLayout, PbAdminSettingsLayout, PbExchangeLayout, PbInitializeLayout,
    PbRemoveLiquidityLayout, PbRemoveLiquidityOneTokenLayout, PbSetAdminSettingLayout,
};

#[derive(BorshDeserialize, Debug, Default)]
pub struct AdminSettingsLayout {
    swap_enabled: bool,
    add_liquidity_enabled: bool,
}

impl AdminSettingsLayout {
    pub fn to_proto_struct(&self) -> PbAdminSettingsLayout {
        PbAdminSettingsLayout {
            swap_enabled: self.swap_enabled,
            add_liquidity_enabled: self.add_liquidity_enabled,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct InitializeLayout {
    nonce: u8,
    amplification_coefficient: u64,
    fee_numerator: u64,
    n_coins: u8,
    admin_settings: AdminSettingsLayout,
}

impl InitializeLayout {
    pub fn to_proto_struct(&self) -> PbInitializeLayout {
        PbInitializeLayout {
            nonce: self.nonce as u32,
            amplification_coefficient: self.amplification_coefficient,
            fee_numerator: self.fee_numerator,
            n_coins: self.n_coins as u32,
            admin_settings: self.admin_settings.to_proto_struct(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct AddLiquidityLayout {
    deposit_amounts: Vec<u64>,
    min_mint_amount: u64,
}

impl AddLiquidityLayout {
    pub fn to_proto_struct(&self) -> PbAddLiquidityLayout {
        PbAddLiquidityLayout {
            deposit_amounts: self.deposit_amounts.clone(),
            min_mint_amount: self.min_mint_amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct RemoveLiquidityLayout {
    unmint_amount: u64,
    minimum_amounts: Vec<u64>,
}

impl RemoveLiquidityLayout {
    pub fn to_proto_struct(&self) -> PbRemoveLiquidityLayout {
        PbRemoveLiquidityLayout {
            unmint_amount: self.unmint_amount,
            minimum_amounts: self.minimum_amounts.clone(),
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct RemoveLiquidityOneTokenLayout {
    unmint_amount: u64,
    minimum_out_amount: u64,
}

impl RemoveLiquidityOneTokenLayout {
    pub fn to_proto_struct(&self) -> PbRemoveLiquidityOneTokenLayout {
        PbRemoveLiquidityOneTokenLayout {
            unmint_amount: self.unmint_amount,
            minimum_out_amount: self.minimum_out_amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct ExchangeLayout {
    in_amount: u64,
    minimum_out_amount: u64,
}

impl ExchangeLayout {
    pub fn to_proto_struct(&self) -> PbExchangeLayout {
        PbExchangeLayout {
            in_amount: self.in_amount,
            minimum_out_amount: self.minimum_out_amount,
        }
    }
}

#[derive(BorshDeserialize, Debug, Default)]
pub struct SetAdminSettingLayout {
    admin_setting: AdminSettingsLayout,
}

impl SetAdminSettingLayout {
    pub fn to_proto_struct(&self) -> PbSetAdminSettingLayout {
        PbSetAdminSettingLayout {
            admin_setting: self.admin_setting.to_proto_struct(),
        }
    }
}
