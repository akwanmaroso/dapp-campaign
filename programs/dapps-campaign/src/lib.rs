use anchor_lang::prelude::*;

declare_id!("9k1Z1sPcHRBNpU5rAcv1EfLA3eK6dN7CX21bv2Em6Zfg");

#[program]
pub mod dapps_campaign {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        description: String,
        target: u64,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        campaign.owner = *ctx.accounts.signer.key;
        campaign.name = name;
        campaign.description = description;
        campaign.target = target;
        campaign.is_completed = false;
        campaign.amount_donated = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init,payer = signer, space=8+Campaign::INIT_SPACE)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub owner: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(100)]
    pub description: String,
    pub target: u64,
    pub amount_donated: u64,
    pub is_completed: bool,
}
