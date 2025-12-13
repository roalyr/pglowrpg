// [Revision: v0.1] [Path: libs/lib_oracle/src/lib.rs] [Date: 2025-12-13]
// Description: Oracle system for narrative prompts and random tables

pub mod tables;
pub mod oracle;

pub use oracle::Oracle;
pub use tables::{OracleTable, OracleEntry, YesNoResult};
