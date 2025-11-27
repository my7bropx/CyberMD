//! # CyberMD Highlighter - Semantic Syntax Highlighting
//!
//! This module provides semantic highlighting for markdown documents.
//! Unlike regex-based highlighting, it uses the AST to understand context.
//!
//! ## Features
//!
//! - AST-based highlighting (context-aware)
//! - Color themes (dark, light)
//! - Highlight ranges with token types
//! - Tree-sitter integration for code blocks

use cybermd_ast::{ASTNode, Position};
use std::collections::HashMap;

/// Token type for highlighting
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Paragraph,
    CodeBlock,
    InlineCode,
    Bold,
    Italic,
    Link,
    ListMarker,
}

/// A highlight range with type and position
#[derive(Debug, Clone)]
pub struct HighlightRange {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub token_type: TokenType,
}

impl HighlightRange {
    pub fn new(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
        token_type: TokenType,
    ) -> Self {
        Self {
            start_line,
            start_col,
            end_line,
            end_col,
            token_type,
        }
    }
}

/// Color scheme for highlighting
#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub name: String,
    colors: HashMap<String, String>,
}

impl ColorTheme {
    /// Create a new dark theme
    pub fn dark() -> Self {
        let mut colors = HashMap::new();

        // Headings (different color per level)
        colors.insert("heading1".to_string(), "#569CD6".to_string()); // Blue
        colors.insert("heading2".to_string(), "#4EC9B0".to_string()); // Teal
        colors.insert("heading3".to_string(), "#DCDCAA".to_string()); // Yellow
        colors.insert("heading4".to_string(), "#9CDCFE".to_string()); // Light blue
        colors.insert("heading5".to_string(), "#C586C0".to_string()); // Purple
        colors.insert("heading6".to_string(), "#CE9178".to_string()); // Orange

        // Content
        colors.insert("paragraph".to_string(), "#D4D4D4".to_string()); // Light gray
        colors.insert("code_block".to_string(), "#CE9178".to_string()); // Orange
        colors.insert("code_block_bg".to_string(), "#1E1E1E".to_string()); // Dark bg
        colors.insert("inline_code".to_string(), "#CE9178".to_string());
        colors.insert("list_marker".to_string(), "#4EC9B0".to_string());

        // Emphasis
        colors.insert("bold".to_string(), "#569CD6".to_string());
        colors.insert("italic".to_string(), "#C586C0".to_string());
        colors.insert("link".to_string(), "#4EC9B0".to_string());

        Self {
            name: "dark".to_string(),
            colors,
        }
    }

    /// Create a new light theme
    pub fn light() -> Self {
        let mut colors = HashMap::new();

        colors.insert("heading1".to_string(), "#0000FF".to_string()); // Blue
        colors.insert("heading2".to_string(), "#008080".to_string()); // Teal
        colors.insert("heading3".to_string(), "#808000".to_string()); // Olive
        colors.insert("heading4".to_string(), "#000080".to_string()); // Navy
        colors.insert("heading5".to_string(), "#800080".to_string()); // Purple
        colors.insert("heading6".to_string(), "#FF8C00".to_string()); // Dark orange

        colors.insert("paragraph".to_string(), "#000000".to_string());
        colors.insert("code_block".to_string(), "#A31515".to_string());
        colors.insert("code_block_bg".to_string(), "#F5F5F5".to_string());
        colors.insert("inline_code".to_string(), "#A31515".to_string());
        colors.insert("list_marker".to_string(), "#008080".to_string());

        colors.insert("bold".to_string(), "#0000FF".to_string());
        colors.insert("italic".to_string(), "#800080".to_string());
        colors.insert("link".to_string(), "#0000EE".to_string());

        Self {
            name: "light".to_string(),
            colors,
        }
    }

    /// Get color for an element
    pub fn get_color(&self, element: &str) -> Option<&str> {
        self.colors.get(element).map(|s| s.as_str())
    }
}

/// Semantic highlighter that produces highlighting information from AST
pub struct SemanticHighlighter {
    theme: ColorTheme,
}

impl SemanticHighlighter {
    /// Create new highlighter with theme
    pub fn new(theme: ColorTheme) -> Self {
        Self { theme }
    }

    /// Create with dark theme
    pub fn dark() -> Self {
        Self::new(ColorTheme::dark())
    }

    /// Create with light theme
    pub fn light() -> Self {
        Self::new(ColorTheme::light())
    }

    /// Generate highlight ranges from AST
    pub fn highlight(&self, ast: &ASTNode) -> Vec<HighlightRange> {
        let mut ranges = Vec::new();
        self.collect_ranges(ast, &mut ranges);
        ranges
    }

    fn collect_ranges(&self, node: &ASTNode, ranges: &mut Vec<HighlightRange>) {
        match node {
            ASTNode::Heading {
                level,
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                let token_type = match level {
                    1 => TokenType::Heading1,
                    2 => TokenType::Heading2,
                    3 => TokenType::Heading3,
                    4 => TokenType::Heading4,
                    5 => TokenType::Heading5,
                    _ => TokenType::Heading6,
                };

                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    token_type,
                ));
            }

            ASTNode::CodeBlock {
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    TokenType::CodeBlock,
                ));
            }

            ASTNode::Paragraph {
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    TokenType::Paragraph,
                ));
            }

            ASTNode::InlineCode {
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    TokenType::InlineCode,
                ));
            }

            ASTNode::Bold {
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    TokenType::Bold,
                ));
            }

            ASTNode::Italic {
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    TokenType::Italic,
                ));
            }

            ASTNode::Link {
                start_pos: Some(start),
                end_pos: Some(end),
                ..
            } => {
                ranges.push(HighlightRange::new(
                    start.line,
                    start.column,
                    end.line,
                    end.column,
                    TokenType::Link,
                ));
            }

            _ => {}
        }

        // Recursively process children
        for child in node.children() {
            self.collect_ranges(child, ranges);
        }
    }

    /// Get color for token type
    pub fn get_token_color(&self, token_type: &TokenType) -> Option<&str> {
        let key = match token_type {
            TokenType::Heading1 => "heading1",
            TokenType::Heading2 => "heading2",
            TokenType::Heading3 => "heading3",
            TokenType::Heading4 => "heading4",
            TokenType::Heading5 => "heading5",
            TokenType::Heading6 => "heading6",
            TokenType::Paragraph => "paragraph",
            TokenType::CodeBlock => "code_block",
            TokenType::InlineCode => "inline_code",
            TokenType::Bold => "bold",
            TokenType::Italic => "italic",
            TokenType::Link => "link",
            TokenType::ListMarker => "list_marker",
        };

        self.theme.get_color(key)
    }
}

impl Default for SemanticHighlighter {
    fn default() -> Self {
        Self::dark()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cybermd_ast::ASTNode;

    #[test]
    fn test_dark_theme() {
        let theme = ColorTheme::dark();
        assert_eq!(theme.get_color("heading1"), Some("#569CD6"));
        assert_eq!(theme.get_color("code_block"), Some("#CE9178"));
    }

    #[test]
    fn test_light_theme() {
        let theme = ColorTheme::light();
        assert_eq!(theme.get_color("heading1"), Some("#0000FF"));
    }

    #[test]
    fn test_highlight_heading() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));

        let highlighter = SemanticHighlighter::dark();
        let ranges = highlighter.highlight(&doc);

        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].token_type, TokenType::Heading1);
    }

    #[test]
    fn test_multiple_nodes() {
        let mut doc = ASTNode::new_document();
        let _ = doc.add_child(ASTNode::new_heading(1, "Title".to_string()));
        let _ = doc.add_child(ASTNode::new_paragraph("Text".to_string()));
        let _ = doc.add_child(ASTNode::new_code_block("rust".to_string(), "code".to_string()));

        let highlighter = SemanticHighlighter::dark();
        let ranges = highlighter.highlight(&doc);

        // Should have ranges for heading, paragraph, and code block
        assert!(ranges.len() >= 3);
    }
}
