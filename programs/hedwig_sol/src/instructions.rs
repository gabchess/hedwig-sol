pub mod assign_role;
pub mod check_role;
pub mod create_org;
pub mod create_role;
pub mod revoke_role;

// Glob re-exports are required by the Anchor macro for CPI client generation.
// The `handler` name collision across modules is harmless at the crate level
// because callers use the fully-qualified module path (e.g. create_org::handler).
#[allow(ambiguous_glob_reexports)]
pub use assign_role::*;
#[allow(ambiguous_glob_reexports)]
pub use check_role::*;
#[allow(ambiguous_glob_reexports)]
pub use create_org::*;
#[allow(ambiguous_glob_reexports)]
pub use create_role::*;
#[allow(ambiguous_glob_reexports)]
pub use revoke_role::*;
