use std::cell::RefCell;
use std::sync::OnceLock;

use super::lexer::Token;
use crate::nodes::node::{ExpressionTree, Node};
use crate::utils::errors::{Result, ScriptingError};

pub struct Parser {
    tokens: RefCell<Vec<Token>>,
    position: RefCell<usize>,
    line: RefCell<usize>,
    column: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: RefCell::new(tokens),
            position: RefCell::new(0),
            line: RefCell::new(1),
            column: RefCell::new(1),
        }
    }

    pub fn current_token(&self) -> Token {
        self.tokens
            .borrow()
            .get(*self.position.borrow())
            .cloned()
            .unwrap_or(Token::EOF)
    }

    pub fn prev_token(&self) -> Token {
        self.tokens
            .borrow()
            .get(*self.position.borrow() - 1)
            .cloned()
            .unwrap_or(Token::EOF)
    }

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

    // Generate an error message containing the current line and column
    pub fn error_message(&self, msg: &str) -> ScriptingError {
        let line = *self.line.borrow();
        let column = *self.column.borrow();
        ScriptingError::InvalidSyntax(format!(
            "Error at line {}, column {}: {}",
            line, column, msg
        ))
    }

    /// Verifies that the current token matches the expected token and advances the parser.
    /// Returns an error if the token does not match.
    pub fn expect_token(&self, expected: Token) -> Result<()> {
        if self.current_token() == expected {
            Ok(())
        } else {
            Err(self.error_message(&format!(
                "Expected token {:?}, found {:?}",
                expected,
                self.current_token()
            )))
        }
    }
}

impl Parser {
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

    pub fn parse_expression(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::If => self.parse_if(),
            Token::EOF => Err(self.error_message("Unexpected end of expression")),
            _ => {
                //let lhs = self.parse_variable()?;
                let lhs = self.parse_variable()?;
                match self.current_token() {
                    Token::Assign => self.parse_assign(lhs),
                    Token::EOF => Err(self.error_message("Unexpected end of expression")),
                    Token::Newline => Err(self.error_message("Unexpected newline")),
                    _ => Err(ScriptingError::UnexpectedToken(format!(
                        "{:?}",
                        self.current_token()
                    ))),
                }
            }
        }
    }

    pub fn parse_if(&self) -> Result<ExpressionTree> {
        self.expect_token(Token::If)?;
        self.advance();
        let mut conditions = self.parse_conditions()?;

        self.expect_token(Token::Then)?;
        self.advance();

        let mut expressions = Vec::new();
        while self.current_token() != Token::EOF
            && self.current_token() != Token::Else
            && self.current_token() != Token::End
        {
            let expr = self.parse_expression()?;
            expressions.push(expr);
        }

        let mut else_index = None;
        if self.current_token() == Token::Else {
            self.advance();
            else_index = Some(expressions.len());

            let mut else_statements = Vec::new();
            while self.current_token() != Token::EOF && self.current_token() != Token::End {
                // Parse either a regular expression or another nested if
                let statement = match self.current_token() {
                    Token::If => self.parse_if()?, // Handle nested `if` here
                    _ => self.parse_expression()?,
                };
                else_statements.push(statement);
            }

            if self.current_token() == Token::End {
                self.advance();
                conditions.extend(expressions);
                conditions.extend(else_statements);
                return Ok(Box::new(Node::If(conditions, else_index)));
            } else {
                return Err(self.error_message("Expected `end` after `else` block"));
            }
        } else if self.current_token() == Token::End {
            self.advance();
            conditions.extend(expressions);
            return Ok(Box::new(Node::If(conditions, else_index)));
        } else {
            return Err(self.error_message("Expected `else` or `end` after `then` block"));
        }
    }

    pub fn parse_variable(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::Identifier(name) => {
                self.advance();
                Ok(Box::new(Node::Variable(Vec::new(), name, OnceLock::new())))
            }
            _ => Err(ScriptingError::UnexpectedToken(format!(
                "{:?}",
                self.current_token()
            ))),
        }
    }

    pub fn parse_assign(&self, lhs: ExpressionTree) -> Result<ExpressionTree> {
        self.expect_token(Token::Assign)?;
        self.advance(); // Advance past the '=' token

        let rhs = self.parse_expr()?; // Parse the right-hand side of the assignment

        // Check for semicolon after the assignment expression
        self.expect_token(Token::Semicolon)?;
        self.advance(); // Advance past the ';' token

        // Create and return the assignment node
        Ok(Box::new(Node::Assign(vec![lhs, rhs])))
    }

    pub fn parse_constant(&self) -> Result<ExpressionTree> {
        if let Token::Value(value, boolean) = self.current_token() {
            self.advance(); // Advance immediately after checking the token
            match boolean {
                Some(true) => Ok(Box::new(Node::True)),
                Some(false) => Ok(Box::new(Node::False)),
                None => match value {
                    Some(v) => Ok(Box::new(Node::Constant(v))),
                    None => Err(ScriptingError::UnexpectedToken(format!(
                        "{:?}",
                        self.current_token()
                    ))),
                },
            }
        } else {
            Err(ScriptingError::UnexpectedToken(format!(
                "{:?}",
                self.current_token()
            )))
        }
    }

    pub fn parse_for(&self) -> Result<ExpressionTree> {
        todo!("Implement for loop parsing")
    }

    pub fn parse_conditions(&self) -> Result<Vec<ExpressionTree>> {
        let mut conditions = Vec::new();
        let mut condition = self.parse_condition_element()?;

        // Loop to handle all logical operators within a condition
        while matches!(self.current_token(), Token::And | Token::Or) {
            let operator = self.current_token();
            self.advance(); // Move past the logical operator

            let rhs = self.parse_condition_element()?; // Parse the right-hand side condition
            condition = match operator {
                Token::And => Box::new(Node::And(vec![condition, rhs])),
                Token::Or => Box::new(Node::Or(vec![condition, rhs])),
                _ => return Err(ScriptingError::UnexpectedToken(format!("{:?}", operator))),
            };
        }

        conditions.push(condition);
        Ok(conditions)
    }

    pub fn parse_condition_element(&self) -> Result<ExpressionTree> {
        // Parse the left-hand side expression
        let lhs = self.parse_expr_l2()?; // Or another appropriate parsing level

        // Match a comparison operator and advance to the next token
        let comparator = self.current_token();
        match comparator {
            Token::Equal
            | Token::NotEqual
            | Token::Superior
            | Token::Inferior
            | Token::SuperiorOrEqual
            | Token::InferiorOrEqual => {
                self.advance(); // Move to the right-hand side expression
            }
            _ => {
                return Err(ScriptingError::UnexpectedToken(format!("{:?}", comparator)));
            }
        }

        // Parse the right-hand side expression
        let rhs = self.parse_expr_l2()?; // Or another appropriate parsing level

        // Create the appropriate comparison node
        let comparison_node = match comparator {
            Token::Equal => Box::new(Node::Equal(vec![lhs, rhs])),
            Token::NotEqual => Box::new(Node::NotEqual(vec![lhs, rhs])),
            Token::Superior => Box::new(Node::Superior(vec![lhs, rhs])),
            Token::Inferior => Box::new(Node::Inferior(vec![lhs, rhs])),
            Token::SuperiorOrEqual => Box::new(Node::SuperiorOrEqual(vec![lhs, rhs])),
            Token::InferiorOrEqual => Box::new(Node::InferiorOrEqual(vec![lhs, rhs])),
            _ => return Err(ScriptingError::UnexpectedToken(format!("{:?}", comparator))),
        };

        Ok(comparison_node)
    }

    pub fn find_matching_parentheses(&self) -> Result<usize> {
        let mut open_parens = 1;
        let mut index = *self.position.borrow() + 1;
        while open_parens > 0 {
            match self.tokens.borrow().get(index) {
                Some(Token::OpenParen) => open_parens += 1,
                Some(Token::CloseParen) => open_parens -= 1,
                Some(Token::EOF) => return Err(self.error_message("Expected closing parenthesis")),
                _ => (),
            }
            index += 1;
        }
        Ok(index)
    }

    pub fn parse_condition_parentheses(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::OpenParen => {
                self.advance();
                let expr = self.parse_condition_element()?;
                match self.current_token() {
                    Token::CloseParen => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err(self.error_message("Expected closing parenthesis")),
                }
            }
            _ => Err(ScriptingError::UnexpectedToken(format!(
                "{:?}",
                self.current_token()
            ))),
        }
    }

    pub fn parse_function_args(&self) -> Result<Vec<ExpressionTree>> {
        self.expect_token(Token::OpenParen)?;
        self.advance();
        let mut args = Vec::new();
        while self.current_token() != Token::CloseParen {
            let arg = self.parse_expr()?;
            args.push(arg);
            match self.current_token() {
                Token::Comma => self.advance(),
                Token::CloseParen => (),
                _ => return Err(self.error_message("Expected comma or closing parenthesis")),
            };
        }
        Ok(args)
    }

    pub fn parse_var_const_func(&self) -> Result<ExpressionTree> {
        let try_const = self.parse_constant();
        if try_const.is_ok() {
            return try_const;
        }

        let mut min_args = 0;
        let mut max_args = 0;
        let mut expr = None;
        match self.current_token() {
            Token::Identifier(name) => match name.as_str() {
                "ln" => {
                    min_args = 1;
                    max_args = 1;
                    expr = Some(Node::Ln(Vec::new()));
                }
                "exp" => {
                    min_args = 1;
                    max_args = 1;
                    expr = Some(Node::Exp(Vec::new()));
                }
                "pow" => {
                    min_args = 2;
                    max_args = 2;
                    expr = Some(Node::Pow(Vec::new()));
                }
                "min" => {
                    min_args = 2;
                    max_args = 100;
                    expr = Some(Node::Min(Vec::new()));
                }
                "max" => {
                    min_args = 2;
                    max_args = 100;
                    expr = Some(Node::Max(Vec::new()));
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
        self.advance();
        if expr.is_some() {
            let args = self.parse_function_args()?;
            self.expect_token(Token::CloseParen)?;
            self.advance();
            if args.len() < min_args || args.len() > max_args {
                return Err(self.error_message("Invalid number of arguments"));
            }
            args.iter()
                .for_each(|arg| expr.as_mut().unwrap().add_child(arg.clone()));
            return Ok(Box::new(expr.unwrap()));
        }

        self.parse_variable()
    }

    pub fn parse_parentheses<T, U>(
        &self,
        fun_on_match: T,
        fun_on_no_match: U,
    ) -> Result<ExpressionTree>
    where
        T: Fn(&Parser) -> Result<ExpressionTree>,
        U: Fn(&Parser) -> Result<ExpressionTree>,
    {
        match self.current_token() {
            Token::OpenParen => {
                self.advance();
                let expr = fun_on_match(self)?;
                match self.current_token() {
                    Token::CloseParen => {
                        self.advance();
                        Ok(expr)
                    }
                    _ => Err(self.error_message("Expected closing parenthesis")),
                }
            }
            _ => fun_on_no_match(self),
        }
    }

    pub fn parse_expr(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_expr_l2()?;

        while self.current_token() == Token::Plus
            || self.current_token() == Token::Minus
            || self.current_token() == Token::And
            || self.current_token() == Token::Or && self.current_token() != Token::EOF
        {
            let token = self.current_token();
            self.advance();
            match self.current_token() {
                Token::EOF => return Err(self.error_message("Unexpected end of expression")),
                _ => {
                    let rhs = self.parse_expr_l2()?;
                    lhs = match token {
                        Token::Plus => Box::new(Node::Add(vec![lhs, rhs])),
                        Token::Minus => Box::new(Node::Subtract(vec![lhs, rhs])),
                        Token::And => Box::new(Node::And(vec![lhs, rhs])),
                        Token::Or => Box::new(Node::Or(vec![lhs, rhs])),
                        _ => {
                            return Err(ScriptingError::UnexpectedToken(format!(
                                "{:?}",
                                self.current_token()
                            )))
                        }
                    };
                }
            }
        }
        Ok(lhs)
    }

    pub fn parse_expr_l2(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_expr_l3()?;

        while self.current_token() == Token::Multiply
            || self.current_token() == Token::Divide && self.current_token() != Token::EOF
        {
            let token = self.current_token();
            self.advance();
            match self.current_token() {
                Token::EOF => return Err(self.error_message("Unexpected end of expression")),
                _ => {
                    let rhs = self.parse_expr_l3()?;
                    lhs = match token {
                        Token::Multiply => Box::new(Node::Multiply(vec![lhs, rhs])),
                        Token::Divide => Box::new(Node::Divide(vec![lhs, rhs])),
                        _ => {
                            return Err(ScriptingError::UnexpectedToken(format!(
                                "{:?}",
                                self.current_token()
                            )))
                        }
                    };
                }
            }
        }
        Ok(lhs)
    }

    pub fn parse_expr_l3(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_var_const_func()?;

        while self.current_token() == Token::Power && self.current_token() != Token::EOF {
            self.advance();
            match self.current_token() {
                Token::EOF => return Err(self.error_message("Unexpected end of expression")),
                _ => {
                    let rhs = self.parse_var_const_func()?;
                    lhs = Box::new(Node::Pow(vec![lhs, rhs]));
                }
            }
        }
        Ok(lhs)
    }

    // unary plus and minus
    pub fn parse_expr_l4(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::Plus => {
                self.advance();
                self.parse_expr_l4()
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_expr_l4()?;
                Ok(Box::new(Node::UnaryMinus(vec![expr])))
            }
            _ => self.parse_parentheses(Parser::parse_expr, Parser::parse_var_const_func),
        }
    }
}

#[cfg(test)]
mod tests_advance {
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

#[cfg(test)]
mod tests_expect_token {
    use std::sync::OnceLock;

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
        let tokens = Lexer::new(
            "a = 1;
        
        "
            .to_string(),
        )
        .tokenize()
        .unwrap();
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
        
        if a == 1 then 
            b = 2; 
        end
        
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
            ],
            None,
        ))]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_else_statement() {
        let tokens = Lexer::new(
            "
        if a == 1 then 
        b = 2; 
        else 
        b = 3; 
        end
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
            if a == 1 then 
                if b == 2 then 
                    c = 3;
                else 
                    c = 4;
                end 
            else 
                c = 5;
            end"
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
            if a == 1 then 
                if b == 2 then 
                    c = 3; 
                    d = 4;
                else 
                    c = 5;
                    d = 6;
                end 
            else 
                c = 7;
                d = 8;
            end"
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
            if a == 1 and b == 2 then 
                c = 3;
            end"
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
            if a == 1 or b == 2 then 
                c = 3;
            end"
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
                if x == 1 then
                    z = 3;
                    w = 4;
                end
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
                if x == true then
                    z = 3;
                end
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
}
