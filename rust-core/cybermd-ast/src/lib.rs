//! # CyberMD AST - Abstract Syntax Tree Definitions
//!
//! This module defines the Abstract Syntax Tree (AST) for markdown documents.
//! Unlike the Python version which uses inheritance, this Rust implementation
//! uses an enum-based approach which is more idiomatic and type-safe.
//!
//! ## Architecture
//!
//! ```text
//! Raw Markdown → Parser → AST → Analysis/Highlighting
//! ```
//!
//! ## Node Types
//!
//! - Document: Root container
//! - Heading: # to ######
//! - Paragraph: Regular text
//! - CodeBlock: Fenced code blocks
//! - List: Ordered or unordered lists
//! - ListItem: Individual list items
//! - Inline nodes: Bold, Italic, InlineCode, Link

use serde::{Deserialize, Serialize};
use std::fmt;

/// Position in the document (line, column, offset)
///
/// Used for error reporting, navigation, highlighting, and folding.
/// All positions are 0-based.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    /// Line number (0-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
    /// Character offset from start of document (0-based)
    pub offset: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    pub fn zero() -> Self {
        Self {
            line: 0,
            column: 0,
            offset: 0,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "L{}:C{}", self.line, self.column)
    }
}

/// AST Node representing a markdown construct
///
/// This is the main AST structure. Each variant represents a different
/// markdown element. Children and metadata are stored within each variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ASTNode {
    /// Root document node
    Document {
        children: Vec<ASTNode>,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Heading (# to ######)
    Heading {
        level: u8,
        text: String,
        children: Vec<ASTNode>,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Text paragraph
    Paragraph {
        text: String,
        inline_nodes: Vec<ASTNode>,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Fenced code block
    CodeBlock {
        language: String,
        code: String,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// List (ordered or unordered)
    List {
        ordered: bool,
        items: Vec<ASTNode>,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// List item
    ListItem {
        text: String,
        children: Vec<ASTNode>,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Inline code (`code`)
    InlineCode {
        text: String,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Bold text (**text**)
    Bold {
        text: String,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Italic text (*text*)
    Italic {
        text: String,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Link [text](url)
    Link {
        text: String,
        url: String,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Blockquote (> text)
    Blockquote {
        children: Vec<ASTNode>,
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },

    /// Horizontal rule (---, ***, ___)
    HorizontalRule {
        start_pos: Option<Position>,
        end_pos: Option<Position>,
    },
}

impl ASTNode {
    /// Get the node type as a string
    pub fn node_type(&self) -> &'static str {
        match self {
            ASTNode::Document { .. } => "document",
            ASTNode::Heading { .. } => "heading",
            ASTNode::Paragraph { .. } => "paragraph",
            ASTNode::CodeBlock { .. } => "code_block",
            ASTNode::List { .. } => "list",
            ASTNode::ListItem { .. } => "list_item",
            ASTNode::InlineCode { .. } => "inline_code",
            ASTNode::Bold { .. } => "bold",
            ASTNode::Italic { .. } => "italic",
            ASTNode::Link { .. } => "link",
            ASTNode::Blockquote { .. } => "blockquote",
            ASTNode::HorizontalRule { .. } => "horizontal_rule",
        }
    }

    /// Get start position
    pub fn start_pos(&self) -> Option<Position> {
        match self {
            ASTNode::Document { start_pos, .. }
            | ASTNode::Heading { start_pos, .. }
            | ASTNode::Paragraph { start_pos, .. }
            | ASTNode::CodeBlock { start_pos, .. }
            | ASTNode::List { start_pos, .. }
            | ASTNode::ListItem { start_pos, .. }
            | ASTNode::InlineCode { start_pos, .. }
            | ASTNode::Bold { start_pos, .. }
            | ASTNode::Italic { start_pos, .. }
            | ASTNode::Link { start_pos, .. }
            | ASTNode::Blockquote { start_pos, .. }
            | ASTNode::HorizontalRule { start_pos, .. } => *start_pos,
        }
    }

    /// Get end position
    pub fn end_pos(&self) -> Option<Position> {
        match self {
            ASTNode::Document { end_pos, .. }
            | ASTNode::Heading { end_pos, .. }
            | ASTNode::Paragraph { end_pos, .. }
            | ASTNode::CodeBlock { end_pos, .. }
            | ASTNode::List { end_pos, .. }
            | ASTNode::ListItem { end_pos, .. }
            | ASTNode::InlineCode { end_pos, .. }
            | ASTNode::Bold { end_pos, .. }
            | ASTNode::Italic { end_pos, .. }
            | ASTNode::Link { end_pos, .. }
            | ASTNode::Blockquote { end_pos, .. }
            | ASTNode::HorizontalRule { end_pos, .. } => *end_pos,
        }
    }

    /// Get children (if this node type has children)
    pub fn children(&self) -> &[ASTNode] {
        match self {
            ASTNode::Document { children, .. } => children,
            ASTNode::Heading { children, .. } => children,
            ASTNode::Paragraph { inline_nodes, .. } => inline_nodes,
            ASTNode::List { items, .. } => items,
            ASTNode::ListItem { children, .. } => children,
            ASTNode::Blockquote { children, .. } => children,
            _ => &[],
        }
    }

    /// Get mutable children
    pub fn children_mut(&mut self) -> Option<&mut Vec<ASTNode>> {
        match self {
            ASTNode::Document { children, .. } => Some(children),
            ASTNode::Heading { children, .. } => Some(children),
            ASTNode::Paragraph { inline_nodes, .. } => Some(inline_nodes),
            ASTNode::List { items, .. } => Some(items),
            ASTNode::ListItem { children, .. } => Some(children),
            ASTNode::Blockquote { children, .. } => Some(children),
            _ => None,
        }
    }

    /// Add a child node (if this node type supports children)
    pub fn add_child(&mut self, child: ASTNode) -> Result<(), &'static str> {
        match self.children_mut() {
            Some(children) => {
                children.push(child);
                Ok(())
            }
            None => Err("This node type does not support children"),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ASTNode::Document { .. } => write!(f, "Document"),
            ASTNode::Heading { level, text, .. } => {
                let preview = if text.len() > 30 {
                    &text[..30]
                } else {
                    text
                };
                write!(f, "Heading(L{}: {})", level, preview)
            }
            ASTNode::Paragraph { text, .. } => {
                let preview = if text.len() > 30 {
                    &text[..30]
                } else {
                    text
                };
                write!(f, "Paragraph({})", preview)
            }
            ASTNode::CodeBlock { language, code, .. } => {
                let preview = if code.len() > 30 {
                    &code[..30]
                } else {
                    code
                };
                write!(f, "CodeBlock({}: {})", language, preview)
            }
            ASTNode::List { ordered, items, .. } => {
                let list_type = if *ordered { "ordered" } else { "unordered" };
                write!(f, "List({}, {} items)", list_type, items.len())
            }
            ASTNode::ListItem { text, .. } => {
                let preview = if text.len() > 30 {
                    &text[..30]
                } else {
                    text
                };
                write!(f, "ListItem({})", preview)
            }
            ASTNode::InlineCode { text, .. } => write!(f, "InlineCode(`{}`)", text),
            ASTNode::Bold { text, .. } => write!(f, "Bold(**{}**)", text),
            ASTNode::Italic { text, .. } => write!(f, "Italic(*{}*)", text),
            ASTNode::Link { text, url, .. } => write!(f, "Link([{}]({}))", text, url),
            ASTNode::Blockquote { .. } => write!(f, "Blockquote"),
            ASTNode::HorizontalRule { .. } => write!(f, "HorizontalRule"),
        }
    }
}

/// Helper functions for creating AST nodes
impl ASTNode {
    /// Create a new document node
    pub fn new_document() -> Self {
        ASTNode::Document {
            children: Vec::new(),
            start_pos: None,
            end_pos: None,
        }
    }

    /// Create a new heading node
    pub fn new_heading(level: u8, text: String) -> Self {
        ASTNode::Heading {
            level,
            text,
            children: Vec::new(),
            start_pos: None,
            end_pos: None,
        }
    }

    /// Create a new paragraph node
    pub fn new_paragraph(text: String) -> Self {
        ASTNode::Paragraph {
            text,
            inline_nodes: Vec::new(),
            start_pos: None,
            end_pos: None,
        }
    }

    /// Create a new code block node
    pub fn new_code_block(language: String, code: String) -> Self {
        ASTNode::CodeBlock {
            language,
            code,
            start_pos: None,
            end_pos: None,
        }
    }

    /// Create a new list node
    pub fn new_list(ordered: bool) -> Self {
        ASTNode::List {
            ordered,
            items: Vec::new(),
            start_pos: None,
            end_pos: None,
        }
    }

    /// Create a new list item node
    pub fn new_list_item(text: String) -> Self {
        ASTNode::ListItem {
            text,
            children: Vec::new(),
            start_pos: None,
            end_pos: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let pos = Position::new(5, 10, 100);
        assert_eq!(pos.line, 5);
        assert_eq!(pos.column, 10);
        assert_eq!(pos.offset, 100);
    }

    #[test]
    fn test_node_type() {
        let doc = ASTNode::new_document();
        assert_eq!(doc.node_type(), "document");

        let heading = ASTNode::new_heading(1, "Title".to_string());
        assert_eq!(heading.node_type(), "heading");
    }

    #[test]
    fn test_add_child() {
        let mut doc = ASTNode::new_document();
        let heading = ASTNode::new_heading(1, "Title".to_string());

        assert!(doc.add_child(heading).is_ok());
        assert_eq!(doc.children().len(), 1);
    }

    #[test]
    fn test_node_display() {
        let heading = ASTNode::new_heading(2, "Test Heading".to_string());
        let display = format!("{}", heading);
        assert!(display.contains("Heading"));
        assert!(display.contains("L2"));
    }
}
