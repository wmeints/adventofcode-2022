use std::fmt::Display;
use crate::scanner::{get_tokens, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Syntax {
    ChangeDirectory { target: String },
    ListContents,
    Directory { name: String },
    File { name: String, size: i32 },
}

impl Display for Syntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Syntax::ChangeDirectory { target } => {
                write!(f, "cd {}", target)
            }
            Syntax::ListContents => {
                write!(f, "ls")
            }
            Syntax::Directory { name } => {
                write!(f, "dir {}", name)
            }
            Syntax::File { name, size } => {
                write!(f, "file {} {}", name, size)
            }
        }
    }
}

fn parse_cd(it: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Syntax {
    let target_token = it.next().expect("Expected another token. Got end of stream.");

    let target = match target_token {
        Token::String { value } => value.clone(),
        _ => panic!("Invalid token {}", target_token),
    };

    let postfix_token = it.next().expect("Expected another token. Got end of stream.");

    match postfix_token {
        Token::NewLine | Token::EndOfStream => {},
        _ => panic!("Invalid token {}", postfix_token),
    };

    Syntax::ChangeDirectory { target }
}

fn parse_ls(it: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Syntax {
    let postfix_token = it.next().expect("Expected another token. Got end of stream.");

    match postfix_token {
        Token::NewLine | Token::EndOfStream => {},
        _ => panic!("Invalid token {}", postfix_token),
    };

    Syntax::ListContents
}

fn parse_command(it: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Syntax {
    let _prefix = it.next().expect("Expected another token. Got end of stream.");

    let name_token = it.next().expect("Expected another token. Got end of stream.");

    let name = match name_token {
        Token::String { value } => value.clone(),
        _ => unreachable!(),
    };

    match name.as_str() {
        "cd" => parse_cd(it),
        "ls" => parse_ls(it),
        _ => panic!("Invalid token {}", name_token),
    }
}

fn parse_directory(it: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Syntax {
    let _prefix = it.next();

    let name = match it.next() {
        Some(Token::String { value }) => value.clone(),
        _ => panic!("Invalid token"),
    };

    let postfix = it.next();

    match postfix {
        Some(Token::NewLine) | Some(Token::EndOfStream) => {},
        _ => panic!("Invalid token {}", postfix.unwrap()),
    };

    Syntax::Directory { name }
}

fn parse_file(it: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> Syntax {
    let size_token =  it.next().expect("Expected another token. Got end of stream.");

    let size = if let Token::Number { value } = size_token {
        value.clone()
    } else {
        panic!("Invalid token {}", size_token);
    };
    
    let name_token = it.next().expect("Expected another token. Got end of stream.");

    let name = if let Token::String { value } = name_token {
        value.clone()
    } else {
        panic!("Invalid token {}", name_token);
    };

    let postfix_token = it.next().expect("Expected another token. Got end of stream.");

    match postfix_token {
        Token::NewLine | Token::EndOfStream => {},
        _ => panic!("Invalid token {}", postfix_token),
    };

    Syntax::File {
        name,
        size,
    }
}

pub fn parse_text(input: &str) -> Vec<Syntax> {
    let tokens = get_tokens(input);
    let mut it = tokens.iter().peekable();
    let mut syntax: Vec<Syntax> = Vec::new();

    while let Some(token) = it.peek() {
        match token {
            Token::Prefix => {
                let command = parse_command(&mut it);
                syntax.push(command);
            }
            Token::String { value: _ } => {
                let directory = parse_directory(&mut it);
                syntax.push(directory);
            }
            Token::Number { value: _ } => {
                let file = parse_file(&mut it);
                syntax.push(file);
            }
            _ => {
                it.next();
            }
        }
    }

    syntax
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cd_command() {
        let input = "$ cd /";
        let syntax = parse_text(input);

        let expected_syntax = vec![super::Syntax::ChangeDirectory {
            target: "/".to_string(),
        }];

        assert_eq!(syntax, expected_syntax);
    }

    #[test]
    fn test_parse_cd_command_parent() {
        let input = "$ cd ..";
        let syntax = parse_text(input);

        let expected_syntax = vec![super::Syntax::ChangeDirectory {
            target: "..".to_string(),
        }];

        assert_eq!(syntax, expected_syntax);
    }

    #[test]
    fn test_parse_ls_command() {
        let input = "$ ls";
        let syntax = parse_text(input);

        let expected_syntax = vec![Syntax::ListContents];

        assert_eq!(syntax, expected_syntax);
    }

    #[test]
    fn test_parse_directory() {
        let input = "dir test";
        let syntax = parse_text(input);

        let expected_syntax = vec![Syntax::Directory {
            name: "test".to_string(),
        }];

        assert_eq!(syntax, expected_syntax);
    }

    #[test]
    fn test_parse_file() {
        let input = "1024 test.txt";
        let syntax = parse_text(input);

        let expected_syntax = vec![Syntax::File {
            name: "test.txt".to_string(),
            size: 1024,
        }];

        assert_eq!(syntax, expected_syntax);
    }
}
