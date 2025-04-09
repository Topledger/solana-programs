use crate::pb::sf::solana::orca_whirlpool::v1::InputAccounts;

// This function maps the flat list of account keys from the instruction
// to the named accounts based on the instruction type and its IDL definition.
pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &[u32],
    accounts: &[String],
) -> InputAccounts {
    let mut input_accounts = InputAccounts::default();
    let accounts_len = account_indices.len();

    // Helper macro to safely get account key by index
    macro_rules! get_account {
        ($index:expr) => {
            if $index < accounts_len {
                Some(accounts[account_indices[$index] as usize].clone())
            } else {
                // Log error or handle missing account? Returning None for safety.
                None
            }
        };
    }

    match instruction_type.as_str() {
        "InitializeConfig" => {
            // IDL: config, funder, systemProgram
            input_accounts.config = get_account!(0);
            input_accounts.funder = get_account!(1);
            input_accounts.system_program = get_account!(2);
        }
        "InitializePool" => {
            // IDL: whirlpoolsConfig, tokenMintA, tokenMintB, funder, whirlpool, 
            //      tokenVaultA, tokenVaultB, feeTier, tokenProgram, systemProgram, rent
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.token_mint_a = get_account!(1);
            input_accounts.token_mint_b = get_account!(2);
            input_accounts.funder = get_account!(3);
            input_accounts.whirlpool = get_account!(4);
            input_accounts.token_vault_a = get_account!(5);
            input_accounts.token_vault_b = get_account!(6);
            input_accounts.fee_tier = get_account!(7);
            input_accounts.token_program = get_account!(8);
            input_accounts.system_program = get_account!(9);
            input_accounts.rent = get_account!(10);
        }
        "InitializeTickArray" => {
            // IDL: whirlpool, funder, tickArray, systemProgram
            input_accounts.whirlpool = get_account!(0);
            input_accounts.funder = get_account!(1);
            input_accounts.tick_array = get_account!(2); // Renamed from tick_array_0/1/2
            input_accounts.system_program = get_account!(3);
        }
        "InitializeFeeTier" => {
            // IDL: config, feeTier, funder, feeAuthority, systemProgram
            input_accounts.config = get_account!(0);
            input_accounts.fee_tier = get_account!(1);
            input_accounts.funder = get_account!(2);
            input_accounts.fee_authority = get_account!(3);
            input_accounts.system_program = get_account!(4);
        }
        "InitializeReward" => {
            // IDL: rewardAuthority, funder, whirlpool, rewardMint, rewardVault, 
            //      tokenProgram, systemProgram, rent
            input_accounts.reward_authority = get_account!(0);
            input_accounts.funder = get_account!(1);
            input_accounts.whirlpool = get_account!(2);
            input_accounts.reward_mint = get_account!(3);
            input_accounts.reward_vault = get_account!(4);
            input_accounts.token_program = get_account!(5);
            input_accounts.system_program = get_account!(6);
            input_accounts.rent = get_account!(7);
        }
        "SetRewardEmissions" => {
            // IDL: whirlpool, rewardAuthority, rewardVault
            input_accounts.whirlpool = get_account!(0);
            input_accounts.reward_authority = get_account!(1);
            input_accounts.reward_vault = get_account!(2);
        }
        "OpenPosition" => {
            // IDL: funder, owner, position, positionMint, positionTokenAccount, whirlpool, 
            //      tokenProgram, systemProgram, rent, associatedTokenProgram
            input_accounts.funder = get_account!(0);
            input_accounts.owner = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_mint = get_account!(3);
            input_accounts.position_token_account = get_account!(4);
            input_accounts.whirlpool = get_account!(5);
            input_accounts.token_program = get_account!(6);
            input_accounts.system_program = get_account!(7);
            input_accounts.rent = get_account!(8);
            input_accounts.associated_token_program = get_account!(9);
        }
        "OpenPositionWithMetadata" => {
            // IDL: funder, owner, position, positionMint, positionMetadataAccount, positionTokenAccount, whirlpool, 
            //      tokenProgram, systemProgram, rent, associatedTokenProgram, metadataProgram, metadataUpdateAuth
            input_accounts.funder = get_account!(0);
            input_accounts.owner = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_mint = get_account!(3);
            input_accounts.position_metadata_account = get_account!(4);
            input_accounts.position_token_account = get_account!(5);
            input_accounts.whirlpool = get_account!(6);
            input_accounts.token_program = get_account!(7);
            input_accounts.system_program = get_account!(8);
            input_accounts.rent = get_account!(9);
            input_accounts.associated_token_program = get_account!(10);
            input_accounts.metadata_program = get_account!(11);
            input_accounts.metadata_update_auth = get_account!(12);
        }
        "IncreaseLiquidity" => {
            // IDL: whirlpool, tokenProgram, positionAuthority, position, positionTokenAccount, 
            //      tokenOwnerAccountA, tokenOwnerAccountB, tokenVaultA, tokenVaultB, tickArrayLower, tickArrayUpper
            input_accounts.whirlpool = get_account!(0);
            input_accounts.token_program = get_account!(1);
            input_accounts.position_authority = get_account!(2);
            input_accounts.position = get_account!(3);
            input_accounts.position_token_account = get_account!(4);
            input_accounts.token_owner_account_a = get_account!(5);
            input_accounts.token_owner_account_b = get_account!(6);
            input_accounts.token_vault_a = get_account!(7);
            input_accounts.token_vault_b = get_account!(8);
            input_accounts.tick_array_lower = get_account!(9);
            input_accounts.tick_array_upper = get_account!(10);
        }
        "DecreaseLiquidity" => {
            // IDL: whirlpool, tokenProgram, positionAuthority, position, positionTokenAccount, 
            //      tokenOwnerAccountA, tokenOwnerAccountB, tokenVaultA, tokenVaultB, tickArrayLower, tickArrayUpper
            input_accounts.whirlpool = get_account!(0);
            input_accounts.token_program = get_account!(1);
            input_accounts.position_authority = get_account!(2);
            input_accounts.position = get_account!(3);
            input_accounts.position_token_account = get_account!(4);
            input_accounts.token_owner_account_a = get_account!(5);
            input_accounts.token_owner_account_b = get_account!(6);
            input_accounts.token_vault_a = get_account!(7);
            input_accounts.token_vault_b = get_account!(8);
            input_accounts.tick_array_lower = get_account!(9);
            input_accounts.tick_array_upper = get_account!(10);
        }
        "UpdateFeesAndRewards" => {
            // IDL: whirlpool, position, tickArrayLower, tickArrayUpper
            input_accounts.whirlpool = get_account!(0);
            input_accounts.position = get_account!(1);
            input_accounts.tick_array_lower = get_account!(2);
            input_accounts.tick_array_upper = get_account!(3);
        }
        "CollectFees" => {
            // IDL: whirlpool, positionAuthority, position, positionTokenAccount, 
            //      tokenOwnerAccountA, tokenVaultA, tokenOwnerAccountB, tokenVaultB, tokenProgram
            input_accounts.whirlpool = get_account!(0);
            input_accounts.position_authority = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_token_account = get_account!(3);
            input_accounts.token_owner_account_a = get_account!(4);
            input_accounts.token_vault_a = get_account!(5);
            input_accounts.token_owner_account_b = get_account!(6);
            input_accounts.token_vault_b = get_account!(7);
            input_accounts.token_program = get_account!(8);
        }
        "CollectReward" => {
            // IDL: whirlpool, positionAuthority, position, positionTokenAccount, 
            //      rewardOwnerAccount, rewardVault, tokenProgram
            input_accounts.whirlpool = get_account!(0);
            input_accounts.position_authority = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_token_account = get_account!(3);
            input_accounts.reward_owner_account = get_account!(4);
            input_accounts.reward_vault = get_account!(5);
            input_accounts.token_program = get_account!(6);
        }
        "CollectProtocolFees" => {
            // IDL: whirlpoolsConfig, whirlpool, collectProtocolFeesAuthority, tokenVaultA, tokenVaultB, 
            //      tokenDestinationA, tokenDestinationB, tokenProgram
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.whirlpool = get_account!(1);
            input_accounts.collect_protocol_fees_authority = get_account!(2);
            input_accounts.token_vault_a = get_account!(3);
            input_accounts.token_vault_b = get_account!(4);
            input_accounts.token_destination_a = get_account!(5);
            input_accounts.token_destination_b = get_account!(6);
            input_accounts.token_program = get_account!(7);
        }
        "Swap" => {
            // IDL: tokenProgram, tokenAuthority, whirlpool, tokenOwnerAccountA, tokenVaultA, 
            //      tokenOwnerAccountB, tokenVaultB, tickArray0, tickArray1, tickArray2, oracle
            input_accounts.token_program = get_account!(0);
            input_accounts.token_authority = get_account!(1);
            input_accounts.whirlpool = get_account!(2);
            input_accounts.token_owner_account_a = get_account!(3);
            input_accounts.token_vault_a = get_account!(4);
            input_accounts.token_owner_account_b = get_account!(5);
            input_accounts.token_vault_b = get_account!(6);
            input_accounts.tick_array_0 = get_account!(7);
            input_accounts.tick_array_1 = get_account!(8);
            input_accounts.tick_array_2 = get_account!(9);
            input_accounts.oracle = get_account!(10);
        }
        "ClosePosition" => {
            // IDL: positionAuthority, receiver, position, positionMint, positionTokenAccount, tokenProgram
            input_accounts.position_authority = get_account!(0);
            input_accounts.receiver = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_mint = get_account!(3);
            input_accounts.position_token_account = get_account!(4);
            input_accounts.token_program = get_account!(5);
        }
        "SetDefaultFeeRate" => {
            // IDL: whirlpoolsConfig, feeTier, feeAuthority
            // Missing systemProgram in current code, but IDL doesn't show it either?
            // Assuming IDL is correct. If calls fail, check if systemProgram is needed implicitly.
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.fee_tier = get_account!(1);
            input_accounts.fee_authority = get_account!(2);
        }
        "SetDefaultProtocolFeeRate" => {
            // IDL: whirlpoolsConfig, feeAuthority
            // Missing systemProgram in current code, IDL also doesn't show it.
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.fee_authority = get_account!(1);
        }
        "SetFeeRate" => {
            // IDL: whirlpoolsConfig, whirlpool, feeAuthority
            // Missing systemProgram in current code, IDL also doesn't show it.
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.whirlpool = get_account!(1);
            input_accounts.fee_authority = get_account!(2);
        }
        "SetProtocolFeeRate" => {
            // IDL: whirlpoolsConfig, whirlpool, feeAuthority
            // Missing systemProgram in current code, IDL also doesn't show it.
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.whirlpool = get_account!(1);
            input_accounts.fee_authority = get_account!(2);
        }
        "SetFeeAuthority" => {
            // IDL: whirlpoolsConfig, feeAuthority, newFeeAuthority
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.fee_authority = get_account!(1);
            input_accounts.new_fee_authority = get_account!(2);
        }
        "SetCollectProtocolFeesAuthority" => {
            // IDL: whirlpoolsConfig, collectProtocolFeesAuthority, newCollectProtocolFeesAuthority
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.collect_protocol_fees_authority = get_account!(1);
            input_accounts.new_collect_protocol_fees_authority = get_account!(2);
        }
        "SetRewardAuthority" => {
            // IDL: whirlpool, rewardAuthority, newRewardAuthority
            // Current code has rewardVault at index 3, IDL does not.
            input_accounts.whirlpool = get_account!(0);
            input_accounts.reward_authority = get_account!(1);
            input_accounts.new_reward_authority = get_account!(2);
        }
        "SetRewardAuthorityBySuperAuthority" => {
            // IDL: whirlpoolsConfig, whirlpool, rewardEmissionsSuperAuthority, newRewardAuthority
            // Current code has rewardVault at index 3, IDL does not.
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.whirlpool = get_account!(1);
            input_accounts.reward_emissions_super_authority = get_account!(2);
            input_accounts.new_reward_authority = get_account!(3);
        }
        "SetRewardEmissionsSuperAuthority" => {
            // IDL: whirlpoolsConfig, rewardEmissionsSuperAuthority, newRewardEmissionsSuperAuthority
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.reward_emissions_super_authority = get_account!(1);
            input_accounts.new_reward_emissions_super_authority = get_account!(2);
        }
        "TwoHopSwap" => {
            // IDL: tokenProgram, tokenAuthority, whirlpoolOne, whirlpoolTwo, tokenOwnerAccountOneA, tokenVaultOneA,
            //      tokenOwnerAccountOneB, tokenVaultOneB, tokenOwnerAccountTwoA, tokenVaultTwoA, 
            //      tokenOwnerAccountTwoB, tokenVaultTwoB, tickArrayOne0, tickArrayOne1, tickArrayOne2, 
            //      tickArrayTwo0, tickArrayTwo1, tickArrayTwo2, oracleOne, oracleTwo
            input_accounts.token_program = get_account!(0);
            input_accounts.token_authority = get_account!(1);
            input_accounts.whirlpool_one = get_account!(2);
            input_accounts.whirlpool_two = get_account!(3);
            input_accounts.token_owner_account_one_a = get_account!(4);
            input_accounts.token_vault_one_a = get_account!(5);
            input_accounts.token_owner_account_one_b = get_account!(6);
            input_accounts.token_vault_one_b = get_account!(7);
            input_accounts.token_owner_account_two_a = get_account!(8);
            input_accounts.token_vault_two_a = get_account!(9);
            input_accounts.token_owner_account_two_b = get_account!(10);
            input_accounts.token_vault_two_b = get_account!(11);
            input_accounts.tick_array_one_0 = get_account!(12);
            input_accounts.tick_array_one_1 = get_account!(13);
            input_accounts.tick_array_one_2 = get_account!(14);
            input_accounts.tick_array_two_0 = get_account!(15);
            input_accounts.tick_array_two_1 = get_account!(16);
            input_accounts.tick_array_two_2 = get_account!(17);
            input_accounts.oracle_one = get_account!(18);
            input_accounts.oracle_two = get_account!(19);
        }
        "InitializePositionBundle" => {
            // IDL: positionBundle, positionBundleMint, positionBundleTokenAccount, positionBundleOwner, funder, 
            //      tokenProgram, systemProgram, rent, associatedTokenProgram
            input_accounts.position_bundle = get_account!(0);
            input_accounts.position_bundle_mint = get_account!(1);
            input_accounts.position_bundle_token_account = get_account!(2);
            input_accounts.owner = get_account!(3); // positionBundleOwner -> owner
            input_accounts.funder = get_account!(4);
            input_accounts.token_program = get_account!(5);
            input_accounts.system_program = get_account!(6);
            input_accounts.rent = get_account!(7);
            input_accounts.associated_token_program = get_account!(8);
        }
        "InitializePositionBundleWithMetadata" => {
            // IDL: positionBundle, positionBundleMint, positionBundleMetadata, positionBundleTokenAccount, positionBundleOwner, funder, 
            //      metadataUpdateAuth, tokenProgram, systemProgram, rent, associatedTokenProgram, metadataProgram
            input_accounts.position_bundle = get_account!(0);
            input_accounts.position_bundle_mint = get_account!(1);
            input_accounts.position_bundle_metadata = get_account!(2);
            input_accounts.position_bundle_token_account = get_account!(3);
            input_accounts.owner = get_account!(4); // positionBundleOwner -> owner
            input_accounts.funder = get_account!(5);
            input_accounts.metadata_update_auth = get_account!(6);
            input_accounts.token_program = get_account!(7);
            input_accounts.system_program = get_account!(8);
            input_accounts.rent = get_account!(9);
            input_accounts.associated_token_program = get_account!(10);
            input_accounts.metadata_program = get_account!(11);
        }
        "DeletePositionBundle" => {
            // IDL: positionBundle, positionBundleMint, positionBundleTokenAccount, positionBundleOwner, receiver, tokenProgram
            input_accounts.position_bundle = get_account!(0);
            input_accounts.position_bundle_mint = get_account!(1);
            input_accounts.position_bundle_token_account = get_account!(2);
            input_accounts.owner = get_account!(3); // positionBundleOwner -> owner
            input_accounts.receiver = get_account!(4);
            input_accounts.token_program = get_account!(5);
        }
        "OpenBundledPosition" => {
            // IDL: bundledPosition, positionBundle, positionBundleTokenAccount, positionBundleAuthority, whirlpool, 
            //      funder, systemProgram, rent
            // Current code has owner, positionBundleOwner - unclear mapping in IDL
            input_accounts.bundled_position = get_account!(0);
            input_accounts.position_bundle = get_account!(1);
            input_accounts.position_bundle_token_account = get_account!(2);
            input_accounts.position_bundle_authority = get_account!(3);
            input_accounts.whirlpool = get_account!(4);
            input_accounts.funder = get_account!(5);
            input_accounts.system_program = get_account!(6);
            input_accounts.rent = get_account!(7);
            // Missing associatedTokenProgram? No, IDL does not list it.
        }
        "CloseBundledPosition" => {
            // IDL: bundledPosition, positionBundle, positionBundleTokenAccount, positionBundleAuthority, receiver
            // Current code has whirlpool, tokenProgram, systemProgram, rent - not in IDL
            input_accounts.bundled_position = get_account!(0);
            input_accounts.position_bundle = get_account!(1);
            input_accounts.position_bundle_token_account = get_account!(2);
            input_accounts.position_bundle_authority = get_account!(3);
            input_accounts.receiver = get_account!(4);
        }
        "OpenPositionWithTokenExtensions" => {
            // IDL: funder, owner, position, positionMint, positionTokenAccount, whirlpool, 
            //      token2022Program, systemProgram, associatedTokenProgram, metadataUpdateAuth
            input_accounts.funder = get_account!(0);
            input_accounts.owner = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_mint = get_account!(3);
            input_accounts.position_token_account = get_account!(4);
            input_accounts.whirlpool = get_account!(5);
            input_accounts.token_program = get_account!(6); // token2022Program -> token_program
            input_accounts.system_program = get_account!(7);
            input_accounts.associated_token_program = get_account!(8);
            input_accounts.metadata_update_auth = get_account!(9);
        }
        // V2 Instructions need careful review against IDL V2 accounts
        "CollectFeesV2" => {
            // IDL: whirlpool, positionAuthority, position, positionTokenAccount, tokenMintA, tokenMintB,
            //      tokenOwnerAccountA, tokenVaultA, tokenOwnerAccountB, tokenVaultB, tokenProgramA, 
            //      tokenProgramB, memoProgram
            input_accounts.whirlpool = get_account!(0);
            input_accounts.position_authority = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_token_account = get_account!(3);
            input_accounts.token_mint_a = get_account!(4);
            input_accounts.token_mint_b = get_account!(5);
            input_accounts.token_owner_account_a = get_account!(6);
            input_accounts.token_vault_a = get_account!(7);
            input_accounts.token_owner_account_b = get_account!(8);
            input_accounts.token_vault_b = get_account!(9);
            input_accounts.token_program_a = get_account!(10);
            input_accounts.token_program_b = get_account!(11);
            input_accounts.memo_program = get_account!(12);
        }
        "CollectProtocolFeesV2" => {
            // IDL: whirlpoolsConfig, whirlpool, collectProtocolFeesAuthority, tokenMintA, tokenMintB,
            //      tokenVaultA, tokenVaultB, tokenDestinationA, tokenDestinationB, 
            //      tokenProgramA, tokenProgramB, memoProgram
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.whirlpool = get_account!(1);
            input_accounts.collect_protocol_fees_authority = get_account!(2);
            input_accounts.token_mint_a = get_account!(3);
            input_accounts.token_mint_b = get_account!(4);
            input_accounts.token_vault_a = get_account!(5);
            input_accounts.token_vault_b = get_account!(6);
            input_accounts.token_destination_a = get_account!(7);
            input_accounts.token_destination_b = get_account!(8);
            input_accounts.token_program_a = get_account!(9);
            input_accounts.token_program_b = get_account!(10);
            input_accounts.memo_program = get_account!(11);
        }
        "CollectRewardV2" => {
            // IDL: whirlpool, positionAuthority, position, positionTokenAccount, rewardOwnerAccount, 
            //      rewardMint, rewardVault, rewardTokenProgram, memoProgram
            input_accounts.whirlpool = get_account!(0);
            input_accounts.position_authority = get_account!(1);
            input_accounts.position = get_account!(2);
            input_accounts.position_token_account = get_account!(3);
            input_accounts.reward_owner_account = get_account!(4);
            input_accounts.reward_mint = get_account!(5);
            input_accounts.reward_vault = get_account!(6);
            input_accounts.token_program = get_account!(7); // rewardTokenProgram -> token_program
            input_accounts.memo_program = get_account!(8);
        }
        "DecreaseLiquidityV2" => {
            // IDL: whirlpool, tokenProgramA, tokenProgramB, memoProgram, positionAuthority, 
            //      position, positionTokenAccount, tokenMintA, tokenMintB, tokenOwnerAccountA, 
            //      tokenOwnerAccountB, tokenVaultA, tokenVaultB, tickArrayLower, tickArrayUpper
            input_accounts.whirlpool = get_account!(0);
            input_accounts.token_program_a = get_account!(1);
            input_accounts.token_program_b = get_account!(2);
            input_accounts.memo_program = get_account!(3);
            input_accounts.position_authority = get_account!(4);
            input_accounts.position = get_account!(5);
            input_accounts.position_token_account = get_account!(6);
            input_accounts.token_mint_a = get_account!(7);
            input_accounts.token_mint_b = get_account!(8);
            input_accounts.token_owner_account_a = get_account!(9);
            input_accounts.token_owner_account_b = get_account!(10);
            input_accounts.token_vault_a = get_account!(11);
            input_accounts.token_vault_b = get_account!(12);
            input_accounts.tick_array_lower = get_account!(13);
            input_accounts.tick_array_upper = get_account!(14);
        }
        "IncreaseLiquidityV2" => {
            // IDL: whirlpool, tokenProgramA, tokenProgramB, memoProgram, positionAuthority, 
            //      position, positionTokenAccount, tokenMintA, tokenMintB, tokenOwnerAccountA, 
            //      tokenOwnerAccountB, tokenVaultA, tokenVaultB, tickArrayLower, tickArrayUpper
            input_accounts.whirlpool = get_account!(0);
            input_accounts.token_program_a = get_account!(1);
            input_accounts.token_program_b = get_account!(2);
            input_accounts.memo_program = get_account!(3);
            input_accounts.position_authority = get_account!(4);
            input_accounts.position = get_account!(5);
            input_accounts.position_token_account = get_account!(6);
            input_accounts.token_mint_a = get_account!(7);
            input_accounts.token_mint_b = get_account!(8);
            input_accounts.token_owner_account_a = get_account!(9);
            input_accounts.token_owner_account_b = get_account!(10);
            input_accounts.token_vault_a = get_account!(11);
            input_accounts.token_vault_b = get_account!(12);
            input_accounts.tick_array_lower = get_account!(13);
            input_accounts.tick_array_upper = get_account!(14);
        }
        "InitializePoolV2" => {
            // IDL: whirlpoolsConfig, tokenMintA, tokenMintB, tokenBadgeA, tokenBadgeB, funder, 
            //      whirlpool, tokenVaultA, tokenVaultB, feeTier, tokenProgramA, tokenProgramB, 
            //      systemProgram, rent
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.token_mint_a = get_account!(1);
            input_accounts.token_mint_b = get_account!(2);
            input_accounts.token_badge_a = get_account!(3);
            input_accounts.token_badge_b = get_account!(4);
            input_accounts.funder = get_account!(5);
            input_accounts.whirlpool = get_account!(6);
            input_accounts.token_vault_a = get_account!(7);
            input_accounts.token_vault_b = get_account!(8);
            input_accounts.fee_tier = get_account!(9);
            input_accounts.token_program_a = get_account!(10);
            input_accounts.token_program_b = get_account!(11);
            input_accounts.system_program = get_account!(12);
            input_accounts.rent = get_account!(13);
        }
        "InitializeRewardV2" => {
            // IDL: rewardAuthority, funder, whirlpool, rewardMint, rewardTokenBadge, rewardVault, 
            //      rewardTokenProgram, systemProgram, rent
            input_accounts.reward_authority = get_account!(0);
            input_accounts.funder = get_account!(1);
            input_accounts.whirlpool = get_account!(2);
            input_accounts.reward_mint = get_account!(3);
            input_accounts.token_badge = get_account!(4); // rewardTokenBadge -> token_badge
            input_accounts.reward_vault = get_account!(5);
            input_accounts.token_program = get_account!(6); // rewardTokenProgram -> token_program
            input_accounts.system_program = get_account!(7);
            input_accounts.rent = get_account!(8);
        }
        "SetRewardEmissionsV2" => {
            // IDL: whirlpool, rewardAuthority, rewardVault
            input_accounts.whirlpool = get_account!(0);
            input_accounts.reward_authority = get_account!(1);
            input_accounts.reward_vault = get_account!(2);
        }
        "SwapV2" => {
            // IDL: tokenProgramA, tokenProgramB, memoProgram, tokenAuthority, whirlpool, 
            //      tokenMintA, tokenMintB, tokenOwnerAccountA, tokenVaultA, tokenOwnerAccountB, 
            //      tokenVaultB, tickArray0, tickArray1, tickArray2, oracle
            input_accounts.token_program_a = get_account!(0);
            input_accounts.token_program_b = get_account!(1);
            input_accounts.memo_program = get_account!(2);
            input_accounts.token_authority = get_account!(3);
            input_accounts.whirlpool = get_account!(4);
            input_accounts.token_mint_a = get_account!(5);
            input_accounts.token_mint_b = get_account!(6);
            input_accounts.token_owner_account_a = get_account!(7);
            input_accounts.token_vault_a = get_account!(8);
            input_accounts.token_owner_account_b = get_account!(9);
            input_accounts.token_vault_b = get_account!(10);
            input_accounts.tick_array_0 = get_account!(11);
            input_accounts.tick_array_1 = get_account!(12);
            input_accounts.tick_array_2 = get_account!(13);
            input_accounts.oracle = get_account!(14);
        }
        "TwoHopSwapV2" => {
            // IDL: whirlpoolOne, whirlpoolTwo, tokenMintInput, tokenMintIntermediate, tokenMintOutput, 
            //      tokenProgramInput, tokenProgramIntermediate, tokenProgramOutput, tokenOwnerAccountInput, 
            //      tokenVaultOneInput, tokenVaultOneIntermediate, tokenVaultTwoIntermediate, tokenVaultTwoOutput, 
            //      tokenOwnerAccountOutput, tokenAuthority, tickArrayOne0, tickArrayOne1, tickArrayOne2, 
            //      tickArrayTwo0, tickArrayTwo1, tickArrayTwo2, oracleOne, oracleTwo, memoProgram
            input_accounts.whirlpool_one = get_account!(0);
            input_accounts.whirlpool_two = get_account!(1);
            input_accounts.token_mint_input = get_account!(2); // Added
            input_accounts.token_mint_intermediate = get_account!(3); // Added
            input_accounts.token_mint_output = get_account!(4); // Added
            input_accounts.token_program_input = get_account!(5); // Added
            input_accounts.token_program_intermediate = get_account!(6); // Added
            input_accounts.token_program_output = get_account!(7); // Added
            input_accounts.token_owner_account_input = get_account!(8); // Renamed from tokenOwnerAccountOneA
            input_accounts.token_vault_one_a = get_account!(9); // Renamed tokenVaultOneInput -> tokenVaultOneA
            input_accounts.token_vault_one_b = get_account!(10); // Renamed tokenVaultOneIntermediate -> tokenVaultOneB
            input_accounts.token_vault_two_a = get_account!(11); // Renamed tokenVaultTwoIntermediate -> tokenVaultTwoA
            input_accounts.token_vault_two_b = get_account!(12); // Renamed tokenVaultTwoOutput -> tokenVaultTwoB
            input_accounts.token_owner_account_output = get_account!(13); // Renamed from tokenOwnerAccountTwoB
            input_accounts.token_authority = get_account!(14);
            input_accounts.tick_array_one_0 = get_account!(15);
            input_accounts.tick_array_one_1 = get_account!(16);
            input_accounts.tick_array_one_2 = get_account!(17);
            input_accounts.tick_array_two_0 = get_account!(18);
            input_accounts.tick_array_two_1 = get_account!(19);
            input_accounts.tick_array_two_2 = get_account!(20);
            input_accounts.oracle_one = get_account!(21);
            input_accounts.oracle_two = get_account!(22);
            input_accounts.memo_program = get_account!(23); // Added
        }
        "InitializeConfigExtension" => {
            // IDL: config, configExtension, funder, feeAuthority, systemProgram
            input_accounts.config = get_account!(0);
            input_accounts.config_extension = get_account!(1);
            input_accounts.funder = get_account!(2);
            input_accounts.fee_authority = get_account!(3);
            input_accounts.system_program = get_account!(4);
        }
        "SetConfigExtensionAuthority" => {
            // IDL: whirlpoolsConfig, whirlpoolsConfigExtension, configExtensionAuthority, newConfigExtensionAuthority
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.config_extension = get_account!(1); // whirlpoolsConfigExtension -> config_extension
            input_accounts.config_extension_authority = get_account!(2);
            input_accounts.new_config_extension_authority = get_account!(3);
        }
        "SetTokenBadgeAuthority" => {
             // IDL: whirlpoolsConfig, whirlpoolsConfigExtension, configExtensionAuthority, newTokenBadgeAuthority
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.config_extension = get_account!(1); // whirlpoolsConfigExtension -> config_extension
            input_accounts.config_extension_authority = get_account!(2);
            input_accounts.new_token_badge_authority = get_account!(3);
        }
        "InitializeTokenBadge" => {
            // IDL: whirlpoolsConfig, whirlpoolsConfigExtension, tokenBadgeAuthority, tokenMint, 
            //      tokenBadge, funder, systemProgram
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.config_extension = get_account!(1); // whirlpoolsConfigExtension -> config_extension
            input_accounts.token_badge_authority = get_account!(2);
            input_accounts.token_mint = get_account!(3);
            input_accounts.token_badge = get_account!(4);
            input_accounts.funder = get_account!(5);
            input_accounts.system_program = get_account!(6);
        }
        "DeleteTokenBadge" => {
            // IDL: whirlpoolsConfig, whirlpoolsConfigExtension, tokenBadgeAuthority, tokenMint, 
            //      tokenBadge, receiver
            input_accounts.config = get_account!(0); // whirlpoolsConfig -> config
            input_accounts.config_extension = get_account!(1); // whirlpoolsConfigExtension -> config_extension
            input_accounts.token_badge_authority = get_account!(2);
            input_accounts.token_mint = get_account!(3);
            input_accounts.token_badge = get_account!(4);
            input_accounts.receiver = get_account!(5);
        }
        // Instructions missing from provided IDL or current code (like lockPosition)
        // IdlWrite and InitializeAccount seem specific, verify their intended accounts if needed.
        "InitializeAccount" => {
            // IDL: funder, owner, systemProgram (IDL shows no accounts, this might be incorrect)
            // Keeping current implementation for now
            input_accounts.funder = get_account!(0);
            input_accounts.owner = get_account!(1);
            input_accounts.system_program = get_account!(2);
        }
        "IdlWrite" => {
            // IDL: (No accounts listed, might be incorrect)
            // Keeping current implementation: tokenAuthority, whirlpool
            input_accounts.token_authority = get_account!(0);
            input_accounts.whirlpool = get_account!(1);
        }
        _ => { 
            // Log unhandled instruction if necessary
        }
    }

    input_accounts
} 