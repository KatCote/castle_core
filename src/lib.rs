//! # CastleCore
//!
//! It's my crate!

// Public constants
pub(crate) const CC_VER: &str = "v0.0.5";

// Heavy lines
pub(crate) const LU_CORNER: char = '┏';
pub(crate) const LD_CORNER: char = '┗';
pub(crate) const RU_CORNER: char = '┓';
pub(crate) const RD_CORNER: char = '┛';
pub(crate) const UD_LINE: char = '━';
pub(crate) const LR_LINE: char = '┃';

/// A temporary module for debugging functions.
pub mod functions;

/// Return current CastleCore version
pub fn cc_ver() -> String {
    (&CC_VER).to_string()
}
