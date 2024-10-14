use crate::pb::sf::solana::block_meta::v1::InputAccounts;

pub fn prepare_input_accounts(
    instruction_type: String,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
) -> InputAccounts {
    let input_accounts = populate_input_accounts(account_indices, accounts);

    let mut result = InputAccounts::default();
    match instruction_type.as_str() {
        "ChangeStakeMasterAuthority" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.new_authority = get_account_with(&input_accounts, 1);
            result.stake_master = get_account_with(&input_accounts, 2);
        }
        "ChangeStakeMasterPausedState" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
        }
        "ChangeTotalGeneAllocated" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
        }
        "ChangePoolWeight" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.staking_pool = get_account_with(&input_accounts, 2);
        }
        "CreateGlobalState" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.sgene_minter = get_account_with(&input_accounts, 2);
            result.gene_rewarder = get_account_with(&input_accounts, 3);
            result.mint_sgene = get_account_with(&input_accounts, 4);
            result.gene_mint = get_account_with(&input_accounts, 5);
            result.ata_gene_rewarder = get_account_with(&input_accounts, 6);
            result.associated_token_program = get_account_with(&input_accounts, 7);
            result.system_program = get_account_with(&input_accounts, 8);
            result.token_program = get_account_with(&input_accounts, 9);
            result.rent = get_account_with(&input_accounts, 10);
        }
        "CreateStakingPool" => {
            result.authority = get_account_with(&input_accounts, 0);
            result.pool_token = get_account_with(&input_accounts, 1);
            result.staking_pool = get_account_with(&input_accounts, 2);
            result.stake_master = get_account_with(&input_accounts, 3);
            result.system_program = get_account_with(&input_accounts, 4);
            result.token_program = get_account_with(&input_accounts, 5);
            result.rent = get_account_with(&input_accounts, 6);
        }
        "Stake" => {
            result.user = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.staking_pool = get_account_with(&input_accounts, 2);
            result.pool_token = get_account_with(&input_accounts, 3);
            result.staker = get_account_with(&input_accounts, 4);
            result.ata_user = get_account_with(&input_accounts, 5);
            result.ata_user_sgene = get_account_with(&input_accounts, 6);
            result.ata_vault = get_account_with(&input_accounts, 7);
            result.sgene_minter = get_account_with(&input_accounts, 8);
            result.mint_sgene = get_account_with(&input_accounts, 9);
            result.user_deposit = get_account_with(&input_accounts, 10);
            result.user_re_deposit = get_account_with(&input_accounts, 11);
            result.associated_token_program = get_account_with(&input_accounts, 12);
            result.token_program = get_account_with(&input_accounts, 13);
            result.system_program = get_account_with(&input_accounts, 14);
            result.rent = get_account_with(&input_accounts, 15);
        }
        "ReLockDeposit" => {
            result.user = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.staking_pool = get_account_with(&input_accounts, 2);
            result.staker = get_account_with(&input_accounts, 3);
            result.sgene_minter = get_account_with(&input_accounts, 4);
            result.mint_sgene = get_account_with(&input_accounts, 5);
            result.ata_user_sgene = get_account_with(&input_accounts, 6);
            result.user_deposit = get_account_with(&input_accounts, 7);
            result.user_re_deposit = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.associated_token_program = get_account_with(&input_accounts, 10);
            result.rent = get_account_with(&input_accounts, 11);
            result.token_program = get_account_with(&input_accounts, 12);
        }
        "Withdraw" => {
            result.user = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.staking_pool = get_account_with(&input_accounts, 2);
            result.pool_token = get_account_with(&input_accounts, 3);
            result.staker = get_account_with(&input_accounts, 4);
            result.ata_user = get_account_with(&input_accounts, 5);
            result.ata_vault = get_account_with(&input_accounts, 6);
            result.sgene_minter = get_account_with(&input_accounts, 7);
            result.ata_user_sgene = get_account_with(&input_accounts, 8);
            result.mint_sgene = get_account_with(&input_accounts, 9);
            result.gene_rewarder = get_account_with(&input_accounts, 10);
            result.ata_gene_rewarder = get_account_with(&input_accounts, 11);
            result.user_re_deposit = get_account_with(&input_accounts, 12);
            result.user_deposit = get_account_with(&input_accounts, 13);
            result.associated_token_program = get_account_with(&input_accounts, 14);
            result.token_program = get_account_with(&input_accounts, 15);
            result.system_program = get_account_with(&input_accounts, 16);
            result.rent = get_account_with(&input_accounts, 17);
        }
        "WithdrawAsSgene" => {
            result.user = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.sgene_minter = get_account_with(&input_accounts, 3);
            result.ata_user_sgene = get_account_with(&input_accounts, 4);
            result.mint_sgene = get_account_with(&input_accounts, 5);
            result.user_deposit = get_account_with(&input_accounts, 6);
            result.associated_token_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.rent = get_account_with(&input_accounts, 10);
        }
        "ClaimRewards" => {
            result.user = get_account_with(&input_accounts, 0);
            result.stake_master = get_account_with(&input_accounts, 1);
            result.staker = get_account_with(&input_accounts, 2);
            result.sgene_minter = get_account_with(&input_accounts, 3);
            result.mint_sgene = get_account_with(&input_accounts, 4);
            result.ata_user_sgene = get_account_with(&input_accounts, 5);
            result.user_re_deposit = get_account_with(&input_accounts, 6);
            result.associated_token_program = get_account_with(&input_accounts, 7);
            result.token_program = get_account_with(&input_accounts, 8);
            result.system_program = get_account_with(&input_accounts, 9);
            result.rent = get_account_with(&input_accounts, 10);
        }
        "PendingYieldRewards" => {
            result.stake_master = get_account_with(&input_accounts, 0);
            result.staker = get_account_with(&input_accounts, 1);
        }
        "CurrentGenesPerSecond" => {
            result.stake_master = get_account_with(&input_accounts, 0);
        }
        "GetVotingWeight" => {
            result.deposit = get_account_with(&input_accounts, 0);
        }
        _ => {}
    }

    return result;
}

fn get_account_with(accounts: &Vec<String>, index: usize) -> Option<String> {
    let mut result: Option<String> = None;
    let account = accounts.get(index);
    if account.is_some() {
        result = Some(account.unwrap().to_string());
    }
    return result;
}

fn populate_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}
