use anchor_lang::prelude::*;

declare_id!("9k1Z1sPcHRBNpU5rAcv1EfLA3eK6dN7CX21bv2Em6Zfg");

#[program]
pub mod dapps_campaign {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
