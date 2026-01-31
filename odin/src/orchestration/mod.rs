//! Orchestration: request processing, action planning/execution, responsibility routing.
//!
//! - [`RequestProcessor`]: Parse → Route → Coordinate (via ResponsibilityManager or fallback).
//! - [`ActionOrchestrator`]: Plan and execute actions via Thor.
//! - [`ResponsibilityManager`]: Determine and route requests to services (Geri, Thor, etc.).
//! - [`OrchestrationError`]: Structured errors for orchestration flows.

pub mod audit;
pub mod error;
pub mod processor;
pub mod action;
pub mod responsibility;

pub use audit::*;
pub use error::*;
pub use processor::*;
pub use action::*;
pub use responsibility::*;