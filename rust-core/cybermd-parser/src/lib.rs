//! # CyberMD Parser - Markdown to AST Parser
//!
//! This module implements a markdown parser that builds an Abstract Syntax Tree.
//!
//! ## Architecture
//!
//! ```text
//! Text → Tokenizer → Parser → AST
//! ```
//!
//! ## Two-Stage Parsing
//!
//! 1. **Tokenization**: Convert raw text into tokens (HASH, TEXT, NEWLINE, etc.)
//! 2. **Parsing**: Convert tokens into AST nodes
//!
//! This separation makes the code cleaner and more maintainable.

mod tokenizer;
mod parser;

pub use tokenizer::{Token, TokenType, Tokenizer};
pub use parser::MarkdownParser;

// Re-export AST types for convenience
pub use cybermd_ast::{ASTNode, Position};
