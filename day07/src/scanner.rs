use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Prefix,
    String { value: String },
    Number { value: i32 },
    NewLine,
    EndOfStream
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Prefix => write!(f, "[prefix]"),
            Token::String { value } => write!(f, "[string({})]", value),
            Token::Number { value } => write!(f, "[number({})]", value),
            Token::NewLine => write!(f, "[newline]"),
            Token::EndOfStream => write!(f, "[end-of-stream]"),
        }
    }
}

fn get_number(it: &mut std::iter::Peekable<std::str::Chars>) -> i32 {
    let mut value = 0;
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                value = value * 10 + (c as i32 - '0' as i32);
                it.next();
            }
            _ => break,
        }
    }
    value
}

fn get_string(it: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut value = String::new();
    while let Some(&c) = it.peek() {
        match c {
            'a'..='z' | '.' | '-' | '_' | '/' => {
                value.push(c);
                it.next();
            }
            _ => break,
        }
    }

    value
}

pub fn get_tokens(input: &str) -> Vec<Token> {
    let mut it = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                let value = get_number(&mut it);
                tokens.push(Token::Number { value });
            }
            '$' => {
                it.next();
                tokens.push(Token::Prefix);
            }
            'a'..='z' |'/' | '.'| '_' | '-' => {
                let value = get_string(&mut it);
                tokens.push(Token::String { value });
            }
            '\n' | '\r' => {
                it.next();

                if let Some(nc) = it.peek() {
                    if *nc == '\n' {
                        it.next();
                    }
                }

                tokens.push(Token::NewLine);
            }
            _ => {
                it.next();
            }
        }
    }

    tokens.push(Token::EndOfStream);

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tokens_single_line() {
        let input = "$ cd /";
        let expected_tokens = vec![
            Token::Prefix,
            Token::String {
                value: "cd".to_string(),
            },
            Token::String {
                value: "/".to_string(),
            },
            Token::EndOfStream
        ];

        let tokens = get_tokens(input);

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_get_tokens_single_line_dots() {
        let input = "$ cd ..";
        let expected_tokens = vec![
            Token::Prefix,
            Token::String {
                value: "cd".to_string(),
            },
            Token::String {
                value: "..".to_string(),
            },
            Token::EndOfStream
        ];

        let tokens = get_tokens(input);

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_get_tokens_multiline() {
        let input = "$ cd /
        dir test
    ";

        let expected_tokens = vec![
            Token::Prefix,
            Token::String {
                value: "cd".to_string(),
            },
            Token::String {
                value: "/".to_string(),
            },
            Token::NewLine,
            Token::String {
                value: "dir".to_string(),
            },
            Token::String {
                value: "test".to_string(),
            },
            Token::NewLine,
            Token::EndOfStream
        ];

        let tokens = get_tokens(input);

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_get_tokens_dots() {
        let input = "test.txt";
        let tokens = get_tokens(input);

        let expected_tokens = vec![
            Token::String {
                value: "test.txt".to_string(),
            },
            Token::EndOfStream
        ];
        
        assert_eq!(tokens, expected_tokens);
    }
}
