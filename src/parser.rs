use std::fmt;

use crate::token::{TokenValue, Tokenizer};

pub struct Parser {
    pub input: String,
    tokenizer: Tokenizer,
    lookahead: Option<TokenValue>,
}

enum LiteralValue {
    NumberLiteral(i32),
    StringLiteral(String),
}

struct Literal {
    value: LiteralValue,
}

pub struct Program {
    body: Literal,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::NumberLiteral(value) => write!(f, "NumberLiteral({})", value),
            LiteralValue::StringLiteral(value) => write!(f, "StringLiteral({})", value),
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Program {{\n    body: {:?}\n}}", self.body)
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Literal {{\n        value: {}\n    }}", self.value)
    }
}

impl Parser {
    /*
     * Initializes the parser
     */
    pub fn new() -> Parser {
        Parser {
            input: "".to_string(),
            tokenizer: Tokenizer::new(),
            lookahead: None,
        }
    }

    /*
     * Parses a string into an AST
     */
    pub fn parse(&mut self, input: &str) -> Program {
        self.input = input.to_string();
        self.tokenizer.init(self.input.as_str());

        self.lookahead = match self.tokenizer.get_next() {
            Some(token) => Some(token.value),
            None => None,
        };

        return self.program();
    }

    /*
     * Main entry point
     * Program
     *  : Literal
     *  ;
     *
     */
    fn program(&mut self) -> Program {
        Program {
            body: self.literal(),
        }
    }

    /*
     * Literal
     *  : [0-9]+
     *  ;
     */
    fn literal(&mut self) -> Literal {
        let token = self.lookahead.clone().unwrap();
        match token {
            TokenValue::Int(value) => {
                self.eat("NUMBER");
                Literal {
                    value: LiteralValue::NumberLiteral(value),
                }
            }
            TokenValue::Str(value) => {
                self.eat("IDENTIFIER");
                Literal {
                    value: LiteralValue::StringLiteral(value),
                }
            }
        }
    }

    fn eat(&mut self, token_type: &str) -> TokenValue {
        if self.lookahead.is_none() {
            panic!(
                "Unexpected end of input: expected token type '{}'",
                token_type
            );
        }

        let token = self.lookahead.clone().unwrap();

        if let Some(next_token) = self.tokenizer.get_next() {
            self.lookahead = Some(next_token.value);
        } else {
            self.lookahead = None;
        }

        match (&token, token_type) {
            (TokenValue::Int(_), "NUMBER") => token,
            (TokenValue::Str(_), "IDENTIFIER") => token,
            _ => panic!(
                "Unexpected token type: expected '{}', got '{:?}'",
                token_type, token
            ),
        }
    }
}
