//! Parser - Converts tokens to AST
//!
//! This module implements a recursive descent parser that builds an AST
//! from a token stream.

use crate::tokenizer::{Token, TokenType, Tokenizer};
use cybermd_ast::{ASTNode, Position};

/// Markdown parser
pub struct MarkdownParser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<String>,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            pos: 0,
            errors: Vec::new(),
        }
    }

    /// Parse markdown text into AST
    pub fn parse(&mut self, text: &str) -> ASTNode {
        // Stage 1: Tokenization
        let mut tokenizer = Tokenizer::new(text.to_string());
        self.tokens = tokenizer.tokenize();
        self.pos = 0;
        self.errors.clear();

        // Stage 2: Build AST
        let mut document = ASTNode::new_document();

        while self.peek().is_some() && self.peek_type() != TokenType::Eof {
            // Skip empty lines
            if self.peek_type() == TokenType::Newline {
                self.advance();
                continue;
            }

            // Parse block-level element
            if let Some(node) = self.parse_block() {
                let _ = document.add_child(node);
            }
        }

        document
    }

    /// Get parse errors
    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    // Token manipulation

    fn peek(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    fn peek_type(&self) -> TokenType {
        self.peek()
            .map(|t| t.token_type)
            .unwrap_or(TokenType::Eof)
    }

    fn peek_offset(&self, offset: usize) -> Option<&Token> {
        let pos = self.pos + offset;
        if pos < self.tokens.len() {
            Some(&self.tokens[pos])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while self.peek_type() == TokenType::Space {
            self.advance();
        }
    }

    fn skip_to_newline(&mut self) {
        while self.peek_type() != TokenType::Newline && self.peek_type() != TokenType::Eof {
            self.advance();
        }
        if self.peek_type() == TokenType::Newline {
            self.advance();
        }
    }

    // Block-level parsing

    fn parse_block(&mut self) -> Option<ASTNode> {
        let token_type = self.peek_type();

        match token_type {
            TokenType::Hash => self.parse_heading(),
            TokenType::TripleBacktick => self.parse_code_block(),
            TokenType::Dash | TokenType::Asterisk | TokenType::Plus => {
                self.parse_list(false)
            }
            TokenType::Number => {
                // Check for ordered list (1. Item)
                if let Some(next) = self.peek_offset(1) {
                    if next.token_type == TokenType::Dot {
                        return self.parse_list(true);
                    }
                }
                self.parse_paragraph()
            }
            TokenType::GT => self.parse_blockquote(),
            TokenType::Eof => None,
            _ => self.parse_paragraph(),
        }
    }

    fn parse_heading(&mut self) -> Option<ASTNode> {
        let start_pos = self.peek().map(|t| t.position);

        // Count hashes
        let mut level = 0;
        while self.peek_type() == TokenType::Hash {
            level += 1;
            self.advance();
        }

        // Limit to 1-6
        level = level.min(6);

        // Skip spaces
        self.skip_whitespace();

        // Collect heading text
        let mut text_parts = Vec::new();
        while self.peek_type() != TokenType::Newline && self.peek_type() != TokenType::Eof {
            if let Some(token) = self.peek() {
                text_parts.push(token.value.clone());
            }
            self.advance();
        }

        let text = text_parts.join("");

        // Skip newline
        if self.peek_type() == TokenType::Newline {
            self.advance();
        }

        let end_pos = self.peek().map(|t| t.position);

        Some(match level {
            1..=6 => {
                let mut node = ASTNode::new_heading(level as u8, text);
                // Set positions via pattern matching
                if let ASTNode::Heading {
                    start_pos: ref mut _start,
                    end_pos: ref mut _end,
                    ..
                } = node
                {
                    *_start = start_pos;
                    *_end = end_pos;
                }
                node
            }
            _ => ASTNode::new_paragraph(text),
        })
    }

    fn parse_code_block(&mut self) -> Option<ASTNode> {
        let start_pos = self.peek().map(|t| t.position);

        // Consume opening ```
        self.advance();

        // Get language (rest of first line)
        let mut language_parts = Vec::new();
        while self.peek_type() != TokenType::Newline && self.peek_type() != TokenType::Eof {
            if let Some(token) = self.peek() {
                language_parts.push(token.value.clone());
            }
            self.advance();
        }

        let language = language_parts.join("").trim().to_string();

        // Skip newline after language
        if self.peek_type() == TokenType::Newline {
            self.advance();
        }

        // Collect code until closing ```
        let mut code_parts = Vec::new();
        while self.peek_type() != TokenType::TripleBacktick && self.peek_type() != TokenType::Eof {
            if let Some(token) = self.peek() {
                code_parts.push(token.value.clone());
            }
            self.advance();
        }

        let code = code_parts.join("");

        // Consume closing ```
        if self.peek_type() == TokenType::TripleBacktick {
            self.advance();
        }

        // Skip newline after closing ```
        if self.peek_type() == TokenType::Newline {
            self.advance();
        }

        let end_pos = self.peek().map(|t| t.position);

        let mut node = ASTNode::new_code_block(
            if language.is_empty() {
                "text".to_string()
            } else {
                language
            },
            code,
        );

        if let ASTNode::CodeBlock {
            start_pos: ref mut _start,
            end_pos: ref mut _end,
            ..
        } = node
        {
            *_start = start_pos;
            *_end = end_pos;
        }

        Some(node)
    }

    fn parse_list(&mut self, ordered: bool) -> Option<ASTNode> {
        let start_pos = self.peek().map(|t| t.position);

        let mut list = ASTNode::new_list(ordered);

        // Parse list items
        while self.peek_type() != TokenType::Eof {
            // Check if this is a list item
            let is_item = if ordered {
                // Ordered: number followed by dot
                self.peek_type() == TokenType::Number
                    && self
                        .peek_offset(1)
                        .map(|t| t.token_type == TokenType::Dot)
                        .unwrap_or(false)
            } else {
                // Unordered: -, *, or +
                matches!(
                    self.peek_type(),
                    TokenType::Dash | TokenType::Asterisk | TokenType::Plus
                )
            };

            if !is_item {
                break;
            }

            // Consume list marker
            if ordered {
                self.advance(); // number
                self.advance(); // dot
            } else {
                self.advance(); // -, *, or +
            }

            // Skip spaces
            self.skip_whitespace();

            // Collect item text
            let mut text_parts = Vec::new();
            while self.peek_type() != TokenType::Newline && self.peek_type() != TokenType::Eof {
                if let Some(token) = self.peek() {
                    text_parts.push(token.value.clone());
                }
                self.advance();
            }

            let text = text_parts.join("");
            let item = ASTNode::new_list_item(text);

            let _ = list.add_child(item);

            // Skip newline
            if self.peek_type() == TokenType::Newline {
                self.advance();
            }
        }

        let end_pos = self.peek().map(|t| t.position);

        if let ASTNode::List {
            start_pos: ref mut _start,
            end_pos: ref mut _end,
            ..
        } = list
        {
            *_start = start_pos;
            *_end = end_pos;
        }

        Some(list)
    }

    fn parse_blockquote(&mut self) -> Option<ASTNode> {
        let start_pos = self.peek().map(|t| t.position);

        // Consume >
        self.advance();
        self.skip_whitespace();

        // Collect quoted text
        let mut text_parts = Vec::new();
        while self.peek_type() != TokenType::Newline && self.peek_type() != TokenType::Eof {
            if let Some(token) = self.peek() {
                text_parts.push(token.value.clone());
            }
            self.advance();
        }

        let text = text_parts.join("");

        // Skip newline
        if self.peek_type() == TokenType::Newline {
            self.advance();
        }

        let end_pos = self.peek().map(|t| t.position);

        // For now, store as paragraph child in blockquote
        let mut blockquote = ASTNode::Blockquote {
            children: vec![ASTNode::new_paragraph(text)],
            start_pos,
            end_pos,
        };

        Some(blockquote)
    }

    fn parse_paragraph(&mut self) -> Option<ASTNode> {
        let start_pos = self.peek().map(|t| t.position);

        let mut text_parts = Vec::new();

        // Collect text until blank line or special block start
        while self.peek_type() != TokenType::Eof {
            // Check for block-level markers
            if matches!(
                self.peek_type(),
                TokenType::Hash | TokenType::TripleBacktick | TokenType::Newline
            ) {
                break;
            }

            // Check for list start
            if matches!(
                self.peek_type(),
                TokenType::Dash | TokenType::Asterisk | TokenType::Plus
            ) {
                // Only break if at start of line (TODO: track column)
                break;
            }

            if let Some(token) = self.peek() {
                text_parts.push(token.value.clone());
            }
            self.advance();
        }

        let text = text_parts.join("");

        // Skip trailing newline
        if self.peek_type() == TokenType::Newline {
            self.advance();
        }

        if text.trim().is_empty() {
            return None;
        }

        let end_pos = self.peek().map(|t| t.position);

        let mut node = ASTNode::new_paragraph(text);

        if let ASTNode::Paragraph {
            start_pos: ref mut _start,
            end_pos: ref mut _end,
            ..
        } = node
        {
            *_start = start_pos;
            *_end = end_pos;
        }

        Some(node)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let mut parser = MarkdownParser::new();
        let ast = parser.parse("# Hello World");

        assert_eq!(ast.children().len(), 1);
        match &ast.children()[0] {
            ASTNode::Heading { level, text, .. } => {
                assert_eq!(*level, 1);
                assert_eq!(text, "Hello World");
            }
            _ => panic!("Expected heading node"),
        }
    }

    #[test]
    fn test_parse_code_block() {
        let mut parser = MarkdownParser::new();
        let ast = parser.parse("```rust\nfn main() {}\n```");

        assert_eq!(ast.children().len(), 1);
        match &ast.children()[0] {
            ASTNode::CodeBlock { language, code, .. } => {
                assert_eq!(language, "rust");
                assert!(code.contains("fn main"));
            }
            _ => panic!("Expected code block node"),
        }
    }

    #[test]
    fn test_parse_list() {
        let mut parser = MarkdownParser::new();
        let ast = parser.parse("- Item 1\n- Item 2");

        assert_eq!(ast.children().len(), 1);
        match &ast.children()[0] {
            ASTNode::List { ordered, items, .. } => {
                assert!(!ordered);
                assert_eq!(items.len(), 2);
            }
            _ => panic!("Expected list node"),
        }
    }

    #[test]
    fn test_parse_paragraph() {
        let mut parser = MarkdownParser::new();
        let ast = parser.parse("This is a paragraph.");

        assert_eq!(ast.children().len(), 1);
        match &ast.children()[0] {
            ASTNode::Paragraph { text, .. } => {
                assert!(text.contains("paragraph"));
            }
            _ => panic!("Expected paragraph node"),
        }
    }
}
