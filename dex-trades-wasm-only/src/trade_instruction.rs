#[derive(Debug)]
pub struct TradeInstruction {
    pub dapp_address: String,
    pub name: String,
    pub amm: String,
    pub vault_a: String,
    pub vault_b: String,
    pub second_swap_amm: Option<String>,
    pub second_swap_vault_a: Option<String>,
    pub second_swap_vault_b: Option<String>,
}

impl Default for TradeInstruction {
    fn default() -> Self {
        TradeInstruction {
            dapp_address: "".to_string(),
            name: "".to_string(),
            amm: "".to_string(),
            vault_a: "".to_string(),
            vault_b: "".to_string(),
            second_swap_amm: None,
            second_swap_vault_a: None,
            second_swap_vault_b: None,
        }
    }
}
