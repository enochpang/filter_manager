// The lexer will always return a token with calls to next_token.
// Unknown characters return a error token instead of throwing an error.
// Validation is deferred to the Astparser.

/// Represents the type of lex item.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Error,
    Text,
    Eol,
    End,
}

/// Represents a lex item.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

/// Holds the state of the lexer
pub struct Lexer {
    input: Vec<u8>,
    offset: usize,
    rdoffset: usize,
}

impl Lexer {
    pub fn new(items: Vec<u8>) -> Lexer {
        Lexer {
            input: items,
            offset: 0,
            rdoffset: 0,
        }
    }

    /// Returns the next token in the input
    pub fn next_token(&mut self) -> Token {
        let mut ch = self.advance();

        // Ignore whitespace
        while ch == ' ' {
            self.offset = self.rdoffset;
            ch = self.advance();
        }

        let token = match ch {
            '\r' => self.emit_newline(),
            '\n' => self.emit_newline(),
            '\0' => self.emit(TokenKind::End),
            _ => self.emit_text(),
        };

        self.offset = self.rdoffset;
        token
    }

    /// Returns a text token
    fn emit_text(&mut self) -> Token {
        loop {
            match self.peek() {
                ' ' | '\r' | '\n' | '\0' => break,
                _ => _ = self.advance(),
            }
        }

        self.emit(TokenKind::Text)
    }

    /// Returns a newline token
    fn emit_newline(&mut self) -> Token {
        while self.peek() == '\r' || self.peek() == '\n' {
            self.advance();
        }

        self.emit(TokenKind::Eol)
    }

    /// Consumes the next character in the input
    fn advance(&mut self) -> char {
        match self.rdoffset >= self.input.len() {
            true => '\0',
            false => {
                let ch = self.input[self.rdoffset] as char;
                self.rdoffset += 1;
                ch
            }
        }
    }

    /// Look ahead one character
    fn peek(&self) -> char {
        match self.rdoffset >= self.input.len() {
            true => '\0',
            false => self.input[self.rdoffset] as char,
        }
    }

    /// Returns a token with the lexer's state
    fn emit(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: self.current_lexeme(),
        }
    }

    /// Returns the current substring under observation
    fn current_lexeme(&self) -> String {
        String::from_utf8_lossy(&self.input[self.offset..self.rdoffset]).to_string()
    }
}
