use crate::term::*;
use std::{fmt, iter::Peekable, str::Chars};

// Source: https://github.com/notJoon/lambda
// Author: Lee ByeongJun

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedCharacter(char),
    UnmatchedParenthesis,
    InvalidLambda,
    InvalidApplication,
    InvalidVariable,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidLambda => write!(f, "Invalid lambda expression"),
            ParseError::InvalidApplication => write!(f, "Invalid application expression"),
            ParseError::InvalidVariable => write!(f, "Invalid variable expression"),
            ParseError::UnexpectedCharacter(c) => write!(f, "Unexpected character: {}", c),
            ParseError::UnmatchedParenthesis => write!(f, "Unmatched parenthesis"),
        }
    }
}

type TermResult = Result<Term, ParseError>;

struct Parser<'a> {
    chars: Peekable<Chars<'a>>,
}

/// A parser for lambda calculus expressions.
impl<'a> Parser<'a> {
    /// Create a new parser for the given input.
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    /// Parse a non-application term
    fn parse_lambda(&mut self) -> TermResult {
        if self.chars.next() == Some('λ') {
            let bind = self.parse_var().map_err(|_| ParseError::InvalidLambda)?;

            self.skip_whitespace();

            if self.chars.next() == Some('.') {
                let body = self.parse_term()?;
                Ok(Term::Abs(
                    bind,
                    Box::new(body),
                ))
            } else {
                Err(ParseError::InvalidLambda)
            }
        } else {
            Err(ParseError::UnexpectedCharacter('λ'))
        }
    }

    /// Parse an application
    fn parse_application(&mut self) -> TermResult {
        let mut terms: Vec<Term> = vec![self.parse_term()?];

        while let Ok(term) = self.parse_term() {
            terms.push(term);
        }

        if terms.is_empty() {
            Err(ParseError::InvalidApplication)
        } else if terms.len() == 1 {
            Ok(terms.pop().unwrap())
        } else {
            let mut iter = terms.into_iter();
            let mut app = iter.next().unwrap();

            for term in iter {
                app = Term::App(
                    Box::new(app),
                    Box::new(term),
                );
            }

            Ok(app)
        }
    }

    /// Parse a variable
    fn parse_var(&mut self) -> Result<String, ParseError> {
        let mut name = String::new();

        while let Some(c) = self.chars.peek() {
            if c.is_alphanumeric() || *c == '_' {
                name.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        if name.is_empty() {
            Err(ParseError::InvalidVariable)
        } else {
            Ok(name)
        }
    }

    /// Parse a non-application term (i.e., a lambda abstraction or a variable) from the input.
    fn parse_term(&mut self) -> TermResult {
        self.skip_whitespace();

        if self.chars.peek() == Some(&'(') {
            // consume the '('
            self.chars.next();

            let term = match self.chars.peek() {
                Some('λ') => self.parse_lambda()?,
                Some(_) => self.parse_application()?,
                None => return Err(ParseError::UnmatchedParenthesis),
            };
            
            self.chars
                .next()
                .and_then(|c| if c == ')' { Some(term) } else { None })
                .ok_or(ParseError::UnmatchedParenthesis)
        } else {
            self.parse_non_application_term()
        }
    }

    /// Parse a non-application term (i.e., a lambda abstraction or a variable) from the input.
    ///
    /// This function is used to parse the sub-expressions of an application. Since an application
    /// consists of a sequence of non-application terms, this function ensures that only lambda
    /// abstractions or variables are parsed within an application.
    ///
    /// # Returns
    ///
    /// * `Ok(JsonTerm)` - A successfully parsed non-application term (lambda abstraction or variable).
    /// * `Err(ParseError::InvalidApplication)` - If the input doesn't match a valid non-application term.
    fn parse_non_application_term(&mut self) -> TermResult {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&'λ') => self.parse_lambda(),
            Some(c) if c.is_alphanumeric() || *c == '_' => Ok(Term::Var(self.parse_var()?)),
            _ => Err(ParseError::InvalidApplication),
        }
    }
}

pub fn parse(input: &str) -> TermResult {
    let mut parser = Parser::new(input);
    let term = parser.parse_term()?;
    if parser.chars.peek().is_some() {
        Err(ParseError::InvalidApplication)
    } else {
        Ok(term)
    }
}
