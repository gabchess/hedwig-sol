use anchor_lang::prelude::*;

use crate::{
    constants::MEMBER_SEED,
    error::HedwigError,
    state::{Member, Role},
};

/// Revokes a role from a holder.
///
/// The role admin signs this transaction. The member PDA is closed and rent
/// is returned to the admin. The role member_count is decremented.
///
/// Revocation is immediate and irreversible. To re-grant the role, call
/// assign_role again.
pub fn handler(ctx: Context<RevokeRole>) -> Result<()> {
    let role = &mut ctx.accounts.role;
    role.member_count = role
        .member_count
        .checked_sub(1)
        .ok_or(HedwigError::MathOverflow)?;

    emit!(RoleRevoked {
        role: role.key(),
        holder: ctx.accounts.member.holder,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct RevokeRole<'info> {
    #[account(
        mut,
        close = admin,
        seeds = [MEMBER_SEED, role.key().as_ref(), member.holder.as_ref()],
        bump = member.bump,
        has_one = role,
    )]
    pub member: Account<'info, Member>,

    #[account(
        mut,
        has_one = admin @ HedwigError::NotRoleAdmin,
    )]
    pub role: Account<'info, Role>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct RoleRevoked {
    pub role: Pubkey,
    pub holder: Pubkey,
}
