#![allow(unexpected_cfgs)]
#![allow(deprecated)]


use anchor_lang::prelude::*;

declare_id!("6B1kr7fBRKaeHXSq5qS37g8d5Be9RmeAg2PChbQXRimp");

#[program]
pub mod anchor_vault_q3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
