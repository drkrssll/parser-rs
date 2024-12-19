use std::fmt::Display;

pub struct Tokenizer {
    input: String,
    cursor: usize,
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    Int(i32),
    Str(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedToken {
    token_type: String,
    pub value: TokenValue,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::Int(value) => write!(f, "{}", value),
            TokenValue::Str(value) => write!(f, "{}", value),
        }
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor: 0,
        }
    }

    pub fn init(&mut self, input: &str) {
        self.input = input.to_string();
    }

    fn has_more(&self) -> bool {
        self.cursor < self.input.len()
    }

    pub fn get_next(&mut self) -> Option<ParsedToken> {
        if !self.has_more() {
            return None;
        }

        let mut string = String::new();
        let first_char = self.input[self.cursor..].chars().next().unwrap();

        let is_number = first_char.is_digit(10);

        while self.has_more() {
            let c = self.input[self.cursor..].chars().next().unwrap();

            if is_number && c.is_digit(10) {
                string.push(c);
                self.cursor += c.len_utf8();
            } else if !is_number
                && (c.is_alphabetic() || (c == '_') || (!string.is_empty() && c.is_digit(10)))
            {
                string.push(c);
                self.cursor += c.len_utf8();
            } else {
                break;
            }
        }

        if !string.is_empty() {
            return Some(ParsedToken {
                token_type: if is_number { "NUMBER" } else { "IDENTIFIER" }.to_string(),
                value: if is_number {
                    TokenValue::Int(string.parse::<i32>().unwrap())
                } else {
                    TokenValue::Str(string)
                },
            });
        }

        None
    }
}
