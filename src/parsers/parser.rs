use std::cell::RefCell;

use super::lexer::Token;
use crate::nodes::node::{ExpressionTree, Node};
use crate::utils::errors::{Result, ScriptingError};

pub struct Parser {
    tokens: RefCell<Vec<Token>>,
    position: RefCell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: RefCell::new(tokens),
            position: RefCell::new(0),
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

    pub fn advance(&self) {
        *self.position.borrow_mut() += 1;
    }
}

impl Parser {
    pub fn parse(&self) -> Result<ExpressionTree> {
        let mut expressions = Vec::new();
        while self.current_token() != Token::EOF {
            let expr = self.parse_expression()?;
            expressions.push(expr);
        }
        Ok(Box::new(Node::Base(expressions)))
    }

    pub fn parse_expression(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::If => return self.parse_if(),
            Token::EOF => Err(ScriptingError::InvalidSyntax(
                "Unexpected end of expression".to_string(),
            )),
            _ => {
                let lhs = self.parse_variable()?;
                match self.current_token() {
                    Token::Assign => self.parse_assign(lhs),
                    Token::EOF => Err(ScriptingError::InvalidSyntax(
                        "Unexpected end of expression".to_string(),
                    )),
                    _ => Err(ScriptingError::UnexpectedToken),
                }
            }
        }
    }

    pub fn parse_if(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::If => {
                self.advance();

                match self.current_token() {
                    Token::EOF => Err(ScriptingError::InvalidSyntax(
                        "Unexpected end of if statement".to_string(),
                    )),
                    _ => {
                        let mut conditions = self.parse_conditions()?;
                        match self.current_token() {
                            Token::Then => {
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
                                match self.current_token() {
                                    Token::Else => {
                                        self.advance();
                                        else_index = Some(expressions.len());
                                        let mut else_statements = Vec::new();
                                        while self.current_token() != Token::EOF
                                            && self.current_token() != Token::End
                                        {
                                            let statement = self.parse_expression()?;
                                            else_statements.push(statement);
                                        }

                                        match self.current_token() {
                                            Token::End => {
                                                self.advance();
                                                conditions.extend(expressions);
                                                conditions.extend(else_statements);
                                                Ok(Box::new(Node::If(conditions, else_index)))
                                            }
                                            _ => Err(ScriptingError::InvalidSyntax(
                                                "Expected end of if statement".to_string(),
                                            )),
                                        }
                                    }
                                    Token::End => {
                                        self.advance();
                                        conditions.extend(expressions);
                                        Ok(Box::new(Node::If(conditions, else_index)))
                                    }
                                    _ => Err(ScriptingError::InvalidSyntax(
                                        "Expected else or end of if statement".to_string(),
                                    )),
                                }
                            }
                            _ => Err(ScriptingError::UnexpectedToken),
                        }
                    }
                }
            }
            _ => Err(ScriptingError::UnexpectedToken),
        }
    }

    pub fn parse_variable(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::Identifier(name) => {
                self.advance();
                Ok(Box::new(Node::Variable(Vec::new(), name, None)))
            }
            _ => Err(ScriptingError::UnexpectedToken),
        }
    }

    pub fn parse_assign(&self, lhs: ExpressionTree) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::Assign => {
                self.advance();
                match self.current_token() {
                    Token::EOF => Err(ScriptingError::InvalidSyntax(
                        "Unexpected end of assignment".to_string(),
                    )),
                    _ => {
                        let rhs = self.parse_expr()?;
                        Ok(Box::new(Node::Assign(vec![lhs, rhs])))
                    }
                }
            }
            _ => Err(ScriptingError::UnexpectedToken),
        }
    }

    pub fn parse_constant(&self) -> Result<ExpressionTree> {
        match self.current_token() {
            Token::Value(value, boolean) => match boolean {
                Some(true) => {
                    self.advance();
                    Ok(Box::new(Node::True))
                }
                Some(false) => {
                    self.advance();
                    Ok(Box::new(Node::False))
                }
                None => match value {
                    Some(v) => {
                        self.advance();
                        Ok(Box::new(Node::Constant(v)))
                    }
                    None => Err(ScriptingError::UnexpectedToken),
                },
            },
            _ => Err(ScriptingError::UnexpectedToken),
        }
    }

    pub fn parse_for(&self) -> Result<ExpressionTree> {
        todo!("Implement for loop parsing")
    }

    pub fn parse_conditions(&self) -> Result<Vec<ExpressionTree>> {
        let mut conditions = Vec::new();
        while self.current_token() != Token::EOF && self.current_token() != Token::Then {
            let lhs = self.parse_condition_element()?;
            match self.current_token() {
                Token::And => {
                    self.advance();
                    let rhs = self.parse_condition_element()?;
                    conditions.push(Box::new(Node::And(vec![lhs, rhs])));
                }
                Token::Or => {
                    self.advance();
                    let rhs = self.parse_condition_element()?;
                    conditions.push(Box::new(Node::Or(vec![lhs, rhs])));
                }
                Token::Then => conditions.push(lhs),
                _ => return Err(ScriptingError::UnexpectedToken),
            }
        }
        Ok(conditions)
    }

    pub fn parse_condition_element(&self) -> Result<ExpressionTree> {
        let lhs = self.parse_expr()?;
        if self.current_token() == Token::EOF {
            return Err(ScriptingError::InvalidSyntax(
                "Unexpected end of condition".to_string(),
            ));
        }

        let comparator = self.current_token();
        self.advance();
        if self.current_token() == Token::EOF {
            return Err(ScriptingError::InvalidSyntax(
                "Unexpected end of condition".to_string(),
            ));
        }
        let rhs = self.parse_expr()?;

        match comparator {
            Token::Equal => Ok(Box::new(Node::Equal(vec![lhs, rhs]))),
            Token::NotEqual => Ok(Box::new(Node::NotEqual(vec![lhs, rhs]))),
            Token::Superior => Ok(Box::new(Node::Superior(vec![lhs, rhs]))),
            Token::Inferior => Ok(Box::new(Node::Inferior(vec![lhs, rhs]))),
            Token::SuperiorOrEqual => Ok(Box::new(Node::SuperiorOrEqual(vec![lhs, rhs]))),
            Token::InferiorOrEqual => Ok(Box::new(Node::InferiorOrEqual(vec![lhs, rhs]))),
            Token::EOF => Err(ScriptingError::InvalidSyntax(
                "Unexpected end of condition".to_string(),
            )),
            _ => Err(ScriptingError::UnexpectedToken),
        }
    }

    pub fn find_matching_parentheses(&self) -> Result<usize> {
        let mut open_parens = 1;
        let mut index = *self.position.borrow() + 1;
        while open_parens > 0 {
            match self.tokens.borrow().get(index) {
                Some(Token::OpenParen) => open_parens += 1,
                Some(Token::CloseParen) => open_parens -= 1,
                Some(Token::EOF) => {
                    return Err(ScriptingError::InvalidSyntax(
                        "Expected closing parenthesis".to_string(),
                    ))
                }
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
                    _ => Err(ScriptingError::InvalidSyntax(
                        "Expected closing parenthesis".to_string(),
                    )),
                }
            }
            _ => Err(ScriptingError::UnexpectedToken),
        }
    }

    pub fn parse_function_args(&self) -> Result<Vec<ExpressionTree>> {
        let mut args = Vec::new();
        while self.current_token() != Token::CloseParen {
            let arg = self.parse_expression()?;
            args.push(arg);
            match self.current_token() {
                Token::Comma => self.advance(),
                Token::CloseParen => (),
                _ => return Err(ScriptingError::InvalidSyntax("Expected comma".to_string())),
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
                "LN" => {
                    min_args = 1;
                    max_args = 1;
                    expr = Some(Node::Ln(Vec::new()));
                }
                "EXP" => {
                    min_args = 1;
                    max_args = 1;
                    expr = Some(Node::Exp(Vec::new()));
                }
                "POW" => {
                    min_args = 2;
                    max_args = 2;
                    expr = Some(Node::Pow(Vec::new()));
                }
                "MIN" => {
                    min_args = 2;
                    max_args = 100;
                    expr = Some(Node::Min(Vec::new()));
                }
                "MAX" => {
                    min_args = 2;
                    max_args = 100;
                    expr = Some(Node::Max(Vec::new()));
                }
                _ => (),
            },
            _ => (),
        }

        if expr.is_some() {
            let args = self.parse_function_args()?;
            if args.len() < min_args || args.len() > max_args {
                return Err(ScriptingError::InvalidSyntax(
                    "Invalid number of arguments".to_string(),
                ));
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
                    _ => Err(ScriptingError::InvalidSyntax(
                        "Expected closing parenthesis".to_string(),
                    )),
                }
            }
            _ => fun_on_no_match(self),
        }
    }

    pub fn parse_expr(&self) -> Result<ExpressionTree> {
        let mut lhs = self.parse_expr_l2()?;

        while self.current_token() == Token::Plus
            || self.current_token() == Token::Minus && self.current_token() != Token::EOF
        {
            let token = self.current_token();
            self.advance();
            match self.current_token() {
                Token::EOF => {
                    return Err(ScriptingError::InvalidSyntax(
                        "Unexpected end of expression".to_string(),
                    ))
                }
                _ => {
                    let rhs = self.parse_expr_l2()?;
                    lhs = match token {
                        Token::Plus => Box::new(Node::Add(vec![lhs, rhs])),
                        Token::Minus => Box::new(Node::Subtract(vec![lhs, rhs])),
                        _ => return Err(ScriptingError::UnexpectedToken),
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
                Token::EOF => {
                    return Err(ScriptingError::InvalidSyntax(
                        "Unexpected end of expression".to_string(),
                    ))
                }
                _ => {
                    let rhs = self.parse_expr_l3()?;
                    lhs = match token {
                        Token::Multiply => Box::new(Node::Multiply(vec![lhs, rhs])),
                        Token::Divide => Box::new(Node::Divide(vec![lhs, rhs])),
                        _ => return Err(ScriptingError::UnexpectedToken),
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
                Token::EOF => {
                    return Err(ScriptingError::InvalidSyntax(
                        "Unexpected end of expression".to_string(),
                    ))
                }
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
mod tests {
    use super::*;
    use crate::parsers::lexer::Lexer;

    #[test]
    fn test_variable_assignment() {
        let tokens = Lexer::new("a = 1".to_string()).tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        println!("{:?}", result);

        let expected = Box::new(Node::Base(vec![Box::new(Node::Assign(vec![
            Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
            Box::new(Node::Constant(1.0)),
        ]))]));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_statement() {
        let tokens = Lexer::new("if a == 1 then b = 2 end".to_string())
            .tokenize()
            .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();
        println!("{:?}", result);

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Equal(vec![
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
                    Box::new(Node::Constant(2.0)),
                ])),
            ],
            None,
        ))]));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_if_else_statement() {
        let tokens = Lexer::new("if a == 1 then b = 2 else b = 3 end".to_string())
            .tokenize()
            .unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        let expected = Box::new(Node::Base(vec![Box::new(Node::If(
            vec![
                Box::new(Node::Equal(vec![
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
                    Box::new(Node::Constant(2.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
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
                    c = 3 
                else 
                    c = 4 
                end 
            else 
                c = 5 
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
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::If(
                    vec![
                        Box::new(Node::Equal(vec![
                            Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
                            Box::new(Node::Constant(2.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                            Box::new(Node::Constant(3.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                            Box::new(Node::Constant(4.0)),
                        ])),
                    ],
                    Some(1),
                )),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
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
                    c = 3 
                    d = 4
                else 
                    c = 5 
                    d = 6
                end 
            else 
                c = 7 
                d = 8
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
                    Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
                    Box::new(Node::Constant(1.0)),
                ])),
                Box::new(Node::If(
                    vec![
                        Box::new(Node::Equal(vec![
                            Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
                            Box::new(Node::Constant(2.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                            Box::new(Node::Constant(3.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "d".to_string(), None)),
                            Box::new(Node::Constant(4.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                            Box::new(Node::Constant(5.0)),
                        ])),
                        Box::new(Node::Assign(vec![
                            Box::new(Node::Variable(Vec::new(), "d".to_string(), None)),
                            Box::new(Node::Constant(6.0)),
                        ])),
                    ],
                    Some(2),
                )),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                    Box::new(Node::Constant(7.0)),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "d".to_string(), None)),
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
                c = 3 
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
                        Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
                        Box::new(Node::Constant(1.0)),
                    ])),
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
                        Box::new(Node::Constant(2.0)),
                    ])),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                    Box::new(Node::Constant(3.0)),
                ])),
            ],
            None,
        ))]));

        assert_eq!(result, expected);

        let tokens = Lexer::new(
            "
            if a == 1 or b == 2 then 
                c = 3 
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
                        Box::new(Node::Variable(Vec::new(), "a".to_string(), None)),
                        Box::new(Node::Constant(1.0)),
                    ])),
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "b".to_string(), None)),
                        Box::new(Node::Constant(2.0)),
                    ])),
                ])),
                Box::new(Node::Assign(vec![
                    Box::new(Node::Variable(Vec::new(), "c".to_string(), None)),
                    Box::new(Node::Constant(3.0)),
                ])),
            ],
            None,
        ))]));

        assert_eq!(result, expected);
    }
}
