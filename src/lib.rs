//! A small Arabic-first interpreter. This milestone is experimental, not a stable language ABI.
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use unicode_ident::{is_xid_continue, is_xid_start};

/// Maximum source size in UTF-8 bytes accepted by the interpreter and CLI.
pub const MAX_SOURCE_BYTES: usize = 1_048_576;
/// Maximum size, in UTF-8 bytes, of a single runtime string value.
pub const MAX_STRING_BYTES: usize = MAX_SOURCE_BYTES;
/// Conservative cumulative budget for runtime string clones, concatenations and output.
pub const MAX_RUNTIME_STRING_BYTES: usize = 16 * 1024 * 1024;
/// Maximum number of while-condition evaluations in one program run.
pub const MAX_LOOP_CONDITION_EVALUATIONS: usize = 1_000_000;
/// Maximum recursive function-call depth.
pub const MAX_CALL_DEPTH: usize = 128;
const MAX_FUNCTION_PARAMETERS: usize = 64;
const MAX_TOKENS: usize = 16_384;
// Keep recursion well below typical thread stack limits; this is a safety bound, not a language feature.
const MAX_NESTING: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LangError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl LangError {
    fn at(message: impl Into<String>, token: &Token) -> Self {
        Self {
            message: message.into(),
            line: token.line,
            column: token.column,
        }
    }
}
impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "خطأ في السطر {}، العمود {}: {}",
            self.line, self.column, self.message
        )
    }
}
impl std::error::Error for LangError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Semicolon,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Identifier,
    Number,
    String,
    Let,
    Print,
    Function,
    Return,
    If,
    Else,
    While,
    True,
    False,
    Nil,
    Eof,
}
#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    lexeme: String,
    line: usize,
    column: usize,
}

struct Lexer {
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
    token_line: usize,
    token_column: usize,
    previous_was_carriage_return: bool,
}
impl Lexer {
    fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
            token_line: 1,
            token_column: 1,
            previous_was_carriage_return: false,
        }
    }
    fn scan(mut self) -> Result<Vec<Token>, LangError> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            self.token_line = self.line;
            self.token_column = self.column;
            let ch = self.advance();
            match ch {
                '(' => tokens.push(self.token(TokenKind::LeftParen)),
                ')' => tokens.push(self.token(TokenKind::RightParen)),
                '{' => tokens.push(self.token(TokenKind::LeftBrace)),
                '}' => tokens.push(self.token(TokenKind::RightBrace)),
                ',' => tokens.push(self.token(TokenKind::Comma)),
                '+' => tokens.push(self.token(TokenKind::Plus)),
                '-' => tokens.push(self.token(TokenKind::Minus)),
                '*' => tokens.push(self.token(TokenKind::Star)),
                '/' => tokens.push(self.token(TokenKind::Slash)),
                ';' | '؛' => tokens.push(self.token(TokenKind::Semicolon)),
                '=' => {
                    let k = if self.take('=') {
                        TokenKind::EqualEqual
                    } else {
                        TokenKind::Equal
                    };
                    tokens.push(self.token(k));
                }
                '!' => {
                    let k = if self.take('=') {
                        TokenKind::BangEqual
                    } else {
                        TokenKind::Bang
                    };
                    tokens.push(self.token(k));
                }
                '>' => {
                    let k = if self.take('=') {
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    };
                    tokens.push(self.token(k));
                }
                '<' => {
                    let k = if self.take('=') {
                        TokenKind::LessEqual
                    } else {
                        TokenKind::Less
                    };
                    tokens.push(self.token(k));
                }
                '#' => {
                    while !matches!(self.peek(), '\n' | '\r') && !self.is_at_end() {
                        self.advance();
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {}
                '"' | '«' => tokens.push(self.string(ch)?),
                c if is_digit(c) => tokens.push(self.number()?),
                c if is_identifier_start(c) => tokens.push(self.identifier()),
                _ => {
                    let escaped = ch.escape_default().to_string();
                    return Err(self.error_at_token_start(format!(
                        "محرف Unicode غير مدعوم: U+{:04X} ({escaped})",
                        ch as u32
                    )));
                }
            }
            if tokens.len() > MAX_TOKENS {
                return Err(self.error_at_token_start(format!(
                    "تجاوز البرنامج الحد الأقصى للرموز ({MAX_TOKENS})"
                )));
            }
        }
        tokens.push(Token {
            kind: TokenKind::Eof,
            lexeme: String::new(),
            line: self.line,
            column: self.column,
        });
        Ok(tokens)
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }
    fn advance(&mut self) -> char {
        let ch = self.chars[self.current];
        self.current += 1;
        if ch == '\r' {
            self.line += 1;
            self.column = 1;
            self.previous_was_carriage_return = true;
        } else if ch == '\n' {
            if !self.previous_was_carriage_return {
                self.line += 1;
            }
            self.column = 1;
            self.previous_was_carriage_return = false;
        } else {
            self.column += 1;
            self.previous_was_carriage_return = false;
        }
        ch
    }
    fn peek(&self) -> char {
        self.chars.get(self.current).copied().unwrap_or('\0')
    }
    fn peek_next(&self) -> char {
        self.chars.get(self.current + 1).copied().unwrap_or('\0')
    }
    fn take(&mut self, expected: char) -> bool {
        if self.peek() != expected {
            return false;
        }
        self.advance();
        true
    }
    fn token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: self.chars[self.start..self.current].iter().collect(),
            line: self.token_line,
            column: self.token_column,
        }
    }
    fn string(&mut self, opener: char) -> Result<Token, LangError> {
        let closer = if opener == '«' { '»' } else { '"' };
        while !self.is_at_end() && self.peek() != closer {
            if matches!(self.peek(), '\n' | '\r') {
                return Err(self.error_at_cursor("لا يُسمح بسطر جديد داخل النص"));
            }

            if self.peek() == '\\' {
                self.advance();
                if matches!(self.peek(), '\n' | '\r') {
                    return Err(self.error_at_cursor("لا يُسمح بسطر جديد داخل النص"));
                }
                if !self.is_at_end() {
                    self.advance();
                }
            } else {
                self.advance();
            }
        }
        if self.is_at_end() {
            return Err(self.error_at_cursor("نص غير مغلق"));
        }
        self.advance();
        Ok(self.token(TokenKind::String))
    }
    fn number(&mut self) -> Result<Token, LangError> {
        let family = match digit_family(self.chars[self.start]) {
            Some(family) => family,
            None => {
                return Err(self.error_at_token_start("بداية عدد غير صالحة".into()));
            }
        };

        while let Some(next_family) = digit_family(self.peek()) {
            if next_family != family {
                return Err(self.error_at_cursor("لا يجوز خلط مجموعات الأرقام داخل العدد الواحد"));
            }
            self.advance();
        }

        if matches!(self.peek(), '.' | '٫') && is_digit(self.peek_next()) {
            self.advance();
            while let Some(next_family) = digit_family(self.peek()) {
                if next_family != family {
                    return Err(self.error_at_cursor("لا يجوز خلط مجموعات الأرقام داخل العدد الواحد"));
                }
                self.advance();
            }
        }
        Ok(self.token(TokenKind::Number))
    }
    fn identifier(&mut self) -> Token {
        while is_identifier_continue(self.peek()) {
            self.advance();
        }
        let text: String = self.chars[self.start..self.current].iter().collect();
        let kind = match text.as_str() {
            "دع" => TokenKind::Let,
            "اطبع" => TokenKind::Print,
            "دالة" => TokenKind::Function,
            "أرجع" => TokenKind::Return,
            "إذا" => TokenKind::If,
            "وإلا" => TokenKind::Else,
            "بينما" => TokenKind::While,
            "صحيح" => TokenKind::True,
            "خطأ" => TokenKind::False,
            "لاشيء" => TokenKind::Nil,
            "و" => TokenKind::And,
            "أو" => TokenKind::Or,
            _ => TokenKind::Identifier,
        };
        self.token(kind)
    }
    fn error_at_token_start(&self, message: String) -> LangError {
        LangError {
            message,
            line: self.token_line,
            column: self.token_column,
        }
    }
    fn error_at_cursor(&self, message: impl Into<String>) -> LangError {
        LangError {
            message: message.into(),
            line: self.line,
            column: self.column,
        }
    }
}

/// Decode string escapes after the lexer has verified the matching delimiters.
/// Keep this explicit: silently accepting unknown escapes makes typos invisible.
fn decode_string_literal(token: &Token) -> Result<String, LangError> {
    let opener = token.lexeme.chars().next().unwrap_or('"');
    let closer = if opener == '«' { '»' } else { '"' };
    let body = token
        .lexeme
        .strip_prefix(opener)
        .and_then(|text| text.strip_suffix(closer))
        .ok_or_else(|| LangError::at("صياغة النص غير صالحة", token))?;

    let mut decoded = String::with_capacity(body.len());
    let mut chars = body.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            decoded.push(ch);
            continue;
        }

        let escaped = chars
            .next()
            .ok_or_else(|| LangError::at("شرطة هروب عكسية غير مكتملة في نهاية النص", token))?;
        match escaped {
            'n' => decoded.push('\n'),
            'r' => decoded.push('\r'),
            't' => decoded.push('\t'),
            '\\' => decoded.push('\\'),
            '"' => decoded.push('"'),
            '«' => decoded.push('«'),
            '»' => decoded.push('»'),
            other => {
                return Err(LangError::at(
                    format!(
                        "تسلسل هروب غير مدعوم: \\{} (المدعوم: \\n و\\r و\\t و\\\\ وعلامات الاقتباس)",
                        other.escape_default()
                    ),
                    token,
                ));
            }
        }
    }
    Ok(decoded)
}

fn finite_number(value: f64, operator: &Token) -> Result<Value, LangError> {
    if value.is_finite() {
        Ok(Value::Number(value))
    } else {
        Err(LangError::at(
            "نتج عن العملية عدد غير محدود؛ تجاوزت القيمة نطاق الأعداد المدعوم",
            operator,
        ))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DigitFamily {
    Ascii,
    ArabicIndic,
    ExtendedArabicIndic,
}

fn digit_family(ch: char) -> Option<DigitFamily> {
    match ch {
        '0'..='9' => Some(DigitFamily::Ascii),
        '٠'..='٩' => Some(DigitFamily::ArabicIndic),
        '۰'..='۹' => Some(DigitFamily::ExtendedArabicIndic),
        _ => None,
    }
}

fn is_digit(ch: char) -> bool {
    digit_family(ch).is_some()
}
fn normalize_digits(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '٠'..='٩' => char::from(b'0' + (ch as u32 - '٠' as u32) as u8),
            '۰'..='۹' => char::from(b'0' + (ch as u32 - '۰' as u32) as u8),
            '٫' => '.',
            _ => ch,
        })
        .collect()
}
// UAX #31 identifier properties from the pinned unicode-ident dependency.
// Underscore is a deliberate Rukn-specific addition to both positions.
fn is_identifier_start(ch: char) -> bool {
    ch == '_' || is_xid_start(ch)
}
fn is_identifier_continue(ch: char) -> bool {
    ch == '_' || is_xid_continue(ch)
}

#[derive(Debug, Clone)]
enum Expr {
    Literal {
        value: Value,
        token: Token,
    },
    Variable(Token),
    Call {
        name: Token,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping(Box<Expr>),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
enum Stmt {
    Function {
        name: Token,
        parameters: Vec<Token>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Let {
        name: Token,
        initializer: Expr,
    },
    Assign {
        name: Token,
        value: Expr,
    },
    Print {
        expression: Expr,
        keyword: Token,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        keyword: Token,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        keyword: Token,
        condition: Expr,
        body: Box<Stmt>,
    },
    Expression(Expr),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    nesting: usize,
    block_depth: usize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            nesting: 0,
            block_depth: 0,
        }
    }
    fn enter_nesting(&mut self, token: &Token) -> Result<(), LangError> {
        if self.nesting >= MAX_NESTING {
            return Err(LangError::at(
                format!("تجاوز البرنامج الحد الأقصى للتداخل ({MAX_NESTING})"),
                token,
            ));
        }
        self.nesting += 1;
        Ok(())
    }
    fn nested_expression(&mut self) -> Result<Expr, LangError> {
        let token = self.peek().clone();
        self.enter_nesting(&token)?;
        let result = self.expression();
        self.nesting -= 1;
        result
    }
    fn nested_unary(&mut self) -> Result<Expr, LangError> {
        let token = self.peek().clone();
        self.enter_nesting(&token)?;
        let result = self.unary();
        self.nesting -= 1;
        result
    }
    fn parse(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements = Vec::new();
        while !self.check(TokenKind::Eof) {
            let statement = self.statement()?;
            if let Stmt::Function { name, .. } = &statement
                && statements.iter().any(|previous| {
                    matches!(
                        previous,
                        Stmt::Function {
                            name: previous_name,
                            ..
                        } if previous_name.lexeme == name.lexeme
                    )
                })
            {
                return Err(LangError::at(
                    format!("تعريف دالة مكرر: {}", name.lexeme),
                    name,
                ));
            }
            statements.push(statement);
        }
        Ok(statements)
    }
    fn statement(&mut self) -> Result<Stmt, LangError> {
        if self.matches(TokenKind::Function) {
            let keyword = self.previous().clone();
            if self.block_depth != 0 {
                return Err(LangError::at(
                    "تعريف الدوال مسموح في المستوى الأعلى فقط في هذه النسخة",
                    &keyword,
                ));
            }
            let name = self.consume(TokenKind::Identifier, "توقعت اسمًا بعد «دالة»")?;
            self.consume(TokenKind::LeftParen, "توقعت «(» بعد اسم الدالة")?;
            let mut parameters = Vec::new();
            if !self.check(TokenKind::RightParen) {
                loop {
                    if parameters.len() >= MAX_FUNCTION_PARAMETERS {
                        return Err(LangError::at(
                            format!("تجاوزت الدالة حد المعاملات ({MAX_FUNCTION_PARAMETERS})"),
                            self.peek(),
                        ));
                    }
                    let parameter = self.consume(TokenKind::Identifier, "توقعت اسم معامل")?;
                    if parameters
                        .iter()
                        .any(|existing: &Token| existing.lexeme == parameter.lexeme)
                    {
                        return Err(LangError::at(
                            format!("اسم معامل مكرر: {}", parameter.lexeme),
                            &parameter,
                        ));
                    }
                    parameters.push(parameter);
                    if !self.matches(TokenKind::Comma) {
                        break;
                    }
                }
            }
            self.consume(TokenKind::RightParen, "توقعت «)» بعد معاملات الدالة")?;
            let opening = self.consume(TokenKind::LeftBrace, "توقعت كتلة «{ ... }» لجسم الدالة")?;
            let body = self.block_statements(&opening)?;
            return Ok(Stmt::Function {
                name,
                parameters,
                body,
            });
        }
        if self.matches(TokenKind::Return) {
            let keyword = self.previous().clone();
            let value = if self.check(TokenKind::Semicolon) {
                None
            } else {
                Some(self.expression()?)
            };
            self.consume(
                TokenKind::Semicolon,
                "أنهِ تعليمة الإرجاع بالفاصلة المنقوطة «؛»",
            )?;
            return Ok(Stmt::Return { keyword, value });
        }
        if self.matches(TokenKind::Let) {
            let name = self.consume(TokenKind::Identifier, "توقعت اسمًا بعد «دع»")?;
            self.consume(TokenKind::Equal, "توقعت «=» بعد اسم المتغير")?;
            let initializer = self.expression()?;
            self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
            return Ok(Stmt::Let { name, initializer });
        }
        if self.matches(TokenKind::Print) {
            let keyword = self.previous().clone();
            let parenthesized = self.matches(TokenKind::LeftParen);
            let expression = self.expression()?;
            if parenthesized {
                self.consume(TokenKind::RightParen, "توقعت «)» بعد قيمة الطباعة")?;
            }
            self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
            return Ok(Stmt::Print {
                expression,
                keyword,
            });
        }
        if self.matches(TokenKind::If) {
            let keyword = self.previous().clone();
            self.consume(TokenKind::LeftParen, "توقعت «(» بعد «إذا»")?;
            let condition = self.expression()?;
            self.consume(TokenKind::RightParen, "توقعت «)» بعد شرط «إذا»")?;
            let then_branch = Box::new(self.required_block("توقعت كتلة «{ ... }» بعد شرط «إذا»")?);
            let else_branch = if self.matches(TokenKind::Else) {
                Some(Box::new(
                    self.required_block("توقعت كتلة «{ ... }» بعد «وإلا»")?,
                ))
            } else {
                None
            };
            return Ok(Stmt::If {
                keyword,
                condition,
                then_branch,
                else_branch,
            });
        }
        if self.matches(TokenKind::While) {
            let keyword = self.previous().clone();
            self.consume(TokenKind::LeftParen, "توقعت «(» بعد «بينما»")?;
            let condition = self.expression()?;
            self.consume(TokenKind::RightParen, "توقعت «)» بعد شرط «بينما»")?;
            let body = Box::new(self.required_block("توقعت كتلة «{ ... }» بعد شرط «بينما»")?);
            return Ok(Stmt::While {
                keyword,
                condition,
                body,
            });
        }
        if self.matches(TokenKind::LeftBrace) {
            let opening = self.previous().clone();
            return Ok(Stmt::Block {
                statements: self.block_statements(&opening)?,
            });
        }
        if self.check(TokenKind::Identifier)
            && self
                .tokens
                .get(self.current + 1)
                .is_some_and(|token| token.kind == TokenKind::Equal)
        {
            let name = self.consume(TokenKind::Identifier, "توقعت اسم متغير قبل الإسناد")?;
            self.consume(TokenKind::Equal, "توقعت «=» بعد اسم المتغير")?;
            let value = self.expression()?;
            self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
            return Ok(Stmt::Assign { name, value });
        }
        let expression = self.expression()?;
        self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
        Ok(Stmt::Expression(expression))
    }

    fn required_block(&mut self, message: &str) -> Result<Stmt, LangError> {
        let opening = self.consume(TokenKind::LeftBrace, message)?;
        Ok(Stmt::Block {
            statements: self.block_statements(&opening)?,
        })
    }

    fn block_statements(&mut self, opening: &Token) -> Result<Vec<Stmt>, LangError> {
        self.enter_nesting(opening)?;
        self.block_depth += 1;
        let result = self.block_statements_inner();
        self.block_depth -= 1;
        self.nesting -= 1;
        result
    }

    fn block_statements_inner(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.check(TokenKind::Eof) {
            statements.push(self.statement()?);
        }
        self.consume(TokenKind::RightBrace, "توقعت «}» لإغلاق الكتلة")?;
        Ok(statements)
    }
    fn expression(&mut self) -> Result<Expr, LangError> {
        self.logical_or()
    }
    fn logical_or(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.logical_and()?;
        while self.matches(TokenKind::Or) {
            let operator = self.previous().clone();
            let right = self.logical_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn logical_and(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.equality()?;
        while self.matches(TokenKind::And) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn equality(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.comparison()?;
        while self.matches_any(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.term()?;
        while self.matches_any(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn term(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.factor()?;
        while self.matches_any(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn factor(&mut self) -> Result<Expr, LangError> {
        let mut expr = self.unary()?;
        while self.matches_any(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, LangError> {
        if self.matches_any(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.nested_unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, LangError> {
        let mut expression = self.primary()?;
        while self.matches(TokenKind::LeftParen) {
            let opening = self.previous().clone();
            self.enter_nesting(&opening)?;
            let parsed_arguments = (|| {
                let mut arguments = Vec::new();
                if !self.check(TokenKind::RightParen) {
                    loop {
                        if arguments.len() >= MAX_FUNCTION_PARAMETERS {
                            return Err(LangError::at(
                                format!("تجاوز الاستدعاء حد المعاملات ({MAX_FUNCTION_PARAMETERS})"),
                                self.peek(),
                            ));
                        }
                        arguments.push(self.expression()?);
                        if !self.matches(TokenKind::Comma) {
                            break;
                        }
                    }
                }
                let paren = self.consume(TokenKind::RightParen, "توقعت «)» بعد معاملات الاستدعاء")?;
                Ok((arguments, paren))
            })();
            self.nesting -= 1;
            let (arguments, paren) = parsed_arguments?;
            let name = match expression {
                Expr::Variable(name) => name,
                _ => {
                    return Err(LangError::at(
                        "يدعم ركن حاليًا استدعاء الدوال المسماة فقط",
                        &paren,
                    ));
                }
            };
            expression = Expr::Call {
                name,
                paren,
                arguments,
            };
        }
        Ok(expression)
    }

    fn primary(&mut self) -> Result<Expr, LangError> {
        if self.matches(TokenKind::False) {
            let token = self.previous().clone();
            return Ok(Expr::Literal {
                value: Value::Bool(false),
                token,
            });
        }
        if self.matches(TokenKind::True) {
            let token = self.previous().clone();
            return Ok(Expr::Literal {
                value: Value::Bool(true),
                token,
            });
        }
        if self.matches(TokenKind::Nil) {
            let token = self.previous().clone();
            return Ok(Expr::Literal {
                value: Value::Nil,
                token,
            });
        }
        if self.matches(TokenKind::Number) {
            let token = self.previous().clone();
            let value = normalize_digits(&token.lexeme)
                .parse::<f64>()
                .map_err(|_| LangError::at("عدد غير صالح", &token))?;
            if !value.is_finite() {
                return Err(LangError::at("يجب أن يكون العدد محدودًا", &token));
            }
            return Ok(Expr::Literal {
                value: Value::Number(value),
                token,
            });
        }
        if self.matches(TokenKind::String) {
            let token = self.previous().clone();
            let value = decode_string_literal(&token)?;
            return Ok(Expr::Literal {
                value: Value::String(value),
                token,
            });
        }
        if self.matches(TokenKind::Identifier) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.matches(TokenKind::LeftParen) {
            let expr = self.nested_expression()?;
            self.consume(TokenKind::RightParen, "توقعت «)» بعد التعبير")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(LangError::at("توقعت قيمة أو تعبيرًا", self.peek()))
    }
    fn matches(&mut self, kind: TokenKind) -> bool {
        if !self.check(kind) {
            return false;
        }
        self.advance();
        true
    }
    fn matches_any(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<Token, LangError> {
        if self.check(kind) {
            return Ok(self.advance().clone());
        }
        Err(LangError::at(message, self.peek()))
    }
    fn check(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }
    fn advance(&mut self) -> &Token {
        if !self.check(TokenKind::Eof) {
            self.current += 1;
        }
        self.previous()
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Function(String),
    Nil,
}
impl Value {
    fn string_bytes(&self) -> usize {
        match self {
            Self::String(value) => value.len(),
            Self::Function(name) => name.len(),
            Self::Number(_) | Self::Bool(_) | Self::Nil => 0,
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) if n.fract() == 0.0 => write!(f, "{n:.0}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Bool(true) => write!(f, "صحيح"),
            Self::Bool(false) => write!(f, "خطأ"),
            Self::Function(name) => write!(f, "<دالة {name}>"),
            Self::Nil => write!(f, "لاشيء"),
        }
    }
}
#[derive(Debug, Clone)]
struct FunctionDef {
    parameters: Vec<Token>,
    body: Arc<Vec<Stmt>>,
}

enum Flow {
    Continue,
    Return { value: Value, keyword: Token },
}

struct Interpreter {
    environment: Vec<HashMap<String, Value>>,
    functions: HashMap<String, FunctionDef>,
    output: Vec<String>,
    evaluation_depth: usize,
    runtime_string_bytes: usize,
    loop_condition_evaluations: usize,
    call_depth: usize,
}
impl Interpreter {
    fn new() -> Self {
        Self {
            environment: vec![HashMap::new()],
            functions: HashMap::new(),
            output: Vec::new(),
            evaluation_depth: 0,
            runtime_string_bytes: 0,
            loop_condition_evaluations: 0,
            call_depth: 0,
        }
    }
    fn reserve_runtime_string_bytes(
        &mut self,
        bytes: usize,
        token: &Token,
    ) -> Result<(), LangError> {
        let Some(total) = self.runtime_string_bytes.checked_add(bytes) else {
            return Err(LangError::at("تجاوز البرنامج ميزانية تخصيص النصوص", token));
        };
        if total > MAX_RUNTIME_STRING_BYTES {
            return Err(LangError::at(
                format!(
                    "تجاوز البرنامج ميزانية النصوص أثناء التنفيذ ({MAX_RUNTIME_STRING_BYTES} بايت)"
                ),
                token,
            ));
        }
        self.runtime_string_bytes = total;
        Ok(())
    }
    fn interpret(mut self, statements: &[Stmt]) -> Result<Vec<String>, LangError> {
        // Register top-level function signatures before executing statements so
        // recursion, mutual recursion, and forward calls have deterministic lookup.
        for statement in statements {
            if let Stmt::Function {
                name,
                parameters,
                body,
                ..
            } = statement
            {
                self.functions.insert(
                    name.lexeme.clone(),
                    FunctionDef {
                        parameters: parameters.clone(),
                        body: Arc::new(body.clone()),
                    },
                );
            }
        }

        for statement in statements {
            match self.execute(statement)? {
                Flow::Continue => {}
                Flow::Return { keyword, .. } => {
                    return Err(LangError::at("لا يمكن استخدام «أرجع» خارج دالة", &keyword));
                }
            }
        }
        Ok(self.output)
    }

    fn define_variable(&mut self, name: &Token, value: Value) -> Result<(), LangError> {
        let Some(scope) = self.environment.last_mut() else {
            return Err(LangError::at("بيئة المتغيرات غير متاحة", name));
        };
        scope.insert(name.lexeme.clone(), value);
        Ok(())
    }

    fn assign_variable(&mut self, name: &Token, value: Value) -> Result<(), LangError> {
        for scope in self.environment.iter_mut().rev() {
            if scope.contains_key(&name.lexeme) {
                scope.insert(name.lexeme.clone(), value);
                return Ok(());
            }
        }
        Err(LangError::at(
            format!("إسناد إلى متغير غير معرّف: {}", name.lexeme),
            name,
        ))
    }

    fn evaluate_loop_condition(&mut self, keyword: &Token) -> Result<(), LangError> {
        let Some(next) = self.loop_condition_evaluations.checked_add(1) else {
            return Err(LangError::at("تجاوز عدّاد الحلقات نطاقه", keyword));
        };
        if next > MAX_LOOP_CONDITION_EVALUATIONS {
            return Err(LangError::at(
                format!(
                    "تجاوز البرنامج الحد الأقصى لتقييم شروط الحلقات ({MAX_LOOP_CONDITION_EVALUATIONS})"
                ),
                keyword,
            ));
        }
        self.loop_condition_evaluations = next;
        Ok(())
    }

    fn execute(&mut self, statement: &Stmt) -> Result<Flow, LangError> {
        let flow = match statement {
            Stmt::Function { .. } => Flow::Continue,
            Stmt::Return { keyword, value } => {
                let value = match value {
                    Some(expression) => self.evaluate(expression)?,
                    None => Value::Nil,
                };
                Flow::Return {
                    value,
                    keyword: keyword.clone(),
                }
            }
            Stmt::Let { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.define_variable(name, value)?;
                Flow::Continue
            }
            Stmt::Assign { name, value } => {
                let value = self.evaluate(value)?;
                self.assign_variable(name, value)?;
                Flow::Continue
            }
            Stmt::Print {
                expression,
                keyword,
            } => {
                let value = self.evaluate(expression)?;
                self.reserve_runtime_string_bytes(value.string_bytes(), keyword)?;
                self.output.push(value.to_string());
                Flow::Continue
            }
            Stmt::Block { statements } => {
                self.environment.push(HashMap::new());
                let result = (|| {
                    let mut result = Flow::Continue;
                    for statement in statements {
                        result = self.execute(statement)?;
                        if matches!(&result, Flow::Return { .. }) {
                            break;
                        }
                    }
                    Ok(result)
                })();
                self.environment.pop();
                result?
            }
            Stmt::If {
                keyword,
                condition,
                then_branch,
                else_branch,
            } => match self.evaluate(condition)? {
                Value::Bool(true) => self.execute(then_branch)?,
                Value::Bool(false) => match else_branch {
                    Some(branch) => self.execute(branch)?,
                    None => Flow::Continue,
                },
                _ => {
                    return Err(LangError::at("يتطلب شرط «إذا» قيمة منطقية", keyword));
                }
            },
            Stmt::While {
                keyword,
                condition,
                body,
            } => loop {
                self.evaluate_loop_condition(keyword)?;
                match self.evaluate(condition)? {
                    Value::Bool(true) => {
                        let flow = self.execute(body)?;
                        if matches!(&flow, Flow::Return { .. }) {
                            break flow;
                        }
                    }
                    Value::Bool(false) => break Flow::Continue,
                    _ => {
                        return Err(LangError::at("يتطلب شرط «بينما» قيمة منطقية", keyword));
                    }
                }
            },
            Stmt::Expression(expression) => {
                self.evaluate(expression)?;
                Flow::Continue
            }
        };
        Ok(flow)
    }

    fn call_function(
        &mut self,
        function_name: &str,
        callsite: &Token,
        closing_paren: &Token,
        arguments: &[Expr],
    ) -> Result<Value, LangError> {
        let function = self
            .functions
            .get(function_name)
            .cloned()
            .ok_or_else(|| LangError::at(format!("دالة غير معرّفة: {function_name}"), callsite))?;
        if arguments.len() != function.parameters.len() {
            return Err(LangError::at(
                format!(
                    "الدالة «{}» تتطلب {} معاملًا، لكن استُخدم {}",
                    function_name,
                    function.parameters.len(),
                    arguments.len()
                ),
                closing_paren,
            ));
        }
        if self.call_depth >= MAX_CALL_DEPTH {
            return Err(LangError::at(
                format!("تجاوز البرنامج الحد الأقصى لعمق استدعاء الدوال ({MAX_CALL_DEPTH})"),
                callsite,
            ));
        }

        let mut values = Vec::with_capacity(arguments.len());
        for argument in arguments {
            values.push(self.evaluate(argument)?);
        }
        let mut parameter_scope = HashMap::with_capacity(function.parameters.len());
        for (parameter, value) in function.parameters.iter().zip(values) {
            parameter_scope.insert(parameter.lexeme.clone(), value);
        }

        let mut global_string_bytes = 0usize;
        if let Some(global_scope) = self.environment.first() {
            for value in global_scope.values() {
                let Some(total) = global_string_bytes.checked_add(value.string_bytes()) else {
                    return Err(LangError::at(
                        "تجاوز ميزانية النصوص في النطاق العام",
                        callsite,
                    ));
                };
                global_string_bytes = total;
            }
        }
        self.reserve_runtime_string_bytes(global_string_bytes, callsite)?;
        let global_scope = self.environment.first().cloned().unwrap_or_default();
        let mut caller_environment = std::mem::take(&mut self.environment);
        self.environment = vec![global_scope, parameter_scope];
        self.call_depth += 1;

        // Expression nesting is bounded within each function frame. Keep it
        // separate from the independent recursion/call-depth budget.
        let caller_evaluation_depth = self.evaluation_depth;
        self.evaluation_depth = 0;

        let execution = (|| {
            let mut flow = Flow::Continue;
            for statement in function.body.iter() {
                flow = self.execute(statement)?;
                if matches!(&flow, Flow::Return { .. }) {
                    break;
                }
            }
            Ok(flow)
        })();

        self.evaluation_depth = caller_evaluation_depth;
        self.call_depth -= 1;
        let callee_environment = std::mem::take(&mut self.environment);
        if let Some(caller_globals) = caller_environment.first_mut() {
            *caller_globals = callee_environment.into_iter().next().unwrap_or_default();
        }
        self.environment = caller_environment;

        match execution? {
            Flow::Continue => Ok(Value::Nil),
            Flow::Return { value, .. } => Ok(value),
        }
    }
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LangError> {
        if self.evaluation_depth >= MAX_NESTING {
            let token = match expr {
                Expr::Variable(token) => Some(token),
                Expr::Call { name, .. } => Some(name),
                Expr::Unary { operator, .. }
                | Expr::Binary { operator, .. }
                | Expr::Logical { operator, .. } => Some(operator),
                Expr::Literal { token, .. } => Some(token),
                Expr::Grouping(_) => None,
            };
            return Err(match token {
                Some(token) => LangError::at(
                    format!("تجاوز التنفيذ الحد الأقصى لعمق التعبير ({MAX_NESTING})"),
                    token,
                ),
                None => LangError {
                    message: format!("تجاوز التنفيذ الحد الأقصى لعمق التعبير ({MAX_NESTING})"),
                    line: 1,
                    column: 1,
                },
            });
        }
        self.evaluation_depth += 1;
        let result = self.evaluate_inner(expr);
        self.evaluation_depth -= 1;
        result
    }
    fn evaluate_inner(&mut self, expr: &Expr) -> Result<Value, LangError> {
        match expr {
            Expr::Literal { value, token } => {
                self.reserve_runtime_string_bytes(value.string_bytes(), token)?;
                Ok(value.clone())
            }
            Expr::Variable(name) => {
                let value = self
                    .environment
                    .iter()
                    .rev()
                    .find_map(|scope| scope.get(&name.lexeme))
                    .cloned()
                    .or_else(|| {
                        self.functions
                            .contains_key(&name.lexeme)
                            .then(|| Value::Function(name.lexeme.clone()))
                    })
                    .ok_or_else(|| {
                        LangError::at(format!("متغير غير معرّف: {}", name.lexeme), name)
                    })?;
                self.reserve_runtime_string_bytes(value.string_bytes(), name)?;
                Ok(value)
            }
            Expr::Call {
                name,
                paren,
                arguments,
            } => {
                let callee = self
                    .environment
                    .iter()
                    .rev()
                    .find_map(|scope| scope.get(&name.lexeme))
                    .cloned()
                    .or_else(|| {
                        self.functions
                            .contains_key(&name.lexeme)
                            .then(|| Value::Function(name.lexeme.clone()))
                    })
                    .ok_or_else(|| {
                        LangError::at(format!("دالة غير معرّفة: {}", name.lexeme), name)
                    })?;
                self.reserve_runtime_string_bytes(callee.string_bytes(), name)?;
                match callee {
                    Value::Function(function_name) => {
                        self.call_function(&function_name, name, paren, arguments)
                    }
                    _ => Err(LangError::at(
                        format!("القيمة «{}» ليست دالة قابلة للاستدعاء", name.lexeme),
                        name,
                    )),
                }
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary { operator, right } => {
                let right = self.evaluate(right)?;
                match (operator.kind, right) {
                    (TokenKind::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
                    (TokenKind::Bang, Value::Bool(b)) => Ok(Value::Bool(!b)),
                    (TokenKind::Minus, _) => Err(LangError::at("تتطلب «-» عددًا", operator)),
                    (TokenKind::Bang, _) => Err(LangError::at("تتطلب «!» قيمة منطقية", operator)),
                    _ => unreachable!("parser only constructs valid unary operators"),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
                self.binary(left, operator, right)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(left)?;
                match (operator.kind, left) {
                    (TokenKind::Or, Value::Bool(true)) => Ok(Value::Bool(true)),
                    (TokenKind::And, Value::Bool(false)) => Ok(Value::Bool(false)),
                    (TokenKind::Or, Value::Bool(false)) | (TokenKind::And, Value::Bool(true)) => {
                        match self.evaluate(right)? {
                            Value::Bool(value) => Ok(Value::Bool(value)),
                            _ => Err(LangError::at(
                                format!("يتطلب العامل «{}» قيمتين منطقيتين", operator.lexeme),
                                operator,
                            )),
                        }
                    }
                    _ => Err(LangError::at(
                        format!("يتطلب العامل «{}» قيمتين منطقيتين", operator.lexeme),
                        operator,
                    )),
                }
            }
        }
    }
    fn binary(&mut self, left: Value, op: &Token, right: Value) -> Result<Value, LangError> {
        use TokenKind as K;
        match (op.kind, left, right) {
            (K::Plus, Value::Number(a), Value::Number(b)) => finite_number(a + b, op),
            (K::Plus, Value::String(a), Value::String(b)) => {
                let Some(length) = a.len().checked_add(b.len()) else {
                    return Err(LangError::at("تجاوز طول النص الحد المدعوم", op));
                };
                if length > MAX_STRING_BYTES {
                    return Err(LangError::at(
                        format!("تجاوز النص الناتج الحد الأقصى ({MAX_STRING_BYTES} بايت)"),
                        op,
                    ));
                }
                self.reserve_runtime_string_bytes(length, op)?;
                let mut combined = String::with_capacity(length);
                combined.push_str(&a);
                combined.push_str(&b);
                Ok(Value::String(combined))
            }
            (K::Minus, Value::Number(a), Value::Number(b)) => finite_number(a - b, op),
            (K::Star, Value::Number(a), Value::Number(b)) => finite_number(a * b, op),
            (K::Slash, Value::Number(_), Value::Number(0.0)) => {
                Err(LangError::at("القسمة على صفر غير مسموحة", op))
            }
            (K::Slash, Value::Number(a), Value::Number(b)) => finite_number(a / b, op),
            (K::EqualEqual, a, b) => Ok(Value::Bool(a == b)),
            (K::BangEqual, a, b) => Ok(Value::Bool(a != b)),
            (K::Greater, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
            (K::GreaterEqual, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
            (K::Less, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
            (K::LessEqual, Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
            (K::Plus, _, _) => Err(LangError::at(
                "يدعم «+» جمع عددين أو ضم نصين من النوع نفسه",
                op,
            )),
            (_, _, _) => Err(LangError::at(
                "نوعا القيمتين غير متوافقين مع هذه العملية",
                op,
            )),
        }
    }
}

/// Parse and evaluate a Rukn program, returning lines emitted by the print statement.
pub fn run(source: &str) -> Result<Vec<String>, LangError> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(LangError {
            message: format!("تجاوز حجم المصدر الحد الأقصى ({MAX_SOURCE_BYTES} بايت)"),
            line: 1,
            column: 1,
        });
    }
    let tokens = Lexer::new(source).scan()?;
    let statements = Parser::new(tokens).parse()?;
    Interpreter::new().interpret(&statements)
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_CALL_DEPTH, MAX_LOOP_CONDITION_EVALUATIONS, MAX_NESTING, MAX_RUNTIME_STRING_BYTES,
        MAX_SOURCE_BYTES, run,
    };

    #[test]
    fn arabic_program_and_digits() {
        assert_eq!(run("دع العدد = ٢ + ٣ * ٤؛ اطبع(العدد)؛").unwrap(), ["14"]);
    }
    #[test]
    fn supports_strings_and_arabic_booleans() {
        assert_eq!(
            run("اطبع(«مرحبًا» + \" بالعالم\")؛ اطبع(صحيح == خطأ)؛").unwrap(),
            ["مرحبًا بالعالم", "خطأ"]
        );
    }
    #[test]
    fn accepts_xid_identifiers_with_arabic_marks_and_persian_letters() {
        let source = "دع عَلم = ٧؛ اطبع(عَلم)؛ دع پ = ۳؛ اطبع(پ)؛";
        assert_eq!(run(source).unwrap(), ["7", "3"]);
    }
    #[test]
    fn accepts_supplementary_plane_unicode_identifiers() {
        // U+10400 (DESERET CAPITAL LETTER LONG I) is an XID_Start character.
        // This guards the scanner against accidental BMP-only identifier logic.
        let source = "دع 𐐀اسم = ٨؛ اطبع(𐐀اسم)؛";
        assert_eq!(run(source).unwrap(), ["8"]);
    }
    #[test]
    fn accepts_all_documented_digit_families() {
        assert_eq!(
            run("اطبع(123)؛ اطبع(١٢٣)؛ اطبع(۱۲۳)؛").unwrap(),
            ["123", "123", "123"]
        );
    }
    #[test]
    fn accepts_ascii_and_arabic_decimal_separators_with_all_digit_families() {
        assert_eq!(
            run("اطبع(12.5)؛ اطبع(١٢٫٥)؛ اطبع(۱۲٫۵)؛ اطبع(١٢.٥)؛").unwrap(),
            ["12.5", "12.5", "12.5", "12.5"]
        );
    }

    #[test]
    fn rejects_mixed_digit_families_inside_one_numeric_literal() {
        for (source, expected_column) in [
            ("اطبع(1٢)؛", 7),
            ("اطبع(١2)؛", 7),
            ("اطبع(۱۲٫٣)؛", 9),
            ("اطبع(12.٥)؛", 9),
        ] {
            let error = run(source).unwrap_err();
            assert!(
                error.message.contains("خلط مجموعات الأرقام"),
                "unexpected diagnostic for {source:?}: {}",
                error.message
            );
            assert_eq!(
                (error.line, error.column),
                (1, expected_column),
                "{source:?}"
            );
        }
    }

    #[test]
    fn rejects_repeated_arabic_decimal_separator() {
        let error = run("اطبع(١٫٢٫٣)؛").unwrap_err();
        assert!(error.message.contains("غير مدعوم"));
        assert!(error.line > 0);
        assert!(error.column > 0);
    }

    #[test]
    fn rejects_a_combining_mark_at_identifier_start() {
        let error = run("اطبع(َاسم)؛").unwrap_err();
        assert!(error.message.contains("غير مدعوم"));
    }
    #[test]
    fn respects_precedence_and_parentheses() {
        assert_eq!(
            run("اطبع(2 + 3 * 4)؛ اطبع((2 + 3) * 4)؛").unwrap(),
            ["14", "20"]
        );
    }
    #[test]
    fn logical_operators_short_circuit_and_require_booleans() {
        assert_eq!(
            run("اطبع(خطأ و (١ / ٠ == ٠))؛ اطبع(صحيح أو (١ / ٠ == ٠))؛").unwrap(),
            ["خطأ", "صحيح"]
        );
        assert_eq!(
            run("اطبع(خطأ و ١)؛ اطبع(صحيح أو ١)؛").unwrap(),
            ["خطأ", "صحيح"]
        );

        let error = run("اطبع(١ و صحيح)؛").unwrap_err();
        assert!(error.message.contains("قيمتين منطقيتين"));
        let right_error = run("اطبع(صحيح و ١)؛").unwrap_err();
        assert!(right_error.message.contains("قيمتين منطقيتين"));
        assert!(error.line > 0);
        assert!(error.column > 0);
    }

    #[test]
    fn logical_operator_precedence_and_parentheses() {
        assert_eq!(
            run("اطبع(صحيح أو خطأ و خطأ)؛ اطبع((صحيح أو خطأ) و خطأ)؛ اطبع(خطأ و صحيح أو صحيح)؛")
                .unwrap(),
            ["صحيح", "خطأ", "صحيح"]
        );
    }

    #[test]
    fn reports_locations_after_lf_crlf_and_bare_cr_line_endings() {
        for source in [
            "# comment\nاطبع(غير_معرف)؛",
            "# comment\r\nاطبع(غير_معرف)؛",
            "# comment\rاطبع(غير_معرف)؛",
        ] {
            let error = run(source).unwrap_err();
            assert!(error.message.contains("غير معرّف"), "{source:?}");
            assert_eq!((error.line, error.column), (2, 6), "{source:?}");
        }
    }

    #[test]
    fn undefined_variables_have_source_location() {
        let error = run("اطبع(غير_معرف)؛").unwrap_err();
        assert!(error.message.contains("غير معرّف"));
        assert_eq!((error.line, error.column), (1, 6));
    }
    #[test]
    fn division_by_zero_is_an_error() {
        assert!(run("اطبع(8 / 0)؛").unwrap_err().message.contains("صفر"));
    }
    #[test]
    fn invalid_characters_and_unclosed_strings_are_errors() {
        assert!(run("اطبع(@)؛").unwrap_err().message.contains("غير مدعوم"));
        assert!(run("اطبع(\"نص)؛").unwrap_err().message.contains("غير مغلق"));
    }
    #[test]
    fn oversized_source_is_rejected_before_lexing() {
        let source = " ".repeat(MAX_SOURCE_BYTES + 1);
        let error = run(&source).unwrap_err();
        assert!(error.message.contains("حجم المصدر"));
    }
    #[test]
    fn deeply_nested_parentheses_are_rejected() {
        let source = format!(
            "اطبع({}1{})؛",
            "(".repeat(MAX_NESTING + 1),
            ")".repeat(MAX_NESTING + 1)
        );
        let error = run(&source).unwrap_err();
        assert!(!error.message.trim().is_empty());
        assert!(error.line > 0);
        assert!(error.column > 0);
    }
    #[test]
    fn long_left_associative_expression_is_bounded() {
        let source = format!("اطبع({}1)؛", "1+".repeat(MAX_NESTING + 20));
        let error = run(&source).unwrap_err();
        assert!(error.message.contains("عمق التعبير"));
    }
    #[test]
    fn computed_numeric_overflow_is_a_language_error() {
        // Scientific notation is not in the grammar. Large decimal literals
        // let this test exercise overflowing arithmetic directly.
        let left = "9".repeat(250);
        let right = "9".repeat(100);
        let source = format!("اطبع({left} * {right})؛");
        let error = run(&source).unwrap_err();
        assert!(error.message.contains("غير محدود"));
    }
    #[test]
    fn rejected_unicode_controls_are_visible_in_diagnostics() {
        // U+202E is RIGHT-TO-LEFT OVERRIDE, an invisible bidirectional control.
        let error = run("دع الاسم = \u{202E}1؛").unwrap_err();
        assert!(
            error.message.contains("U+202E"),
            "diagnostic should show the code point: {}",
            error.message
        );
        assert!(
            error.message.to_ascii_lowercase().contains("\\u{202e}"),
            "diagnostic should show an escaped representation: {}",
            error.message
        );
    }
    #[test]
    fn string_escapes_are_decoded_consistently_for_both_delimiters() {
        assert_eq!(
            run(r#"اطبع("سطر\nثانٍ\t\\\"")؛ اطبع(«قالت \»نعم\»»)؛"#).unwrap(),
            ["سطر\nثانٍ\t\\\"", "قالت »نعم»"]
        );
    }

    #[test]
    fn rejects_physical_line_breaks_in_strings_at_the_break_position() {
        for (source, expected_location) in [
            ("اطبع(\"سطر\rثانٍ\")؛", (1, 10)),
            ("اطبع(\"سطر\nثانٍ\")؛", (1, 10)),
            ("اطبع(\"سطر\\\nثانٍ\")؛", (1, 11)),
            ("اطبع(\"سطر\\\rثانٍ\")؛", (1, 11)),
        ] {
            let error = run(source).unwrap_err();
            assert!(
                error.message.contains("سطر جديد"),
                "{source:?}: {}",
                error.message
            );
            assert_eq!((error.line, error.column), expected_location, "{source:?}");
        }
    }

    #[test]
    fn unterminated_strings_point_to_end_of_input() {
        let error = run("اطبع(\"نص").unwrap_err();
        assert!(error.message.contains("غير مغلق"));
        assert_eq!((error.line, error.column), (1, 9));
    }

    #[test]
    fn unsupported_string_escape_is_a_language_error() {
        let error = run(r#"اطبع("خطأ\q")؛"#).unwrap_err();
        assert!(error.message.contains("تسلسل هروب غير مدعوم"));
        assert_eq!((error.line, error.column), (1, 6));
    }

    #[test]
    fn runtime_string_allocation_budget_rejects_repeated_large_copies() {
        let literal = "x".repeat(128 * 1024);
        let mut source = format!("دع النص = «{literal}»؛\n");
        for index in 0..(MAX_RUNTIME_STRING_BYTES / (128 * 1024) + 2) {
            source.push_str(&format!("دع نسخة_{index} = النص؛\n"));
        }
        assert!(source.len() < MAX_SOURCE_BYTES);
        let error = run(&source).unwrap_err();
        assert!(
            error.message.contains("ميزانية النصوص"),
            "unexpected diagnostic: {}",
            error.message
        );
        assert!(error.line > 0 && error.column > 0);
    }

    #[test]
    fn concatenated_string_values_are_bounded() {
        let long_literal = "x".repeat(MAX_SOURCE_BYTES * 3 / 5);
        let source = format!("دع النص = «{long_literal}»؛ دع الناتج = النص + النص؛");
        assert!(source.len() < MAX_SOURCE_BYTES);
        let error = run(&source).unwrap_err();
        assert!(error.message.contains("الحد الأقصى"), "{}", error.message);
        assert!(error.line > 0 && error.column > 0);
    }

    #[test]
    fn repeated_prints_respect_runtime_string_allocation_budget() {
        let literal = "x".repeat(128 * 1024);
        let mut source = format!("دع النص = «{literal}»؛\n");
        for _ in 0..(MAX_RUNTIME_STRING_BYTES / (128 * 1024) + 2) {
            source.push_str("اطبع(النص)؛\n");
        }
        assert!(source.len() < MAX_SOURCE_BYTES);
        let error = run(&source).unwrap_err();
        assert!(
            error.message.contains("ميزانية النصوص"),
            "unexpected diagnostic: {}",
            error.message
        );
        assert!(error.line > 0 && error.column > 0);
    }

    #[test]
    fn conditional_blocks_select_the_expected_branch() {
        assert_eq!(
            run("دع العدد = ٣؛ إذا (العدد > ٢) { اطبع(«كبير»)؛ } وإلا { اطبع(«صغير»)؛ }").unwrap(),
            ["كبير"]
        );
        assert_eq!(
            run("إذا (خطأ) { اطبع(١)؛ } وإلا { اطبع(٢)؛ }").unwrap(),
            ["2"]
        );
    }

    #[test]
    fn while_loops_and_assignment_work() {
        assert_eq!(
            run("دع العد = ٠؛ بينما (العد < ٣) { اطبع(العد)؛ العد = العد + ١؛ }").unwrap(),
            ["0", "1", "2"]
        );
    }

    #[test]
    fn lexical_scopes_shadow_without_leaking_and_assign_nearest_binding() {
        assert_eq!(
            run("دع الاسم = «خارج»؛ { دع الاسم = «داخل»؛ الاسم = «معدل»؛ اطبع(الاسم)؛ } اطبع(الاسم)؛")
                .unwrap(),
            ["معدل", "خارج"]
        );
    }

    #[test]
    fn assignment_to_an_undefined_variable_is_an_error() {
        let error = run("غائب = ١؛").unwrap_err();
        assert!(error.message.contains("غير معرّف"));
        assert_eq!((error.line, error.column), (1, 1));
    }

    #[test]
    fn control_flow_conditions_require_booleans() {
        let if_error = run("إذا (١) { اطبع(١)؛ }").unwrap_err();
        assert!(if_error.message.contains("شرط «إذا»"));
        assert!(if_error.message.contains("منطقية"));

        let while_error = run("بينما («نعم») { }").unwrap_err();
        assert!(while_error.message.contains("شرط «بينما»"));
        assert!(while_error.message.contains("منطقية"));
    }

    #[test]
    fn infinite_loops_are_bounded() {
        let error = run("بينما (صحيح) { }").unwrap_err();
        assert!(error.message.contains("تقييم شروط الحلقات"));
        assert!(error.line > 0 && error.column > 0);
        assert_eq!(MAX_LOOP_CONDITION_EVALUATIONS, 1_000_000);
    }

    #[test]
    fn deeply_nested_blocks_are_rejected() {
        let source = format!(
            "{}{}",
            "{".repeat(MAX_NESTING + 1),
            "}".repeat(MAX_NESTING + 1)
        );
        let error = run(&source).unwrap_err();
        assert!(error.message.contains("الحد الأقصى للتداخل"));
        assert!(error.line > 0 && error.column > 0);
    }

    #[test]
    fn named_functions_accept_arguments_and_return_values() {
        assert_eq!(
            run("دالة جمع(أ, ب) { أرجع أ + ب؛ } اطبع(جمع(٢٠, ٢٢))؛").unwrap(),
            ["42"]
        );
        assert_eq!(
            run("دالة رسالة() { اطبع(«داخل»)؛ } اطبع(رسالة())؛").unwrap(),
            ["داخل", "لاشيء"]
        );
    }

    #[test]
    fn recursive_functions_work() {
        let source =
            "دالة عاملي(ن) { إذا (ن <= ١) { أرجع ١؛ } أرجع ن * عاملي(ن - ١)؛ } اطبع(عاملي(٥))؛";
        assert_eq!(run(source).unwrap(), ["120"]);
    }

    #[test]
    fn functions_can_call_later_definitions_and_mutually_recurse() {
        let source = "دالة أ() { أرجع ب()؛ } دالة ب() { أرجع ٧؛ } اطبع(أ())؛";
        assert_eq!(run(source).unwrap(), ["7"]);

        let mutual = "دالة زوجي(ن) { إذا (ن == ٠) { أرجع صحيح؛ } أرجع فردي(ن - ١)؛ } دالة فردي(ن) { إذا (ن == ٠) { أرجع خطأ؛ } أرجع زوجي(ن - ١)؛ } اطبع(زوجي(٦))؛ اطبع(فردي(٧))؛";
        assert_eq!(run(mutual).unwrap(), ["صحيح", "صحيح"]);
    }

    #[test]
    fn function_calls_can_update_globals_without_capturing_caller_locals() {
        let source =
            "دع عام = ١؛ دالة زد(زيادة) { عام = عام + زيادة؛ أرجع عام؛ } اطبع(زد(٢))؛ اطبع(عام)؛";
        assert_eq!(run(source).unwrap(), ["3", "3"]);

        let dynamic_scope = "دالة اقرأ() { أرجع محلي؛ } { دع محلي = ٢؛ اطبع(اقرأ())؛ }";
        let error = run(dynamic_scope).unwrap_err();
        assert!(error.message.contains("غير معرّف"));
    }

    #[test]
    fn function_errors_are_reported_with_source_locations() {
        let return_error = run("أرجع ١؛").unwrap_err();
        assert!(return_error.message.contains("خارج دالة"));

        let undefined_error = run("مجهولة(١)؛").unwrap_err();
        assert!(undefined_error.message.contains("دالة غير معرّفة"));

        let arity_error = run("دالة واحدة(أ) { أرجع أ؛ } اطبع(واحدة())؛").unwrap_err();
        assert!(arity_error.message.contains("تتطلب 1 معاملًا"));

        let duplicate_parameter = run("دالة تكرار(أ, أ) { أرجع أ؛ }").unwrap_err();
        assert!(duplicate_parameter.message.contains("معامل مكرر"));

        let duplicate_function =
            run("دالة حساب() { أرجع ١؛ } دالة حساب() { أرجع ٢؛ }").unwrap_err();
        assert!(duplicate_function.message.contains("تعريف دالة مكرر"));

        let nested_definition = run("{ دالة محلية() { أرجع ١؛ } }").unwrap_err();
        assert!(nested_definition.message.contains("المستوى الأعلى"));
    }

    #[test]
    fn recursive_function_calls_are_bounded() {
        let error = run("دالة تكرار() { أرجع تكرار()؛ } اطبع(تكرار())؛").unwrap_err();
        assert!(error.message.contains("عمق استدعاء الدوال"));
        assert_eq!(MAX_CALL_DEPTH, 128);
    }

    #[test]
    fn named_function_values_can_be_assigned_passed_and_returned() {
        let alias = "دالة جمع(أ, ب) { أرجع أ + ب؛ } دع عملية = جمع؛ اطبع(عملية(٢, ٣))؛";
        assert_eq!(run(alias).unwrap(), ["5"]);

        let higher_order = "دالة جمع(أ, ب) { أرجع أ + ب؛ } دالة تطبيق(عملية, أ, ب) { أرجع عملية(أ, ب)؛ } اطبع(تطبيق(جمع, ٤, ٥))؛";
        assert_eq!(run(higher_order).unwrap(), ["9"]);

        let returned = "دالة جمع(أ, ب) { أرجع أ + ب؛ } دالة موفر() { أرجع جمع؛ } دع مجموع = موفر()؛ اطبع(مجموع(٧, ٨))؛";
        assert_eq!(run(returned).unwrap(), ["15"]);

        let equality = "دالة أولى() { أرجع ١؛ } دالة ثانية() { أرجع ١؛ } اطبع(أولى == أولى)؛ اطبع(أولى != ثانية)؛";
        assert_eq!(run(equality).unwrap(), ["صحيح", "صحيح"]);
    }

    #[test]
    fn function_reference_name_copies_respect_runtime_budget() {
        let long_name = "م".repeat(160_000);
        let source = format!(
            "دالة {long_name}() {{ أرجع ١؛ }} دع ف = {long_name}؛ دع عد = ٠؛ بينما (عد < 30) {{ ف()؛ عد = عد + ١؛ }}"
        );
        assert!(source.len() < MAX_SOURCE_BYTES);
        let error = run(&source).unwrap_err();
        assert!(
            error.message.contains("ميزانية النصوص"),
            "unexpected diagnostic: {}",
            error.message
        );
    }

    #[test]
    fn non_callable_variable_shadowing_a_function_is_a_diagnostic() {
        let source = "دالة جمع(أ, ب) { أرجع أ + ب؛ } دع جمع = ١؛ جمع(٢, ٣)؛";
        let error = run(source).unwrap_err();
        assert!(error.message.contains("ليست دالة قابلة للاستدعاء"));
    }

    #[test]
    fn empty_program_is_valid() {
        assert!(run("").unwrap().is_empty());
    }
    #[test]
    fn incompatible_types_are_not_coerced() {
        assert!(
            run("اطبع(1 + \"2\")؛")
                .unwrap_err()
                .message
                .contains("جمع عددين")
        );
    }
}
