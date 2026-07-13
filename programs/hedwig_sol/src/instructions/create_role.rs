use anchor_lang::prelude::*;

use crate::{
    constants::{MAX_ROLE_NAME_LEN, ORG_SEED, ROLE_SEED},
    error::HedwigError,
    state::{Org, Role},
};

/// Creates a named role under an org.
///
/// Only the org authority can create roles. The role name is embedded in the
/// PDA seeds so it is immutable after creation. The role admin defaults to the
/// org authority. The current instruction set does not support admin rotation.
///
/// PDA seeds: [ROLE_SEED, org_key, role_name_bytes]
pub fn handler(ctx: Context<CreateRole>, name: String) -> Result<()> {
    let name_bytes = name.as_bytes();
    require!(
        !name.is_empty() && name_bytes.len() <= MAX_ROLE_NAME_LEN,
        HedwigError::InvalidRoleName
    );

    let org = &mut ctx.accounts.org;
    let role = &mut ctx.accounts.role;

    role.org = org.key();
    role.name = name;
    role.admin = org.authority;
    role.member_count = 0;
    role.enabled = true;
    role.bump = ctx.bumps.role;

    org.role_count = org
        .role_count
        .checked_add(1)
        .ok_or(HedwigError::MathOverflow)?;

    emit!(RoleCreated {
        role: role.key(),
        org: role.org,
        name: role.name.clone(),
        admin: role.admin,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreateRole<'info> {
    #[account(
        init,
        payer = authority,
        space = Role::LEN,
        seeds = [ROLE_SEED, org.key().as_ref(), name.as_bytes()],
        bump,
    )]
    pub role: Account<'info, Role>,

    #[account(
        mut,
        seeds = [ORG_SEED, authority.key().as_ref()],
        bump = org.bump,
        has_one = authority @ HedwigError::Unauthorized,
    )]
    pub org: Account<'info, Org>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct RoleCreated {
    pub role: Pubkey,
    pub org: Pubkey,
    pub name: String,
    pub admin: Pubkey,
}
