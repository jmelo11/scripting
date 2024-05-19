use std::cell::RefCell;

use crate::utils::errors::{Result, ScriptingError};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Value(Option<f64>, Option<bool>),
    Identifier(String),
    String(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Assign,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    Pays,
    Superior,
    Inferior,
    SuperiorOrEqual,
    InferiorOrEqual,
    OpenParen,
    CloseParen,
    OpenCurlyParen,
    CloseCurlyParen,
    If,
    Then,
    Else,
    End,
    Comma,
    Power,
    For,
    Semicolon, // for end of an expression or statement
    Newline,   // for end of a line
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: RefCell<usize>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: RefCell::new(0),
        }
    }

    fn next_char(&self) -> char {
        if *self.position.borrow() >= self.input.len() {
            '\0'
        } else {
            let ch = self.input[*self.position.borrow()];
            *self.position.borrow_mut() += 1;
            ch
        }
    }

    fn peek_char(&self) -> char {
        if *self.position.borrow() >= self.input.len() {
            '\0' // Using null character to denote end of input
        } else {
            self.input[*self.position.borrow()]
        }
    }

    pub fn next_token(&self) -> Result<Token> {
        self.skip_whitespace();
        let ch = self.next_char();
        match ch {
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => {
                if self.peek_char() == '*' {
                    self.next_char();
                    Ok(Token::Power)
                } else {
                    Ok(Token::Multiply)
                }
            }
            '#' => {
                while self.peek_char() != '\n' && self.peek_char() != '\0' {
                    self.next_char();
                }
                self.next_token()
            }
            '/' => Ok(Token::Divide),
            '=' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Ok(Token::Equal)
                } else {
                    Ok(Token::Assign)
                }
            }
            '\n' => Ok(Token::Newline),
            ',' => Ok(Token::Comma),
            '!' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Ok(Token::NotEqual)
                } else {
                    Err(ScriptingError::InvalidSyntax(
                        "Invalid character: !".to_string(),
                    ))
                }
            }
            '(' => Ok(Token::OpenParen),
            ')' => Ok(Token::CloseParen),
            '{' => Ok(Token::OpenCurlyParen),
            '}' => Ok(Token::CloseCurlyParen),
            ';' => Ok(Token::Semicolon),
            '\0' => Ok(Token::EOF),
            '>' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Ok(Token::SuperiorOrEqual)
                } else {
                    Ok(Token::Superior)
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.next_char();
                    Ok(Token::InferiorOrEqual)
                } else {
                    Ok(Token::Inferior)
                }
            }
            '\"' => self.read_string(),
            _ if ch.is_digit(10) => self.read_number(ch),
            _ if ch.is_alphabetic() => self.read_identifier(ch),
            _ => Err(ScriptingError::InvalidSyntax(format!(
                "Invalid character: {}",
                ch
            ))),
        }
    }

    fn read_string(&self) -> Result<Token> {
        let mut string = "".to_string();
        while self.peek_char() != '\"' {
            string.push(self.next_char());
        }
        self.next_char(); // consume the closing quote
        Ok(Token::String(string))
    }

    // This function is used to read numerical literals, including floating point numbers.
    // Should fail if the number is not valid or if it is not a number.
    fn read_number(&self, first_char: char) -> Result<Token> {
        let mut number = first_char.to_string();
        while self.peek_char().is_digit(10) || self.peek_char() == '.' {
            number.push(self.next_char());
        }

        Ok(Token::Value(Some(number.parse::<f64>()?), None))
    }

    // This function is used to read identifiers and special keywords
    fn read_identifier(&self, first_char: char) -> Result<Token> {
        let mut identifier = first_char.to_string();
        while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
            identifier.push(self.next_char());
        }
        match identifier.as_str() {
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "end" => Ok(Token::End),
            "and" => Ok(Token::And),
            "or" => Ok(Token::Or),
            "not" => Ok(Token::Not),
            "for" => Ok(Token::For),
            "true" => Ok(Token::Value(None, Some(true))),
            "false" => Ok(Token::Value(None, Some(false))),
            "pays" => Ok(Token::Pays),
            _ => Ok(Token::Identifier(identifier)),
        }
    }

    fn skip_whitespace(&self) {
        while self.peek_char().is_whitespace() && self.peek_char() != '\n' {
            self.next_char();
        }
    }

    pub fn tokenize(&self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_operators() {
        let input = "+ - * / = == ;";
        let expected_tokens = vec![
            Token::Plus,
            Token::Minus,
            Token::Multiply,
            Token::Divide,
            Token::Assign,
            Token::Equal,
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_numerical_literals() {
        let input = "123 4.56";
        let expected_tokens = vec![
            Token::Value(Some(123.0), None),
            Token::Value(Some(4.56), None),
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let input = "x if else and or not true false";
        let expected_tokens = vec![
            Token::Identifier("x".to_string()),
            Token::If,
            Token::Else,
            Token::And,
            Token::Or,
            Token::Not,
            Token::Value(None, Some(true)), // assuming true is represented as 1.0
            Token::Value(None, Some(false)), // assuming false is represented as 0.0
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_mixed_input() {
        let input = "if (x > 100) { x = 100; } else { x = 0; }";
        let expected_tokens = vec![
            Token::If,
            Token::OpenParen,
            Token::Identifier("x".to_string()),
            Token::Superior,
            Token::Value(Some(100.0), None),
            Token::CloseParen,
            Token::OpenCurlyParen,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Value(Some(100.0), None),
            Token::Semicolon,
            Token::CloseCurlyParen,
            Token::Else,
            Token::OpenCurlyParen,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Value(Some(0.0), None),
            Token::Semicolon,
            Token::CloseCurlyParen,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_invalid_input() {
        let input = "@";

        let lexer = Lexer::new(input.to_string());
        let err = lexer.tokenize();
        assert!(err.is_err());
    }

    #[test]
    fn test_whitespace() {
        let input = "  1  + 2  ";
        let expected_tokens = vec![
            Token::Value(Some(1.0), None),
            Token::Plus,
            Token::Value(Some(2.0), None),
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_comparison_operators() {
        let input = "< > <= >=";
        let expected_tokens = vec![
            Token::Inferior,
            Token::Superior,
            Token::InferiorOrEqual,
            Token::SuperiorOrEqual,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_logical_operators() {
        let input = "and or not";
        let expected_tokens = vec![Token::And, Token::Or, Token::Not];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_parentheses() {
        let input = "( ) { }";
        let expected_tokens = vec![
            Token::OpenParen,
            Token::CloseParen,
            Token::OpenCurlyParen,
            Token::CloseCurlyParen,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_assignment() {
        let input = "x = 10;";
        let expected_tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Value(Some(10.0), None),
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_long_var_names() {
        let input = "long_variable_name = 10;";
        let expected_tokens = vec![
            Token::Identifier("long_variable_name".to_string()),
            Token::Assign,
            Token::Value(Some(10.0), None),
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_var_names_spaces() {
        let input = "var_1 var_2";
        let expected_tokens = vec![
            Token::Identifier("var_1".to_string()),
            Token::Identifier("var_2".to_string()),
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_var_names_front_invalid_char() {
        let input = "1var_1 2var_2";

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            Token::Value(Some(1.0), None),
            Token::Identifier("var_1".to_string()),
            Token::Value(Some(2.0), None),
            Token::Identifier("var_2".to_string()),
        ];

        assert_eq!(tokens.unwrap(), expected_tokens);
    }

    #[test]
    fn test_var_names_back_invalid_char() {
        let input = "var_1_ var_2_";

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize();
        let expected_tokens = vec![
            Token::Identifier("var_1_".to_string()),
            Token::Identifier("var_2_".to_string()),
        ];

        assert_eq!(tokens.unwrap(), expected_tokens);
    }

    #[test]
    fn test_boolean_literals() {
        let input = "true false";
        let expected_tokens = vec![
            Token::Value(None, Some(true)),
            Token::Value(None, Some(false)),
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_and_not() {
        let input = "and not";
        let expected_tokens = vec![Token::And, Token::Not];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_and_not_with_vars() {
        let input = "x = y and z";

        let expected_tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("y".to_string()),
            Token::And,
            Token::Identifier("z".to_string()),
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_new_lines() {
        let input = "x = 10;\n y = 20;";
        let expected_tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Value(Some(10.0), None),
            Token::Semicolon,
            Token::Newline,
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Value(Some(20.0), None),
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_and_not_with_vars_2() {
        let input = "x = true;
            y = false;
            z = x and y;
            w = x or y;";

        let expected_tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Value(None, Some(true)),
            Token::Semicolon,
            Token::Newline,
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Value(None, Some(false)),
            Token::Semicolon,
            Token::Newline,
            Token::Identifier("z".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::And,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::Newline,
            Token::Identifier("w".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Or,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
        ];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_skipt_coments() {
        let input = " 1 #+ 2 ";
        let expected_tokens = vec![Token::Value(Some(1.0), None)];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);

        let input = " 1 ###+ 2## ";
        let expected_tokens = vec![Token::Value(Some(1.0), None)];

        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_power_operator() {
        let input = "2 ** 3";
        let expected_tokens = vec![
            Token::Value(Some(2.0), None),
            Token::Power,
            Token::Value(Some(3.0), None),
        ];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_power_operator_with_parentheses() {
        let input = "(2 + 3) ** 2";
        let expected_tokens = vec![
            Token::OpenParen,
            Token::Value(Some(2.0), None),
            Token::Plus,
            Token::Value(Some(3.0), None),
            Token::CloseParen,
            Token::Power,
            Token::Value(Some(2.0), None),
        ];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_max_function() {
        let input = "max(2, 3)";
        let expected_tokens = vec![
            Token::Identifier("max".to_string()),
            Token::OpenParen,
            Token::Value(Some(2.0), None),
            Token::Comma,
            Token::Value(Some(3.0), None),
            Token::CloseParen,
        ];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_pays() {
        let input = "pays";
        let expected_tokens = vec![Token::Pays];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_string_literals() {
        let input = "\"hello\"";
        let expected_tokens = vec![Token::String("hello".to_string())];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_string_literals_with_spaces() {
        let input = "\"hello world\"";
        let expected_tokens = vec![Token::String("hello world".to_string())];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_string_literals_with_special_chars() {
        let input = "\"hello world!\"";
        let expected_tokens = vec![Token::String("hello world!".to_string())];
        let lexer = Lexer::new(input.to_string());
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens, expected_tokens);
    }
}
