extern crate bs58;

use borsh::BorshDeserialize;
use bytes::Buf;

use super::structs::{
    AddLiquidityLayout, ExchangeLayout, InitializeLayout, RemoveLiquidityLayout,
    RemoveLiquidityOneTokenLayout, SetAdminSettingLayout,
};

const INITIALIZE_DISCRIMINATOR: u8 = 0;
const ADD_LIQUIDITY_DISCRIMINATOR: u8 = 1;
const REMOVE_LIQUIDITY_DISCRIMINATOR: u8 = 2;
const REMOVE_LIQUIDITY_ONE_TOKEN_DISCRIMINATOR: u8 = 3;
const EXCHANGE_DISCRIMINATOR: u8 = 4;
const GET_VIRTUAL_PRICE_DISCRIMINATOR: u8 = 5;
const SET_ADMIN_SETTING_DISCRIMINATOR: u8 = 6;

#[derive(Debug, Default)]
pub struct Instruction {
    pub instructionType: String,
    pub initialize: InitializeLayout,
    pub addLiquidity: AddLiquidityLayout,
    pub removeLiquidity: RemoveLiquidityLayout,
    pub removeLiquidityOneToken: RemoveLiquidityOneTokenLayout,
    pub exchange: ExchangeLayout,
    pub setAdminSetting: SetAdminSettingLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut result: Instruction = Instruction::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = disc_bytes.clone().get_u8();
    let rest_bytes = &mut rest.clone();

    match discriminator {
        INITIALIZE_DISCRIMINATOR => {
            result.instructionType = "Initialize".to_string();
            result.initialize = InitializeLayout::deserialize(rest_bytes).unwrap();
        }
        ADD_LIQUIDITY_DISCRIMINATOR => {
            result.instructionType = "AddLiquidity".to_string();
            result.addLiquidity = AddLiquidityLayout::deserialize(rest_bytes).unwrap();
        }
        REMOVE_LIQUIDITY_DISCRIMINATOR => {
            result.instructionType = "RemoveLiquidity".to_string();
            result.removeLiquidity = RemoveLiquidityLayout::deserialize(rest_bytes).unwrap();
        }
        REMOVE_LIQUIDITY_ONE_TOKEN_DISCRIMINATOR => {
            result.instructionType = "RemoveLiquidityOneToken".to_string();
            result.removeLiquidityOneToken =
                RemoveLiquidityOneTokenLayout::deserialize(rest_bytes).unwrap();
        }
        EXCHANGE_DISCRIMINATOR => {
            result.instructionType = "Exchange".to_string();
            result.exchange = ExchangeLayout::deserialize(rest_bytes).unwrap();
        }
        GET_VIRTUAL_PRICE_DISCRIMINATOR => {
            result.instructionType = "GetVirtualPrice".to_string();
        }
        SET_ADMIN_SETTING_DISCRIMINATOR => {
            result.instructionType = "SetAdminSetting".to_string();
            result.setAdminSetting = SetAdminSettingLayout::deserialize(rest_bytes).unwrap();
        }
        _ => {}
    }

    return result;
}
