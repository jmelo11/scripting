use std::cell::RefCell;
use std::sync::OnceLock;

use rustatlas::currencies::enums::Currency;

use super::lexer::Token;
use crate::nodes::node::{ExpressionTree, Node};
use crate::utils::errors::{Result, ScriptingError};

pub struct Parser {
    tokens: RefCell<Vec<Token>>,
    position: RefCell<usize>,
    line: RefCell<usize>,
    column: RefCell<usize>,
    reserved_keywords: Vec<String>,
}

/// public methods
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: RefCell::new(tokens),
            position: RefCell::new(0),
            line: RefCell::new(1),
            column: RefCell::new(1),
            reserved_keywords: vec![
                "if".to_string(),
                "else".to_string(),
                "and".to_string(),
                "or".to_string(),
                "true".to_string(),
                "false".to_string(),
                "spot".to_string(),
                "pays".to_string(),
                "exp".to_string(),
                "ln".to_string(),
                "pow".to_string(),
                "min".to_string(),
                "max".to_string(),
            ],
        }
    }

    pub fn parse(&self) -> Result<ExpressionTree> {
        let mut expressions = Vec::new();
        while self.current_token() != Token::EOF {
            if self.current_token() == Token::Newline {
                self.advance();
                continue;
            }
            let expr = self.parse_expression()?;
            expressions.push(expr);
        }
        Ok(Box::new(Node::Base(expressions)))
    }
}

/// private methods
impl Parser {
    /// Check if the word is a reserved keyword
    fn expect_not_reserved(&self, word: &str) -> Result<()> {
        if self.reserved_keywords.contains(&word.to_string()) {
            Err(self.invalid_syntax_err("Reserved keyword"))
        } else {
            Ok(())
        }
    }

    /// Get the current token
    fn current_token(&self) -> Token {
        self.tokens
            .borrow()
            .get(*self.position.borrow())
            .cloned()
            .unwrap_or(Token::EOF)
    }

    /// Advance the position in the tokens
    fn advance(&self) {
        let mut pos = self.position.borrow_mut();
        let mut line = self.line.borrow_mut();
        let mut column = self.column.borrow_mut();
        let tokens = self.tokens.borrow();

        loop {
            let current_token = tokens.get(*pos + 1).cloned().unwrap_or(Token::EOF);
            match current_token {
                Token::Newline => {
                    *line += 1;
                    *column = 1;
                    *pos += 1;
                }
                _ => {
                    *column += {
                        let token_str = format!("{:?}", current_token);
                        token_str.len()
                    };
                    *pos += 1;
                    break;
                }
            }
        }
    }

    /// Create a new error for invalid syntax
    fn invalid_syntax_err(&self, msg: &str) -> ScriptingError {
        let line = *self.line.borrow();
        let column = *self.column.borrow();
        ScriptingError::InvalidSyntax(format!(
            "Error at line {}, column {}: {}",
            line, column, msg
        ))
    }

    /// Create a new error for unexpected token
    fn unexpected_token_err(&self, expected: Token, received: Token) -> ScriptingError {
        let line = *self.line.borrow();
        let column = *self.column.borrow();
        ScriptingError::UnexpectedToken(format!(
            "Error at line {}, column {}: Expected token {:?}, found {:?}",
            line, column, expected, received
        ))
    }

    /// Expect a token, if it is not the current token, return an error
    fn expect_token(&self, expected: Token) -> Result<()> {
        if self.current_token() == expected {
            Ok(())
        } else {
            Err(self.unexpected_token_err(expected, self.current_token()))
        }
    }

    /// Parse an expression
    fn parse_expression(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::If => self.parse_if(),
            Token::Pays => self.parse_pays(),
            Token::EOF => Err(self.invalid_syntax_err("Unexpected end of expression")),
            _ => {
                let lhs = self.parse_variable()?;
                match self.current_token() {
                    Token::Assign => self.parse_assign(lhs),
                    Token::EOF => Err(self.invalid_syntax_err("Unexpected end of expression")),
                    Token::Newline => Err(self.invalid_syntax_err("Unexpected newline")),
                    _ => Err(self.invalid_syntax_err("Unexpected token")),
                }
            }
        }
    }

    /// Parse a pays expression
    fn parse_pays(&self) -> Result<ExpressionTree> {
        self.expect_token(Token::Pays)?;
        self.advance();
        let mut pays = Vec::new();
        while self.current_token() != Token::EOF {
            let expr = self.parse_expr()?;
            pays.push(expr);
        }
        Ok(Box::new(Node::Pays(pays, OnceLock::new())))
    }

    /// Parse an if expression
    fn parse_if(&self) -> Result<ExpressionTree> {
        self.expect_token(Token::If)?;
        self.advance();
        let condition = self.parse_conditions()?;

        self.expect_token(Token::OpenCurlyParen)?;
        self.advance();

        let mut if_body = Vec::new();
        while self.current_token() != Token::CloseCurlyParen {
            if self.current_token() == Token::EOF {
                return Err(self.invalid_syntax_err("Unexpected end of input in if body"));
            }
            let expr = self.parse_expression()?;
            if_body.push(expr);
        }
        self.advance();
        let mut else_index = None;
        if self.current_token() == Token::Else {
            self.advance();
            self.expect_token(Token::OpenCurlyParen)?;
            self.advance();

            else_index = Some(if_body.len());

            while self.current_token() != Token::CloseCurlyParen {
                if self.current_token() == Token::EOF {
                    return Err(self.invalid_syntax_err("Unexpected end of input in else body"));
                }
                let expr = self.parse_expression()?;
                if_body.push(expr);
            }
            self.advance();
        }

        let mut nodes = condition;
        nodes.append(&mut if_body);

        Ok(Box::new(Node::If(nodes, else_index)))
    }

    /// Parse a variable
    fn parse_variable(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::Identifier(name) => {
                self.expect_not_reserved(&name)?;
                self.advance();
                Ok(Box::new(Node::Variable(Vec::new(), name, OnceLock::new())))
            }
            _ => Err(self
                .unexpected_token_err(Token::Identifier("Any".to_string()), self.current_token())),
        }
    }

    /// Parse a string
    fn parse_string(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::String(string) => {
                self.advance();
                Ok(Box::new(Node::String(string)))
            }
            _ => Err(self.invalid_syntax_err("Invalid string, expected string literal")),
        }
    }

    /// Parse an assign expression
    fn parse_assign(&self, lhs: ExpressionTree) -> Result<ExpressionTree> {
        self.expect_token(Token::Assign)?;
        self.advance();
        let rhs = self.parse_expr()?;
        self.expect_token(Token::Semicolon)?;
        self.advance();
        Ok(Box::new(Node::Assign(vec![lhs, rhs])))
    }

    /// Parse a constant
    fn parse_constant(&self) -> Result<ExpressionTree> {
        if let Token::Value(value, boolean) = self.current_token() {
            self.advance();
            match boolean {
                Some(true) => Ok(Box::new(Node::True)),
                Some(false) => Ok(Box::new(Node::False)),
                None => match value {
                    Some(v) => Ok(Box::new(Node::Constant(v))),
                    None => Err(self.invalid_syntax_err("Invalid constant")),
                },
            }
        } else {
            Err(self.invalid_syntax_err("Invalid constant"))
        }
    }

    /// Parse a condition
    fn parse_conditions(&self) -> Result<Vec<ExpressionTree>> {
        let mut conditions = Vec::new();
        let mut condition = self.parse_condition_element()?;

        while matches!(self.current_token(), Token::And | Token::Or) {
            let operator = self.current_token();
            self.advance();

            let rhs = self.parse_condition_element()?;
            condition = match operator {
                Token::And => Box::new(Node::And(vec![condition, rhs])),
                Token::Or => Box::new(Node::Or(vec![condition, rhs])),
                _ => return Err(self.invalid_syntax_err("Invalid operator")),
            };
        }
        conditions.push(condition);
        Ok(conditions)
    }

    /// Parse a condition element
    fn parse_condition_element(&self) -> Result<ExpressionTree> {
        let lhs = self.parse_expr_l2()?;

        let comparator = self.current_token();
        match comparator {
            Token::Equal
            | Token::NotEqual
            | Token::Superior
            | Token::Inferior
            | Token::SuperiorOrEqual
            | Token::InferiorOrEqual => {
                self.advance();
            }
            _ => {
                return Err(self.invalid_syntax_err("Expected comparison operator"));
            }
        }

        let rhs = self.parse_expr_l2()?;

        let comparison_node = match comparator {
            Token::Equal => Box::new(Node::Equal(vec![lhs, rhs])),
            Token::NotEqual => Box::new(Node::NotEqual(vec![lhs, rhs])),
            Token::Superior => Box::new(Node::Superior(vec![lhs, rhs])),
            Token::Inferior => Box::new(Node::Inferior(vec![lhs, rhs])),
            Token::SuperiorOrEqual => Box::new(Node::SuperiorOrEqual(vec![lhs, rhs])),
            Token::InferiorOrEqual => Box::new(Node::InferiorOrEqual(vec![lhs, rhs])),
            _ => return Err(self.invalid_syntax_err("Invalid comparison operator")),
        };

        Ok(comparison_node)
    }

    /// Parse a function arguments
    fn parse_function_args(&self) -> Result<Vec<ExpressionTree>> {
        self.expect_token(Token::OpenParen)?;
        self.advance();
        let mut args = Vec::new();
        while self.current_token() != Token::CloseParen {
            let arg = self.parse_expr()?;
            args.push(arg);
            match self.current_token() {
                Token::Comma => self.advance(),
                Token::CloseParen => (),
                _ => return Err(self.invalid_syntax_err("Expected comma or closing parenthesis")),
            };
        }
        Ok(args)
    }

    /// Parse a variable, constant or function
    fn parse_var_const_func(&self) -> Result<ExpressionTree> {
        // Check if the current token is a constant
        let try_const = self.parse_constant();
        if try_const.is_ok() {
            return try_const;
        }

        // Check if the current token is a string
        let try_string = self.parse_string();
        if try_string.is_ok() {
            return try_string;
        }

        // Check if the current token is a function
        let mut min_args = 0;
        let mut max_args = 0;
        let mut expr = None;
        match self.current_token() {
            Token::Identifier(name) => match name.as_str() {
                "ln" => {
                    min_args = 1;
                    max_args = 1;
                    expr = Some(Node::new_ln());
                }
                "exp" => {
                    min_args = 1;
                    max_args = 1;
                    expr = Some(Node::new_exp());
                }
                "pow" => {
                    min_args = 2;
                    max_args = 2;
                    expr = Some(Node::new_pow());
                }
                "min" => {
                    min_args = 2;
                    max_args = 100;
                    expr = Some(Node::new_min());
                }
                "max" => {
                    min_args = 2;
                    max_args = 100;
                    expr = Some(Node::new_max());
                }
                "spot" => {
                    return self.parse_spot();
                }
                _ => (),
            },
            _ => {
                return Err(ScriptingError::UnexpectedToken(format!(
                    "{:?}",
                    self.current_token()
                )))
            }
        }
        if expr.is_some() {
            self.advance();
            let args = self.parse_function_args()?;
            self.expect_token(Token::CloseParen)?;
            self.advance();
            if args.len() < min_args || args.len() > max_args {
                return Err(self.invalid_syntax_err("Invalid number of arguments"));
            }
            args.iter()
                .for_each(|arg| expr.as_mut().unwrap().add_child(arg.clone()));
            return Ok(Box::new(expr.unwrap()));
        }

        // Check if the current token is a variable
        self.parse_variable()
    }

    /// Parse a spot expression
    fn parse_spot(&self) -> Result<ExpressionTree> {
        self.expect_token(Token::Identifier("spot".to_string()))?;
        self.advance();
        self.expect_token(Token::OpenParen)?;
        self.advance();
        let currency = match *self.parse_string()? {
            Node::String(s) => {
                Currency::try_from(s).map_err(|_| self.invalid_syntax_err("Invalid currency"))?
            }
            _ => return Err(self.invalid_syntax_err("Invalid argument, expected string")),
        };
        self.expect_token(Token::CloseParen)?;
        self.advance();
        Ok(Box::new(Node::Spot(currency, OnceLock::new())))
    }

    // fn parse_parentheses<T, U>(&self, fun_on_match: T, fun_on_no_match: U) -> Result<ExpressionTree>
    // where
    //     T: Fn(&Parser) -> Result<ExpressionTree>,
    //     U: Fn(&Parser) -> Result<ExpressionTree>,
    // {
    //     match self.current_token() {
    //         Token::OpenParen => {
    //             self.advance();
    //             let expr = fun_on_match(self)?;
    //             match self.current_token() {
    //                 Token::CloseParen => {
    //                     self.advance();
    //                     Ok(expr)
    //                 }
    //                 _ => Err(self.invalid_syntax_err("Expected closing parenthesis")),
    //             }
    //         }
    //         _ => fun_on_no_match(self),
    //     }
    // }

    /// Parse an expression
    fn parse_expr(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_expr_l2()?;

        while self.current_token() == Token::Plus
            || self.current_token() == Token::Minus
            || self.current_token() == Token::And
            || self.current_token() == Token::Or && self.current_token() != Token::EOF
        {
            let token = self.current_token();
            self.advance();
            match self.current_token() {
                Token::EOF => return Err(self.invalid_syntax_err("Unexpected end of expression")),
                _ => {
                    let rhs = self.parse_expr_l2()?;
                    lhs = match token {
                        Token::Plus => Box::new(Node::Add(vec![lhs, rhs])),
                        Token::Minus => Box::new(Node::Subtract(vec![lhs, rhs])),
                        Token::And => Box::new(Node::And(vec![lhs, rhs])),
                        Token::Or => Box::new(Node::Or(vec![lhs, rhs])),
                        _ => {
                            return Err(self.invalid_syntax_err("Invalid operator"));
                        }
                    };
                }
            }
        }
        Ok(lhs)
    }

    /// Parse an expression
    fn parse_expr_l2(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_expr_l3()?;

        while self.current_token() == Token::Multiply
            || self.current_token() == Token::Divide && self.current_token() != Token::EOF
        {
            let token = self.current_token();
            self.advance();
            match self.current_token() {
                Token::EOF => return Err(self.invalid_syntax_err("Unexpected end of expression")),
                _ => {
                    let rhs = self.parse_expr_l3()?;
                    lhs = match token {
                        Token::Multiply => Box::new(Node::Multiply(vec![lhs, rhs])),
                        Token::Divide => Box::new(Node::Divide(vec![lhs, rhs])),
                        _ => {
                            return Err(self.invalid_syntax_err("Invalid operator"));
                        }
                    };
                }
            }
        }
        Ok(lhs)
    }

    /// Parse an expression
    fn parse_expr_l3(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_var_const_func()?;

        while self.current_token() == Token::Power && self.current_token() != Token::EOF {
            self.advance();
            match self.current_token() {
                Token::EOF => return Err(self.invalid_syntax_err("Unexpected end of expression")),
                _ => {
                    let rhs = self.parse_var_const_func()?;
                    lhs = Box::new(Node::Pow(vec![lhs, rhs]));
                }
            }
        }
        Ok(lhs)
    }

    // fn parse_expr_l4(&self) -> Result<ExpressionTree> {
    //     match self.current_token() {
    //         Token::Plus => {
    //             self.advance();
    //             self.parse_expr_l4()
    //         }
    //         Token::Minus => {
    //             self.advance();
    //             let expr = self.parse_expr_l4()?;
    //             Ok(Box::new(Node::UnaryMinus(vec![expr])))
    //         }
    //         _ => self.parse_parentheses(Parser::parse_expr, Parser::parse_var_const_func),
    //     }
    // }
}

/// Tests for the `advance` method
#[cfg(test)]
mod general_tests {
    use super::*;
    use crate::parsers::lexer::Lexer;

    #[test]
    fn test_advance_token() {
        let tokens = Lexer::new("a = 1;".to_string()).tokenize().unwrap();
        let parser = Parser::new(tokens);
        assert_eq!(parser.current_token(), Token::Identifier("a".to_string()));
        parser.advance();
        assert_eq!(parser.current_token(), Token::Assign);
        parser.advance();
        assert_eq!(parser.current_token(), Token::Value(Some(1.0), None));
        parser.advance();
        assert_eq!(parser.current_token(), Token::Semicolon);
        parser.advance();
        assert_eq!(parser.current_token(), Token::EOF);
    }

    #[test]
    fn test_advance_token_with_newlines() {
        let tokens = Lexer::new("a = 1;\n\n".to_string()).tokenize().unwrap();

        let parser = Parser::new(tokens);
        assert_eq!(parser.current_token(), Token::Identifier("a".to_string()));
        parser.advance();
        assert_eq!(parser.current_token(), Token::Assign);
        parser.advance();
        assert_eq!(parser.current_token(), Token::Value(Some(1.0), None));
        parser.advance();
        assert_eq!(parser.current_token(), Token::Semicolon);
        parser.advance();
        assert_eq!(parser.current_token(), Token::EOF);
    }
}

/// Tests for the `parse` method
#[cfg(test)]
mod tests_expect_token {
    use std::sync::OnceLock;

    use rustatlas::currencies::enums::Currency;

    use crate::{
        nodes::node::Node,
        parsers::{lexer::Lexer, parser::Parser},
    };

    #[test]
    fn test_parse_empty() {
        let tokens = Lexer::new("".to_string()).tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(result, Box::new(Node::Base(Vec::new())));
    }

    #[test]
    fn test_handle_newline() {
        let tokens = Lexer::new("\n\n\n".to_string()).tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        assert_eq!(result, Box::new(Node::Base(Vec::new())));
    }

    #[test]
    fn test_variable_assignment() {
        let tokens = Lexer::new("a = 1;".to_string()).tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::Assign(vec![
            Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
            Box::new(Node::Constant(1.0)),
        ]))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_variable_assignment_with_new_lines() {
        let tokens = Lexer::new("a = 1;".to_string()).tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::Assign(vec![
            Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
            Box::new(Node::Constant(1.0)),
        ]))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_statement() {
        let tokens = Lexer::new(
            "
            if a == 1 { 
                b = 2; 
            }"
            .to_string(),
        )
        .tokenize()
        .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Equal(vec![
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(2.0)),
                ])),
            ],
            None,
        ))]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_else_statement() {
        let tokens = Lexer::new(
            "
        if a == 1 { 
            b = 2; 
        } else {
            b = 3; 
        }
        "
            .to_string(),
        )
        .tokenize()
        .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Equal(vec![
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(2.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(3.0)),
                ])),
            ],
            Some(1),
        ))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_if_else_statement() {
        let tokens = Lexer::new(
            "
            if a == 1 { 
                if b == 2 {
                    c = 3;
                }else {
                    c = 4;
                }
            } else {
                c = 5;
            }"
            .to_string(),
        )
        .tokenize()
        .unwrap();

        let result = Parser::new(tokens).parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Equal(vec![
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::If(
                    vec![
                        Box::new(Node::Equal(vec![
                            Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(2.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(3.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(4.0)),
                        ])),
                    ],
                    Some(1),
                )),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(5.0)),
                ])),
            ],
            Some(1),
        ))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_if_else_statement_with_multiple_statements() {
        let tokens = Lexer::new(
            "
            if a == 1 {
                if b == 2 {
                    c = 3; 
                    d = 4;
                } else {
                    c = 5;
                    d = 6;
                }
            } else {
                c = 7;
                d = 8;
            }"
            .to_string(),
        )
        .tokenize()
        .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Equal(vec![
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::If(
                    vec![
                        Box::new(Node::Equal(vec![
                            Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(2.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(3.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "d".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(4.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(5.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "d".to_string(), OnceLock::new())),
                            Box::new(Node::Constant(6.0)),
                        ])),
                    ],
                    Some(2),
                )),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(7.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "d".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(8.0)),
                ])),
            ],
            Some(1),
        ))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_multiple_conditions() {
        let tokens = Lexer::new(
            "
            if a == 1 and b == 2 { 
                c = 3;
            }"
            .to_string(),
        )
        .tokenize()
        .unwrap();

        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::And(vec![
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(1.0)),
                    ])),
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(2.0)),
                    ])),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(3.0)),
                ])),
            ],
            None,
        ))]));

        assert_eq!(result, expected);

        let tokens = Lexer::new(
            "
            if a == 1 or b == 2 {
                c = 3;
            }"
            .to_string(),
        )
        .tokenize();

        let parser = Parser::new(tokens.unwrap());
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Or(vec![
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "a".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(1.0)),
                    ])),
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "b".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(2.0)),
                    ])),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), OnceLock::new())),
                    Box::new(Node::Constant(3.0)),
                ])),
            ],
            None,
        ))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_new_variable() {
        let tokens = Lexer::new(
            "
                x = 2;           
                if x == 1 {
                    z = 3;
                    w = 4;
                }
            "
            .to_string(),
        )
        .tokenize()
        .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                Box::new(Node::Constant(2.0)),
            ])),
            Box::new(Node::If(
                vec![
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(1.0)),
                    ])),
                    Box::new(Node::Assign(vec![
                        Box::new(Node::Variable(Vec::new(), "z".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(3.0)),
                    ])),
                    Box::new(Node::Assign(vec![
                        Box::new(Node::Variable(Vec::new(), "w".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(4.0)),
                    ])),
                ],
                None,
            )),
        ]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_bool_variables_with_if() {
        let tokens = Lexer::new(
            "
                x = true;
                y = false;
                if x == true {
                    z = 3;
                }
            "
            .to_string(),
        )
        .tokenize()
        .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                Box::new(Node::True),
            ])),
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "y".to_string(), OnceLock::new())),
                Box::new(Node::False),
            ])),
            Box::new(Node::If(
                vec![
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                        Box::new(Node::True),
                    ])),
                    Box::new(Node::Assign(vec![
                        Box::new(Node::Variable(Vec::new(), "z".to_string(), OnceLock::new())),
                        Box::new(Node::Constant(3.0)),
                    ])),
                ],
                None,
            )),
        ]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_bool_vars() {
        let script = "
            x = true;
            y = false;
            z = x and y;
            w = x or y;       
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let expected = Box::new(Node::Base(vec![
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                Box::new(Node::True),
            ])),
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "y".to_string(), OnceLock::new())),
                Box::new(Node::False),
            ])),
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "z".to_string(), OnceLock::new())),
                Box::new(Node::And(vec![
                    Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                    Box::new(Node::Variable(Vec::new(), "y".to_string(), OnceLock::new())),
                ])),
            ])),
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "w".to_string(), OnceLock::new())),
                Box::new(Node::Or(vec![
                    Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
                    Box::new(Node::Variable(Vec::new(), "y".to_string(), OnceLock::new())),
                ])),
            ])),
        ]));

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_max_function() {
        let script = "
            z = max(1, 2);
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        tokens.iter().for_each(|t| println!("{:?}", t));

        let nodes = Parser::new(tokens).parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::Assign(vec![
            Box::new(Node::Variable(Vec::new(), "z".to_string(), OnceLock::new())),
            Box::new(Node::Max(vec![
                Box::new(Node::Constant(1.0)),
                Box::new(Node::Constant(2.0)),
            ])),
        ]))]));

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_string_variable() {
        let script = "
            x = \"hello\";
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::Assign(vec![
            Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
            Box::new(Node::String("hello".to_string())),
        ]))]));

        assert_eq!(nodes, expected);
    }

    #[test]
    fn test_spot_function() {
        let script = "
            x = spot(\"USD\");
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::Assign(vec![
            Box::new(Node::Variable(Vec::new(), "x".to_string(), OnceLock::new())),
            Box::new(Node::Spot(Currency::USD, OnceLock::new())),
        ]))]));

        assert_eq!(nodes, expected);
    }
}

/// tests for reserved keywords. These are keywords that are reserved in the scripting language
/// and cannot be used as variable names
#[cfg(test)]
mod test_reserved_keywords {
    #[test]
    fn test_if_reserved() {
        let script = "
            if = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_else_reserved() {
        let script = "
            else = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_and_reserved() {
        let script = "
            and = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_or_reserved() {
        let script = "
            or = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_true_reserved() {
        let script = "
            true = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_false_reserved() {
        let script = "
            false = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_max_reserved() {
        let script = "
            max = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_min_reserved() {
        let script = "
            min = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_pow_reserved() {
        let script = "
            pow = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_ln_reserved() {
        let script = "
            ln = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_exp_reserved() {
        let script = "
            exp = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_spot_reserved() {
        let script = "
            spot = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }

    #[test]
    fn test_pays_reserved() {
        let script = "
            pays = 1;
        "
        .to_string();

        let tokens = crate::parsers::lexer::Lexer::new(script)
            .tokenize()
            .unwrap();
        let nodes = crate::parsers::parser::Parser::new(tokens).parse();
        assert!(nodes.is_err());
    }
}
