//! Tokenizer - Lexical analysis for markdown
//!
//! Converts raw text into a stream of tokens.

use cybermd_ast::Position;

/// Token types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Hash,           // #
    Backtick,       // `
    TripleBacktick, // ```
    Asterisk,       // *
    Underscore,     // _
    Dash,           // -
    Plus,           // +
    LBracket,       // [
    RBracket,       // ]
    LParen,         // (
    RParen,         // )
    Space,          // ' '
    Newline,        // \n
    Text,           // Regular text
    Number,         // Digits
    Dot,            // .
    Colon,          // :
    GT,             // >
    Equals,         // =
    Eof,            // End of file
}

/// A token with its type, value, and position
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: Position,
}

impl Token {
    fn new(token_type: TokenType, value: String, position: Position) -> Self {
        Self {
            token_type,
            value,
            position,
        }
    }
}

/// Tokenizer - converts text to tokens
pub struct Tokenizer {
    text: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            pos: 0,
            line: 0,
            column: 0,
        }
    }

    fn current_position(&self) -> Position {
        Position::new(self.line, self.column, self.pos)
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let pos = self.pos + offset;
        if pos < self.text.len() {
            self.text.chars().nth(pos)
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.pos >= self.text.len() {
            return None;
        }

        let char = self.text.chars().nth(self.pos)?;
        self.pos += 1;

        if char == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        Some(char)
    }

    fn peek_str(&self, len: usize) -> Option<String> {
        if self.pos + len > self.text.len() {
            return None;
        }
        Some(
            self.text
                .chars()
                .skip(self.pos)
                .take(len)
                .collect::<String>(),
        )
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.text.len() {
            let char = match self.peek(0) {
                Some(c) => c,
                None => break,
            };

            // Check for triple backtick first
            if char == '`' {
                if let Some(s) = self.peek_str(3) {
                    if s == "```" {
                        let pos = self.current_position();
                        self.advance();
                        self.advance();
                        self.advance();
                        tokens.push(Token::new(TokenType::TripleBacktick, "```".to_string(), pos));
                        continue;
                    }
                }
            }

            // Single character tokens
            match char {
                '#' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Hash, "#".to_string(), pos));
                }
                '`' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Backtick, "`".to_string(), pos));
                }
                '*' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Asterisk, "*".to_string(), pos));
                }
                '_' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Underscore, "_".to_string(), pos));
                }
                '-' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Dash, "-".to_string(), pos));
                }
                '+' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Plus, "+".to_string(), pos));
                }
                '[' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::LBracket, "[".to_string(), pos));
                }
                ']' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::RBracket, "]".to_string(), pos));
                }
                '(' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::LParen, "(".to_string(), pos));
                }
                ')' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::RParen, ")".to_string(), pos));
                }
                ' ' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Space, " ".to_string(), pos));
                }
                '\n' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Newline, "\n".to_string(), pos));
                }
                '.' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Dot, ".".to_string(), pos));
                }
                ':' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Colon, ":".to_string(), pos));
                }
                '>' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::GT, ">".to_string(), pos));
                }
                '=' => {
                    let pos = self.current_position();
                    self.advance();
                    tokens.push(Token::new(TokenType::Equals, "=".to_string(), pos));
                }
                c if c.is_ascii_digit() => {
                    // Collect number
                    let pos = self.current_position();
                    let mut num = String::new();
                    while let Some(ch) = self.peek(0) {
                        if ch.is_ascii_digit() {
                            num.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::new(TokenType::Number, num, pos));
                }
                _ => {
                    // Regular text - collect until special character
                    let pos = self.current_position();
                    let mut text = String::new();
                    while let Some(ch) = self.peek(0) {
                        if matches!(
                            ch,
                            '#' | '`'
                                | '*'
                                | '_'
                                | '-'
                                | '+'
                                | '['
                                | ']'
                                | '('
                                | ')'
                                | ' '
                                | '\n'
                                | '.'
                                | ':'
                                | '>'
                                | '='
                        ) || ch.is_ascii_digit()
                        {
                            break;
                        }
                        text.push(ch);
                        self.advance();
                    }
                    if !text.is_empty() {
                        tokens.push(Token::new(TokenType::Text, text, pos));
                    }
                }
            }
        }

        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.current_position(),
        ));

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let mut tokenizer = Tokenizer::new("# Hello".to_string());
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 4); // HASH, SPACE, TEXT, EOF
        assert_eq!(tokens[0].token_type, TokenType::Hash);
        assert_eq!(tokens[1].token_type, TokenType::Space);
        assert_eq!(tokens[2].token_type, TokenType::Text);
        assert_eq!(tokens[2].value, "Hello");
    }

    #[test]
    fn test_triple_backtick() {
        let mut tokenizer = Tokenizer::new("```".to_string());
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens[0].token_type, TokenType::TripleBacktick);
        assert_eq!(tokens[0].value, "```");
    }

    #[test]
    fn test_numbers() {
        let mut tokenizer = Tokenizer::new("123".to_string());
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "123");
    }
}
