use crate::pb::sf::solana::orca_whirlpool::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &[u32],
    accounts: &[String],
) -> InputAccounts {
    let mut input_accounts = InputAccounts::default();

    // Map account indices to their roles based on instruction type
    match instruction_type.as_str() {
        "InitializeConfig" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "InitializePool" => {
            if account_indices.len() >= 11 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_mint_a = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.token_mint_b = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.fee_tier = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "InitializeTickArray" => {
            if account_indices.len() >= 4 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.tick_array = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "InitializeFeeTier" => {
            if account_indices.len() >= 5 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.fee_tier = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.fee_authority = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[4] as usize].clone());
            }
        }
        "InitializeReward" => {
            if account_indices.len() >= 8 {
                input_accounts.reward_authority = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.reward_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[7] as usize].clone());
            }
        }
        "SetRewardEmissions" => {
            if account_indices.len() >= 3 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.reward_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "OpenPosition" => {
            if account_indices.len() >= 10 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.associated_token_program = Some(accounts[account_indices[9] as usize].clone());
            }
        }
        "OpenPositionWithMetadata" => {
            if account_indices.len() >= 13 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_metadata_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.associated_token_program = Some(accounts[account_indices[10] as usize].clone());
                input_accounts.metadata_program = Some(accounts[account_indices[11] as usize].clone());
                input_accounts.metadata_update_auth = Some(accounts[account_indices[12] as usize].clone());
            }
        }
        "IncreaseLiquidity" => {
            if account_indices.len() >= 11 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.tick_array_lower = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.tick_array_upper = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "DecreaseLiquidity" => {
            if account_indices.len() >= 11 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.tick_array_lower = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.tick_array_upper = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "UpdateFeesAndRewards" => {
            if account_indices.len() >= 4 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.tick_array_lower = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.tick_array_upper = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "CollectFees" => {
            if account_indices.len() >= 9 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[8] as usize].clone());
            }
        }
        "CollectReward" => {
            if account_indices.len() >= 7 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.reward_owner_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[6] as usize].clone());
            }
        }
        "CollectProtocolFees" => {
            if account_indices.len() >= 8 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.collect_protocol_fees_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_destination_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_destination_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[7] as usize].clone());
            }
        }
        "Swap" => {
            if account_indices.len() >= 11 {
                input_accounts.token_program = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.tick_array_0 = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.tick_array_1 = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.tick_array_2 = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.oracle = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "ClosePosition" => {
            if account_indices.len() >= 6 {
                input_accounts.position_authority = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.receiver = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[5] as usize].clone());
            }
        }
        "TwoHopSwap" => {
            if account_indices.len() >= 20 {
                input_accounts.token_program = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.whirlpool_one = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.whirlpool_two = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_owner_account_one_a = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_vault_one_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_one_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_one_b = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_owner_account_two_a = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.token_vault_two_a = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.token_owner_account_two_b = Some(accounts[account_indices[10] as usize].clone());
                input_accounts.token_vault_two_b = Some(accounts[account_indices[11] as usize].clone());
                input_accounts.tick_array_one_0 = Some(accounts[account_indices[12] as usize].clone());
                input_accounts.tick_array_one_1 = Some(accounts[account_indices[13] as usize].clone());
                input_accounts.tick_array_one_2 = Some(accounts[account_indices[14] as usize].clone());
                input_accounts.tick_array_two_0 = Some(accounts[account_indices[15] as usize].clone());
                input_accounts.tick_array_two_1 = Some(accounts[account_indices[16] as usize].clone());
                input_accounts.tick_array_two_2 = Some(accounts[account_indices[17] as usize].clone());
                input_accounts.oracle_one = Some(accounts[account_indices[18] as usize].clone());
                input_accounts.oracle_two = Some(accounts[account_indices[19] as usize].clone());
            }
        }
        "SetDefaultFeeRate" => {
            if account_indices.len() >= 4 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.fee_tier = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.fee_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "SetDefaultProtocolFeeRate" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.fee_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "SetFeeRate" => {
            if account_indices.len() >= 4 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.fee_tier = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.fee_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "SetProtocolFeeRate" => {
            if account_indices.len() >= 4 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.fee_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "SetFeeAuthority" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.fee_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_fee_authority = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "SetCollectProtocolFeesAuthority" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.collect_protocol_fees_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_collect_protocol_fees_authority = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "SetRewardAuthority" => {
            if account_indices.len() >= 4 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.reward_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_reward_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "SetRewardAuthorityBySuperAuthority" => {
            if account_indices.len() >= 4 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.reward_emissions_super_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_reward_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "SetRewardEmissionsSuperAuthority" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.reward_emissions_super_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_reward_emissions_super_authority = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "InitializePositionBundle" => {
            if account_indices.len() >= 6 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_bundle = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_bundle_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_bundle_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[5] as usize].clone());
            }
        }
        "InitializePositionBundleWithMetadata" => {
            if account_indices.len() >= 9 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_bundle = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_bundle_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_bundle_metadata = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.position_bundle_token_account = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.metadata_program = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.metadata_update_auth = Some(accounts[account_indices[8] as usize].clone());
            }
        }
        "DeletePositionBundle" => {
            if account_indices.len() >= 5 {
                input_accounts.position_bundle_owner = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position_bundle = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_bundle_mint = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_bundle_token_account = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[4] as usize].clone());
            }
        }
        "OpenBundledPosition" => {
            if account_indices.len() >= 11 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_bundle = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_bundle_owner = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_bundle_authority = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.bundled_position = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.associated_token_program = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "CloseBundledPosition" => {
            if account_indices.len() >= 7 {
                input_accounts.position_bundle_owner = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position_bundle_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.bundled_position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[6] as usize].clone());
            }
        }
        "CollectFeesV2" => {
            if account_indices.len() >= 9 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[8] as usize].clone());
            }
        }
        "CollectProtocolFeesV2" => {
            if account_indices.len() >= 8 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.collect_protocol_fees_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_destination_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_destination_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[7] as usize].clone());
            }
        }
        "CollectRewardV2" => {
            if account_indices.len() >= 7 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.reward_owner_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[6] as usize].clone());
            }
        }
        "DecreaseLiquidityV2" => {
            if account_indices.len() >= 11 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.tick_array_lower = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.tick_array_upper = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "IncreaseLiquidityV2" => {
            if account_indices.len() >= 11 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position_authority = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.tick_array_lower = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.tick_array_upper = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "InitializePoolV2" => {
            if account_indices.len() >= 11 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_mint_a = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.token_mint_b = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.fee_tier = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "InitializeRewardV2" => {
            if account_indices.len() >= 8 {
                input_accounts.reward_authority = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.funder = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.reward_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.rent = Some(accounts[account_indices[7] as usize].clone());
            }
        }
        "SetRewardEmissionsV2" => {
            if account_indices.len() >= 3 {
                input_accounts.whirlpool = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.reward_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.reward_vault = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "SwapV2" => {
            if account_indices.len() >= 11 {
                input_accounts.token_program = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.token_owner_account_a = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_vault_a = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_owner_account_b = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_vault_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.tick_array_0 = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.tick_array_1 = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.tick_array_2 = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.oracle = Some(accounts[account_indices[10] as usize].clone());
            }
        }
        "TwoHopSwapV2" => {
            if account_indices.len() >= 20 {
                input_accounts.token_program = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.whirlpool_one = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.whirlpool_two = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.token_owner_account_one_a = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.token_vault_one_a = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_owner_account_one_b = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.token_vault_one_b = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.token_owner_account_two_a = Some(accounts[account_indices[8] as usize].clone());
                input_accounts.token_vault_two_a = Some(accounts[account_indices[9] as usize].clone());
                input_accounts.token_owner_account_two_b = Some(accounts[account_indices[10] as usize].clone());
                input_accounts.token_vault_two_b = Some(accounts[account_indices[11] as usize].clone());
                input_accounts.tick_array_one_0 = Some(accounts[account_indices[12] as usize].clone());
                input_accounts.tick_array_one_1 = Some(accounts[account_indices[13] as usize].clone());
                input_accounts.tick_array_one_2 = Some(accounts[account_indices[14] as usize].clone());
                input_accounts.tick_array_two_0 = Some(accounts[account_indices[15] as usize].clone());
                input_accounts.tick_array_two_1 = Some(accounts[account_indices[16] as usize].clone());
                input_accounts.tick_array_two_2 = Some(accounts[account_indices[17] as usize].clone());
                input_accounts.oracle_one = Some(accounts[account_indices[18] as usize].clone());
                input_accounts.oracle_two = Some(accounts[account_indices[19] as usize].clone());
            }
        }
        "InitializeConfigExtension" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.config_extension = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "SetConfigExtensionAuthority" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.config_extension = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_config_extension_authority = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "SetTokenBadgeAuthority" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_badge_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.new_token_badge_authority = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "InitializeTokenBadge" => {
            if account_indices.len() >= 4 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_badge_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.token_mint = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.token_badge = Some(accounts[account_indices[3] as usize].clone());
            }
        }
        "DeleteTokenBadge" => {
            if account_indices.len() >= 3 {
                input_accounts.config = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.token_badge_authority = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.token_badge = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "OpenPositionWithTokenExtensions" => {
            if account_indices.len() >= 10 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.position = Some(accounts[account_indices[2] as usize].clone());
                input_accounts.position_mint = Some(accounts[account_indices[3] as usize].clone());
                input_accounts.position_token_account = Some(accounts[account_indices[4] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[5] as usize].clone());
                input_accounts.token_program = Some(accounts[account_indices[6] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[7] as usize].clone());
                input_accounts.associated_token_program = Some(accounts[account_indices[8] as usize].clone());
            }
        }
        "InitializeAccount" => {
            if account_indices.len() >= 3 {
                input_accounts.funder = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.owner = Some(accounts[account_indices[1] as usize].clone());
                input_accounts.system_program = Some(accounts[account_indices[2] as usize].clone());
            }
        }
        "IdlWrite" => {
            if account_indices.len() >= 2 {
                input_accounts.token_authority = Some(accounts[account_indices[0] as usize].clone());
                input_accounts.whirlpool = Some(accounts[account_indices[1] as usize].clone());
            }
        }
        // Add more instruction types as needed
        _ => {}
    }

    input_accounts
} 