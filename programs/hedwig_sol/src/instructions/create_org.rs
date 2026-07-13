use anchor_lang::prelude::*;

use crate::{
    constants::{MAX_ORG_NAME_LEN, ORG_SEED},
    error::HedwigError,
    state::Org,
};

/// Creates a new organization PDA.
///
/// The org is the top-level namespace for a set of roles. The signer becomes
/// the org authority and is the only one who can create roles. The current
/// instruction set does not support authority rotation.
///
/// PDA seeds: [ORG_SEED, authority] -- one org per authority is intentional
/// (a deliberate cardinality decision, not an oversight).
pub fn handler(ctx: Context<CreateOrg>, name: String) -> Result<()> {
    require!(
        !name.is_empty() && name.len() <= MAX_ORG_NAME_LEN,
        HedwigError::InvalidOrgName
    );

    let org = &mut ctx.accounts.org;
    org.authority = ctx.accounts.authority.key();
    org.name = name;
    org.role_count = 0;
    org.bump = ctx.bumps.org;

    emit!(OrgCreated {
        org: org.key(),
        authority: org.authority,
        name: org.name.clone(),
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateOrg<'info> {
    #[account(
        init,
        payer = authority,
        space = Org::LEN,
        seeds = [ORG_SEED, authority.key().as_ref()],
        bump,
    )]
    pub org: Account<'info, Org>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct OrgCreated {
    pub org: Pubkey,
    pub authority: Pubkey,
    pub name: String,
}
