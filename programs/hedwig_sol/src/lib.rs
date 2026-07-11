pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("H4J9wWhraK2Zvn4o9aFheFVmAf7nfaBNPw3d7w77X1eC");

/// Hedwig: onchain roles primitive for Solana.
///
/// Grant roles, not keys.
#[program]
pub mod hedwig_sol {
    use super::*;

    /// Create a new organization (top-level role namespace).
    pub fn create_org(ctx: Context<CreateOrg>, name: String) -> Result<()> {
        create_org::handler(ctx, name)
    }

    /// Create a named role under an org.
    pub fn create_role(ctx: Context<CreateRole>, name: String) -> Result<()> {
        create_role::handler(ctx, name)
    }

    /// Assign a role to a holder pubkey.
    pub fn assign_role(ctx: Context<AssignRole>, expires_at: i64) -> Result<()> {
        assign_role::handler(ctx, expires_at)
    }

    /// Revoke a role from a holder, closing the member PDA.
    pub fn revoke_role(ctx: Context<RevokeRole>) -> Result<()> {
        revoke_role::handler(ctx)
    }

    /// Verify that a holder currently holds a role (CPI-queryable).
    pub fn check_role(ctx: Context<CheckRole>) -> Result<()> {
        check_role::handler(ctx)
    }

    /// Enable or disable a role. Circuit breaker: halts `check_role` and new
    /// `assign_role` calls for the whole role without revoking individual
    /// members. Only the role admin can call this.
    pub fn set_role_enabled(ctx: Context<SetRoleEnabled>, enabled: bool) -> Result<()> {
        set_role_enabled::handler(ctx, enabled)
    }
}
