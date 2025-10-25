//! MCP .NET Context Server
//! 
//! A specialized Model Context Protocol (MCP) server for .NET 10 and Blazor Server
//! that provides intelligent context analysis and code pattern training.

pub mod analyzer;
pub mod config;
pub mod context;
pub mod mcp;
pub mod training;
pub mod types;
pub mod utils;

pub use config::Config;
pub use mcp::Server;
