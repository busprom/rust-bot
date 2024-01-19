pub mod utils;
pub mod processor;
pub mod error;
pub mod instruction;
pub mod types;
pub mod token;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;



pub const ADMIN_ID: &str = "Euy2YtCb7sQvFQu3ohS1eqe72g6yRqqEu1eZVwg9oqUG";