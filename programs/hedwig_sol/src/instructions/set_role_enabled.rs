use anchor_lang::prelude::*;

use crate::{error::HedwigError, state::Role};

/// Enables or disables a role.
///
/// This is the circuit breaker described in THREAT-MODEL.md: when a
/// role is disabled, `check_role` fails for every holder of that role and
/// `assign_role` refuses new members, without requiring the admin to revoke
/// each Member PDA individually. The role admin signs this transaction.
pub fn handler(ctx: Context<SetRoleEnabled>, enabled: bool) -> Result<()> {
    let role = &mut ctx.accounts.role;
    role.enabled = enabled;

    emit!(RoleEnabledSet {
        role: role.key(),
        enabled: role.enabled,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SetRoleEnabled<'info> {
    #[account(
        mut,
        has_one = admin @ HedwigError::NotRoleAdmin,
    )]
    pub role: Account<'info, Role>,

    pub admin: Signer<'info>,
}

#[event]
pub struct RoleEnabledSet {
    pub role: Pubkey,
    pub enabled: bool,
}
