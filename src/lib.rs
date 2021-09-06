// inside lib.rs, only the following line should be in here
pub mod error;
pub mod instruction;
pub mod process;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;