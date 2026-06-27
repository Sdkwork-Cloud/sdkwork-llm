//! Gateway assembly for sdkwork-llm.

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_application_router, ApplicationAssembly};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
