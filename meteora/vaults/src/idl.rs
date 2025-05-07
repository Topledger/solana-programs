#![allow(dead_code)]
#![allow(unused_imports)]
pub use accounts_data::*;
use anchor_lang::{self, AnchorDeserialize};
use anyhow;
use borsh::{BorshSchema, BorshSerialize};
pub use ix_data::*;
use std::convert::TryInto;
use std::mem;
pub use typedefs::*;
pub mod typedefs {
    use anchor_lang::prelude::*;
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct VaultBumps {
        pub vault_bump: u8,
        pub token_vault_bump: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct StrategyBumps {
        pub strategy_index: u8,
        pub other_bumps: [u8; 10],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default)]
    pub struct LockedProfitTracker {
        pub last_updated_locked_profit: u64,
        pub last_report: u64,
        pub locked_profit_degradation: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy)]
    pub enum StrategyType {
        PortFinanceWithoutLM,
        PortFinanceWithLM,
        SolendWithoutLM,
        Mango,
        SolendWithLM,
        ApricotWithoutLM,
        Francium,
        Tulip,
        Vault,
        Drift,
        Frakt,
        Marginfi,
    }
    impl Default for StrategyType {
        fn default() -> Self {
            Self::PortFinanceWithoutLM
        }
    }
}
pub mod accounts_data {
    use anchor_lang::prelude::*;
    #[derive(Debug)]
    pub struct InitializeAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub payer: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub tokenMint: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub rent: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub systemProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct EnableVaultAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct SetOperatorAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub operator: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct InitializeStrategyAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategyProgram: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub reserve: anchor_lang::prelude::Pubkey,
        pub collateralVault: anchor_lang::prelude::Pubkey,
        pub collateralMint: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub systemProgram: anchor_lang::prelude::Pubkey,
        pub rent: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct RemoveStrategyAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub strategyProgram: anchor_lang::prelude::Pubkey,
        pub collateralVault: anchor_lang::prelude::Pubkey,
        pub reserve: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub feeVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct RemoveStrategy2Accounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub strategyProgram: anchor_lang::prelude::Pubkey,
        pub collateralVault: anchor_lang::prelude::Pubkey,
        pub reserve: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub tokenAdminAdvancePayment: anchor_lang::prelude::Pubkey,
        pub tokenVaultAdvancePayment: anchor_lang::prelude::Pubkey,
        pub feeVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct CollectDustAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub tokenAdmin: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct AddStrategyAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub admin: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct DepositStrategyAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub feeVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub strategyProgram: anchor_lang::prelude::Pubkey,
        pub collateralVault: anchor_lang::prelude::Pubkey,
        pub reserve: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub operator: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct WithdrawStrategyAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub feeVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub strategyProgram: anchor_lang::prelude::Pubkey,
        pub collateralVault: anchor_lang::prelude::Pubkey,
        pub reserve: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub operator: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct Withdraw2Accounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub userToken: anchor_lang::prelude::Pubkey,
        pub userLp: anchor_lang::prelude::Pubkey,
        pub user: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct DepositAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub userToken: anchor_lang::prelude::Pubkey,
        pub userLp: anchor_lang::prelude::Pubkey,
        pub user: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct WithdrawAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub userToken: anchor_lang::prelude::Pubkey,
        pub userLp: anchor_lang::prelude::Pubkey,
        pub user: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
    #[derive(Debug)]
    pub struct WithdrawDirectlyFromStrategyAccounts {
        pub vault: anchor_lang::prelude::Pubkey,
        pub strategy: anchor_lang::prelude::Pubkey,
        pub reserve: anchor_lang::prelude::Pubkey,
        pub strategyProgram: anchor_lang::prelude::Pubkey,
        pub collateralVault: anchor_lang::prelude::Pubkey,
        pub tokenVault: anchor_lang::prelude::Pubkey,
        pub lpMint: anchor_lang::prelude::Pubkey,
        pub feeVault: anchor_lang::prelude::Pubkey,
        pub userToken: anchor_lang::prelude::Pubkey,
        pub userLp: anchor_lang::prelude::Pubkey,
        pub user: anchor_lang::prelude::Pubkey,
        pub tokenProgram: anchor_lang::prelude::Pubkey,
        pub remaining: Vec<anchor_lang::prelude::Pubkey>,
    }
}
pub mod ix_data {
    use super::*;
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct InitializeArgs {}
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct EnableVaultArgs {
        pub enabled: u8,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct SetOperatorArgs {}
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct InitializeStrategyArgs {
        pub bumps: typedefs::StrategyBumps,
        pub strategy_type: typedefs::StrategyType,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct RemoveStrategyArgs {}
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct RemoveStrategy2Args {
        pub max_admin_pay_amount: u64,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct CollectDustArgs {}
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct AddStrategyArgs {}
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct DepositStrategyArgs {
        pub amount: u64,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct WithdrawStrategyArgs {
        pub amount: u64,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct Withdraw2Args {
        pub unmint_amount: u64,
        pub min_out_amount: u64,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct DepositArgs {
        pub token_amount: u64,
        pub minimum_lp_token_amount: u64,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct WithdrawArgs {
        pub unmint_amount: u64,
        pub min_out_amount: u64,
    }
    #[derive(anchor_lang :: AnchorDeserialize, Debug)]
    pub struct WithdrawDirectlyFromStrategyArgs {
        pub unmint_amount: u64,
        pub min_out_amount: u64,
    }
}
#[derive(Debug)]
pub enum Instruction {
    Initialize {
        accounts: InitializeAccounts,
        args: InitializeArgs,
    },
    EnableVault {
        accounts: EnableVaultAccounts,
        args: EnableVaultArgs,
    },
    SetOperator {
        accounts: SetOperatorAccounts,
        args: SetOperatorArgs,
    },
    InitializeStrategy {
        accounts: InitializeStrategyAccounts,
        args: InitializeStrategyArgs,
    },
    RemoveStrategy {
        accounts: RemoveStrategyAccounts,
        args: RemoveStrategyArgs,
    },
    RemoveStrategy2 {
        accounts: RemoveStrategy2Accounts,
        args: RemoveStrategy2Args,
    },
    CollectDust {
        accounts: CollectDustAccounts,
        args: CollectDustArgs,
    },
    AddStrategy {
        accounts: AddStrategyAccounts,
        args: AddStrategyArgs,
    },
    DepositStrategy {
        accounts: DepositStrategyAccounts,
        args: DepositStrategyArgs,
    },
    WithdrawStrategy {
        accounts: WithdrawStrategyAccounts,
        args: WithdrawStrategyArgs,
    },
    Withdraw2 {
        accounts: Withdraw2Accounts,
        args: Withdraw2Args,
    },
    Deposit {
        accounts: DepositAccounts,
        args: DepositArgs,
    },
    Withdraw {
        accounts: WithdrawAccounts,
        args: WithdrawArgs,
    },
    WithdrawDirectlyFromStrategy {
        accounts: WithdrawDirectlyFromStrategyAccounts,
        args: WithdrawDirectlyFromStrategyArgs,
    },
}
impl Instruction {
    pub fn decode(
        account_keys: &[anchor_lang::prelude::Pubkey],
        data: &[u8],
    ) -> anyhow::Result<Self> {
        if data.len() < 8 {
            anyhow::bail!("Data too short for discriminator: {}", data.len());
        }
        let (disc_slice, rest) = data.split_at(8);
        let disc: [u8; 8] = disc_slice.try_into().unwrap();
        match disc {
            [175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8] => {
                let mut data_slice = rest;
                let args = InitializeArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let payer = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(payer)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let tokenMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenMint)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let rent = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(rent)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let systemProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(systemProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = InitializeAccounts {
                    vault,
                    payer,
                    tokenVault,
                    tokenMint,
                    lpMint,
                    rent,
                    tokenProgram,
                    systemProgram,
                    remaining,
                };
                return Ok(Instruction::Initialize { accounts, args });
            }
            [145u8, 82u8, 241u8, 156u8, 26u8, 154u8, 233u8, 211u8] => {
                let mut data_slice = rest;
                let args = EnableVaultArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = EnableVaultAccounts {
                    vault,
                    admin,
                    remaining,
                };
                return Ok(Instruction::EnableVault { accounts, args });
            }
            [238u8, 153u8, 101u8, 169u8, 243u8, 131u8, 36u8, 1u8] => {
                let mut data_slice = rest;
                let args = SetOperatorArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let operator = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(operator)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = SetOperatorAccounts {
                    vault,
                    operator,
                    admin,
                    remaining,
                };
                return Ok(Instruction::SetOperator { accounts, args });
            }
            [208u8, 119u8, 144u8, 145u8, 178u8, 57u8, 105u8, 252u8] => {
                let mut data_slice = rest;
                let args = InitializeStrategyArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategyProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategyProgram)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let reserve = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(reserve)))
                })?;
                let collateralVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralVault)))
                })?;
                let collateralMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralMint)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let systemProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(systemProgram)))
                })?;
                let rent = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(rent)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = InitializeStrategyAccounts {
                    vault,
                    strategyProgram,
                    strategy,
                    reserve,
                    collateralVault,
                    collateralMint,
                    admin,
                    systemProgram,
                    rent,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::InitializeStrategy { accounts, args });
            }
            [185u8, 238u8, 33u8, 91u8, 134u8, 210u8, 97u8, 26u8] => {
                let mut data_slice = rest;
                let args = RemoveStrategyArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let strategyProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategyProgram)))
                })?;
                let collateralVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralVault)))
                })?;
                let reserve = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(reserve)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let feeVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(feeVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = RemoveStrategyAccounts {
                    vault,
                    strategy,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenVault,
                    feeVault,
                    lpMint,
                    tokenProgram,
                    admin,
                    remaining,
                };
                return Ok(Instruction::RemoveStrategy { accounts, args });
            }
            [138u8, 104u8, 208u8, 148u8, 126u8, 35u8, 195u8, 14u8] => {
                let mut data_slice = rest;
                let args = RemoveStrategy2Args::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let strategyProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategyProgram)))
                })?;
                let collateralVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralVault)))
                })?;
                let reserve = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(reserve)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let tokenAdminAdvancePayment = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tokenAdminAdvancePayment)
                    ))
                })?;
                let tokenVaultAdvancePayment = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!(
                        "Missing account: ",
                        stringify!(tokenVaultAdvancePayment)
                    ))
                })?;
                let feeVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(feeVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = RemoveStrategy2Accounts {
                    vault,
                    strategy,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenVault,
                    tokenAdminAdvancePayment,
                    tokenVaultAdvancePayment,
                    feeVault,
                    lpMint,
                    tokenProgram,
                    admin,
                    remaining,
                };
                return Ok(Instruction::RemoveStrategy2 { accounts, args });
            }
            [246u8, 149u8, 21u8, 82u8, 160u8, 74u8, 254u8, 240u8] => {
                let mut data_slice = rest;
                let args = CollectDustArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let tokenAdmin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenAdmin)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = CollectDustAccounts {
                    vault,
                    tokenVault,
                    tokenAdmin,
                    admin,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::CollectDust { accounts, args });
            }
            [64u8, 123u8, 127u8, 227u8, 192u8, 234u8, 198u8, 20u8] => {
                let mut data_slice = rest;
                let args = AddStrategyArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let admin = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(admin)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = AddStrategyAccounts {
                    vault,
                    strategy,
                    admin,
                    remaining,
                };
                return Ok(Instruction::AddStrategy { accounts, args });
            }
            [246u8, 82u8, 57u8, 226u8, 131u8, 222u8, 253u8, 249u8] => {
                let mut data_slice = rest;
                let args = DepositStrategyArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let feeVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(feeVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let strategyProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategyProgram)))
                })?;
                let collateralVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralVault)))
                })?;
                let reserve = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(reserve)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let operator = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(operator)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = DepositStrategyAccounts {
                    vault,
                    strategy,
                    tokenVault,
                    feeVault,
                    lpMint,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenProgram,
                    operator,
                    remaining,
                };
                return Ok(Instruction::DepositStrategy { accounts, args });
            }
            [31u8, 45u8, 162u8, 5u8, 193u8, 217u8, 134u8, 188u8] => {
                let mut data_slice = rest;
                let args = WithdrawStrategyArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let feeVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(feeVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let strategyProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategyProgram)))
                })?;
                let collateralVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralVault)))
                })?;
                let reserve = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(reserve)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let operator = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(operator)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = WithdrawStrategyAccounts {
                    vault,
                    strategy,
                    tokenVault,
                    feeVault,
                    lpMint,
                    strategyProgram,
                    collateralVault,
                    reserve,
                    tokenProgram,
                    operator,
                    remaining,
                };
                return Ok(Instruction::WithdrawStrategy { accounts, args });
            }
            [80u8, 6u8, 111u8, 73u8, 174u8, 211u8, 66u8, 132u8] => {
                let mut data_slice = rest;
                let args = Withdraw2Args::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let userToken = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userToken)))
                })?;
                let userLp = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userLp)))
                })?;
                let user = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(user)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = Withdraw2Accounts {
                    vault,
                    tokenVault,
                    lpMint,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Withdraw2 { accounts, args });
            }
            [242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8] => {
                let mut data_slice = rest;
                let args = DepositArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let userToken = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userToken)))
                })?;
                let userLp = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userLp)))
                })?;
                let user = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(user)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = DepositAccounts {
                    vault,
                    tokenVault,
                    lpMint,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Deposit { accounts, args });
            }
            [183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8] => {
                let mut data_slice = rest;
                let args = WithdrawArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let userToken = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userToken)))
                })?;
                let userLp = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userLp)))
                })?;
                let user = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(user)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = WithdrawAccounts {
                    vault,
                    tokenVault,
                    lpMint,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::Withdraw { accounts, args });
            }
            [201u8, 141u8, 146u8, 46u8, 173u8, 116u8, 198u8, 22u8] => {
                let mut data_slice = rest;
                let args = WithdrawDirectlyFromStrategyArgs::deserialize(&mut data_slice)?;
                let mut keys = account_keys.iter();
                let vault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(vault)))
                })?;
                let strategy = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategy)))
                })?;
                let reserve = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(reserve)))
                })?;
                let strategyProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(strategyProgram)))
                })?;
                let collateralVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(collateralVault)))
                })?;
                let tokenVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenVault)))
                })?;
                let lpMint = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(lpMint)))
                })?;
                let feeVault = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(feeVault)))
                })?;
                let userToken = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userToken)))
                })?;
                let userLp = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(userLp)))
                })?;
                let user = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(user)))
                })?;
                let tokenProgram = *keys.next().ok_or_else(|| {
                    anyhow::anyhow!(concat!("Missing account: ", stringify!(tokenProgram)))
                })?;
                let remaining = keys.cloned().collect();
                let accounts = WithdrawDirectlyFromStrategyAccounts {
                    vault,
                    strategy,
                    reserve,
                    strategyProgram,
                    collateralVault,
                    tokenVault,
                    lpMint,
                    feeVault,
                    userToken,
                    userLp,
                    user,
                    tokenProgram,
                    remaining,
                };
                return Ok(Instruction::WithdrawDirectlyFromStrategy { accounts, args });
            }
            _ => anyhow::bail!("Unknown discriminator: {:?}", disc),
        }
    }
}
