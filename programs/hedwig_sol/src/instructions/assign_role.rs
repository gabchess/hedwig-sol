use anchor_lang::prelude::*;

use crate::{
    constants::MEMBER_SEED,
    error::HedwigError,
    state::{Member, Role},
};

/// Assigns a role to a holder.
///
/// The role admin signs this transaction. The holder can be any pubkey:
/// a human wallet, a program-derived address, or an autonomous agent key.
///
/// An optional `expires_at` timestamp (Unix seconds) can be set. Pass 0
/// for no expiry; a non-zero value must be strictly in the future.
///
/// PDA seeds: [MEMBER_SEED, role_key, holder]
pub fn handler(ctx: Context<AssignRole>, expires_at: i64) -> Result<()> {
    let role = &mut ctx.accounts.role;
    require!(role.enabled, HedwigError::RoleDisabled);

    let clock = Clock::get()?;
    require!(
        expires_at == 0 || expires_at > clock.unix_timestamp,
        HedwigError::InvalidExpiration
    );

    let member = &mut ctx.accounts.member;

    member.role = role.key();
    member.holder = ctx.accounts.holder.key();
    member.granted_at = clock.unix_timestamp;
    member.expires_at = expires_at;
    member.bump = ctx.bumps.member;

    role.member_count = role
        .member_count
        .checked_add(1)
        .ok_or(HedwigError::MathOverflow)?;

    emit!(RoleAssigned {
        member: member.key(),
        role: member.role,
        holder: member.holder,
        granted_at: member.granted_at,
        expires_at: member.expires_at,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct AssignRole<'info> {
    #[account(
        init,
        payer = admin,
        space = Member::LEN,
        seeds = [MEMBER_SEED, role.key().as_ref(), holder.key().as_ref()],
        bump,
    )]
    pub member: Account<'info, Member>,

    #[account(
        mut,
        has_one = admin @ HedwigError::NotRoleAdmin,
    )]
    pub role: Account<'info, Role>,

    /// CHECK: The holder is any pubkey being granted the role.
    /// No signature required: the admin signs on the holder's behalf.
    pub holder: UncheckedAccount<'info>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct RoleAssigned {
    pub member: Pubkey,
    pub role: Pubkey,
    pub holder: Pubkey,
    pub granted_at: i64,
    pub expires_at: i64,
}
