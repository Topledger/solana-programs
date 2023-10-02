extern crate bs58;
use borsh::{BorshDeserialize, BorshSerialize};
use core::fmt;

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct FeesLayout {
    pub trade_fee_numerator: u64,
    pub trade_fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub owner_withdraw_fee_denominator: u64,
    pub host_fee_numerator: u64,
    pub host_fee_denominator: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
#[repr(u8)]
pub enum CurveTypeLayout {
    #[default]
    ConstantProduct,
    ConstantPrice,
    Stable,
    Offset,
}

impl fmt::Display for CurveTypeLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CurveTypeLayout::ConstantProduct => write!(f, "ConstantProduct"),
            CurveTypeLayout::ConstantPrice => write!(f, "ConstantPrice"),
            CurveTypeLayout::Stable => write!(f, "Stable"),
            CurveTypeLayout::Offset => write!(f, "Offset"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CurveCalculatorLayout {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SwapCurveLayout {
    pub curve_type: CurveTypeLayout,
    pub calculator: CurveCalculatorLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeLayout {
    pub nonce: u8,
    pub fees: FeesLayout,
    pub swap_curve: SwapCurveLayout,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SwapLayout {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DepositAllTokenTypesLayout {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct WithdrawAllTokenTypesLayout {
    pub pool_token_amount: u64,
    pub maximum_token_a_amount: u64,
    pub maximum_token_b_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DepositSingleTokenTypeExactAmountInLayout {
    pub source_token_amount: u64,
    pub minimum_pool_token_amount: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct WithdrawSingleTokenTypeExactAmountOutLayout {
    pub destination_token_amount: u64,
    pub maximum_pool_token_amount: u64,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub initializeArg: InitializeLayout,
    pub swapArg: SwapLayout,
    pub depositAllTokenTypesArg: DepositAllTokenTypesLayout,
    pub withdrawAllTokenTypesArg: WithdrawAllTokenTypesLayout,
    pub depositSingleTokenTypeExactAmountInArg: DepositSingleTokenTypeExactAmountInLayout,
    pub withdrawSingleTokenTypeExactAmountOutArg: WithdrawSingleTokenTypeExactAmountOutLayout,
}

pub fn parse_instruction(bytes_stream: Vec<u8>) -> Instruction {
    let mut instruction_name = String::default();

    let mut initializeArg: InitializeLayout = InitializeLayout::default();
    let mut swapArg: SwapLayout = SwapLayout::default();
    let mut depositAllTokenTypesArg: DepositAllTokenTypesLayout =
        DepositAllTokenTypesLayout::default();
    let mut withdrawAllTokenTypesArg: WithdrawAllTokenTypesLayout =
        WithdrawAllTokenTypesLayout::default();
    let mut depositSingleTokenTypeExactAmountInArg: DepositSingleTokenTypeExactAmountInLayout =
        DepositSingleTokenTypeExactAmountInLayout::default();
    let mut withdrawSingleTokenTypeExactAmountOutArg: WithdrawSingleTokenTypeExactAmountOutLayout =
        WithdrawSingleTokenTypeExactAmountOutLayout::default();

    let (disc_bytes, rest) = bytes_stream.split_at(1);
    let discriminator: u8 = u8::from(disc_bytes[0]);

    match discriminator {
        0 => {
            instruction_name = String::from("Initialize");
            initializeArg = InitializeLayout::try_from_slice(rest).unwrap();
        }
        1 => {
            instruction_name = String::from("Swap");
            swapArg = SwapLayout::try_from_slice(rest).unwrap();
        }
        2 => {
            instruction_name = String::from("DepositAllTokenTypes");
            depositAllTokenTypesArg = DepositAllTokenTypesLayout::try_from_slice(rest).unwrap();
        }
        3 => {
            instruction_name = String::from("WithdrawAllTokenTypes");
            withdrawAllTokenTypesArg = WithdrawAllTokenTypesLayout::try_from_slice(rest).unwrap();
        }
        4 => {
            instruction_name = String::from("DepositSingleTokenTypeExactAmountIn");
            depositSingleTokenTypeExactAmountInArg =
                DepositSingleTokenTypeExactAmountInLayout::try_from_slice(rest).unwrap();
        }
        5 => {
            instruction_name = String::from("WithdrawSingleTokenTypeExactAmountOut");
            withdrawSingleTokenTypeExactAmountOutArg =
                WithdrawSingleTokenTypeExactAmountOutLayout::try_from_slice(rest).unwrap();
        }
        _ => {}
    }

    let result: Instruction = Instruction {
        name: instruction_name,
        initializeArg: initializeArg,
        swapArg: swapArg,
        depositAllTokenTypesArg: depositAllTokenTypesArg,
        withdrawAllTokenTypesArg: withdrawAllTokenTypesArg,
        depositSingleTokenTypeExactAmountInArg: depositSingleTokenTypeExactAmountInArg,
        withdrawSingleTokenTypeExactAmountOutArg: withdrawSingleTokenTypeExactAmountOutArg,
    };

    return result;
}
