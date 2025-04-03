use borsh::{BorshDeserialize, BorshSerialize};

// Instruction argument layout structs for Borsh deserialization
#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeConfigLayoutArgs {
    pub fee_authority: [u8; 32],
    pub collect_protocol_fees_authority: [u8; 32],
    pub reward_emissions_super_authority: [u8; 32],
    pub default_protocol_fee_rate: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializePoolLayoutArgs {
    pub bumps_whirlpool_bump: u8,
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeTickArrayLayoutArgs {
    pub start_tick_index: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeFeeTierLayoutArgs {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeRewardLayoutArgs {
    pub reward_index: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetRewardEmissionsLayoutArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct OpenPositionLayoutArgs {
    pub bumps_position_bump: u8,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct OpenPositionWithMetadataLayoutArgs {
    pub bumps_position_bump: u8,
    pub bumps_metadata_bump: u8,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct IncreaseLiquidityLayoutArgs {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DecreaseLiquidityLayoutArgs {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct UpdateFeesAndRewardsLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CollectFeesLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CollectRewardLayoutArgs {
    pub reward_index: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CollectProtocolFeesLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SwapLayoutArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct ClosePositionLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetDefaultFeeRateLayoutArgs {
    pub default_fee_rate: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetDefaultProtocolFeeRateLayoutArgs {
    pub default_protocol_fee_rate: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetFeeRateLayoutArgs {
    pub fee_rate: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetProtocolFeeRateLayoutArgs {
    pub protocol_fee_rate: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetFeeAuthorityLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetCollectProtocolFeesAuthorityLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetRewardAuthorityLayoutArgs {
    pub reward_index: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetRewardAuthorityBySuperAuthorityLayoutArgs {
    pub reward_index: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetRewardEmissionsSuperAuthorityLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TwoHopSwapLayoutArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializePositionBundleLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializePositionBundleWithMetadataLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DeletePositionBundleLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct OpenBundledPositionLayoutArgs {
    pub bundle_index: u16,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CloseBundledPositionLayoutArgs {
    pub bundle_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CollectFeesV2LayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CollectProtocolFeesV2LayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct CollectRewardV2LayoutArgs {
    pub reward_index: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DecreaseLiquidityV2LayoutArgs {
    pub liquidity_amount: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct IncreaseLiquidityV2LayoutArgs {
    pub liquidity_amount: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializePoolV2LayoutArgs {
    pub tick_spacing: u16,
    pub initial_sqrt_price: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeRewardV2LayoutArgs {
    pub reward_index: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetRewardEmissionsV2LayoutArgs {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SwapV2LayoutArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct TwoHopSwapV2LayoutArgs {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub amount_specified_is_input: bool,
    pub a_to_b_one: bool,
    pub a_to_b_two: bool,
    pub sqrt_price_limit_one: u128,
    pub sqrt_price_limit_two: u128,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeConfigExtensionLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetConfigExtensionAuthorityLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct SetTokenBadgeAuthorityLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeTokenBadgeLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct DeleteTokenBadgeLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct OpenPositionWithTokenExtensionsLayoutArgs {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct InitializeAccountLayoutArgs {}

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct IdlWriteLayoutArgs {} 