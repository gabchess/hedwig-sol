use anchor_lang::prelude::*;

/// An organization is the top-level namespace for a set of roles.
/// PDA seeds: ["org", authority]
/// Authority is the pubkey that controls this org.
#[account]
#[derive(Default)]
pub struct Org {
    /// The account that created this org and can manage its roles.
    pub authority: Pubkey,
    /// A human-readable label for this org (off-chain convenience, opaque on-chain).
    pub name: String,
    /// Number of roles created under this org.
    pub role_count: u64,
    /// PDA bump.
    pub bump: u8,
}

impl Org {
    /// Fixed space: 8 (discriminator) + 32 (authority) + 4+64 (name) + 8 (role_count) + 1 (bump)
    pub const LEN: usize = 8 + 32 + (4 + 64) + 8 + 1;
}

/// A named role under an org.
/// PDA seeds: ["role", org_key, role_name_bytes (up to 32 bytes)]
#[account]
#[derive(Default)]
pub struct Role {
    /// The org this role belongs to.
    pub org: Pubkey,
    /// Name of the role (up to 32 bytes, UTF-8).
    pub name: String,
    /// The authority that can assign / revoke this role.
    /// Defaults to the org authority but can be delegated.
    pub admin: Pubkey,
    /// Number of active members holding this role.
    pub member_count: u64,
    /// Whether this role is enabled. Disabled roles fail CPI checks.
    pub enabled: bool,
    /// PDA bump.
    pub bump: u8,
}

impl Role {
    /// Fixed space: 8 + 32 (org) + (4+32) (name) + 32 (admin) + 8 (member_count) + 1 (enabled) + 1 (bump)
    pub const LEN: usize = 8 + 32 + (4 + 32) + 32 + 8 + 1 + 1;
}

/// Membership record: proof that a pubkey holds a given role.
/// PDA seeds: ["member", role_key, holder]
#[account]
pub struct Member {
    /// The role PDA this membership is for.
    pub role: Pubkey,
    /// The pubkey (wallet, program, or agent key) holding this role.
    pub holder: Pubkey,
    /// Unix timestamp when this membership was granted. 0 = no expiry tracked.
    pub granted_at: i64,
    /// Optional expiry (Unix timestamp). 0 = no expiry.
    pub expires_at: i64,
    /// PDA bump.
    pub bump: u8,
}

impl Member {
    /// Fixed space: 8 + 32 (role) + 32 (holder) + 8 (granted_at) + 8 (expires_at) + 1 (bump)
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 1;
}
