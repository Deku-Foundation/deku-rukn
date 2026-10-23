//! A small Arabic-first interpreter. This milestone is experimental, not a stable language ABI.
use std::collections::HashMap;
use std::fmt;

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
    Identifier,
    Number,
    String,
    Let,
    Print,
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
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                ' ' | '\r' | '\t' | '\n' => {}
                '"' | '«' => tokens.push(self.string(ch)?),
                c if is_digit(c) => tokens.push(self.number()),
                c if is_identifier_start(c) => tokens.push(self.identifier()),
                _ => return Err(self.error_at_current(format!("محرف غير مدعوم: {ch}"))),
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
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
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
            if self.peek() == '\n' {
                return Err(self.error_at_current("لا يُسمح بسطر جديد داخل النص".into()));
            }
            if self.peek() == '\\' && self.peek_next() != '\0' {
                self.advance();
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(self.error_at_current("نص غير مغلق".into()));
        }
        self.advance();
        Ok(self.token(TokenKind::String))
    }
    fn number(&mut self) -> Token {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        self.token(TokenKind::Number)
    }
    fn identifier(&mut self) -> Token {
        while is_identifier_continue(self.peek()) {
            self.advance();
        }
        let text: String = self.chars[self.start..self.current].iter().collect();
        let kind = match text.as_str() {
            "دع" => TokenKind::Let,
            "اطبع" => TokenKind::Print,
            "صحيح" => TokenKind::True,
            "خطأ" => TokenKind::False,
            "لاشيء" => TokenKind::Nil,
            _ => TokenKind::Identifier,
        };
        self.token(kind)
    }
    fn error_at_current(&self, message: String) -> LangError {
        LangError {
            message,
            line: self.token_line,
            column: self.token_column,
        }
    }
}
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit() || ('٠'..='٩').contains(&ch) || ('۰'..='۹').contains(&ch)
}
fn normalize_digits(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            '٠'..='٩' => char::from(b'0' + (ch as u32 - '٠' as u32) as u8),
            '۰'..='۹' => char::from(b'0' + (ch as u32 - '۰' as u32) as u8),
            _ => ch,
        })
        .collect()
}
fn is_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_alphabetic()
}
fn is_identifier_continue(ch: char) -> bool {
    is_identifier_start(ch) || ch.is_numeric()
}

#[derive(Debug, Clone)]
enum Expr {
    Literal(Value),
    Variable(Token),
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
}
#[derive(Debug, Clone)]
enum Stmt {
    Let { name: Token, initializer: Expr },
    Print { expression: Expr },
    Expression(Expr),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    fn parse(&mut self) -> Result<Vec<Stmt>, LangError> {
        let mut statements = Vec::new();
        while !self.check(TokenKind::Eof) {
            statements.push(self.statement()?);
        }
        Ok(statements)
    }
    fn statement(&mut self) -> Result<Stmt, LangError> {
        if self.matches(TokenKind::Let) {
            let name = self.consume(TokenKind::Identifier, "توقعت اسمًا بعد «دع»")?;
            self.consume(TokenKind::Equal, "توقعت «=» بعد اسم المتغير")?;
            let initializer = self.expression()?;
            self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
            return Ok(Stmt::Let { name, initializer });
        }
        if self.matches(TokenKind::Print) {
            let parenthesized = self.matches(TokenKind::LeftParen);
            let expression = self.expression()?;
            if parenthesized {
                self.consume(TokenKind::RightParen, "توقعت «)» بعد قيمة الطباعة")?;
            }
            self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
            return Ok(Stmt::Print { expression });
        }
        let expression = self.expression()?;
        self.consume(TokenKind::Semicolon, "أنهِ التعليمة بالفاصلة المنقوطة «؛»")?;
        Ok(Stmt::Expression(expression))
    }
    fn expression(&mut self) -> Result<Expr, LangError> {
        self.equality()
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
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }
    fn primary(&mut self) -> Result<Expr, LangError> {
        if self.matches(TokenKind::False) {
            return Ok(Expr::Literal(Value::Bool(false)));
        }
        if self.matches(TokenKind::True) {
            return Ok(Expr::Literal(Value::Bool(true)));
        }
        if self.matches(TokenKind::Nil) {
            return Ok(Expr::Literal(Value::Nil));
        }
        if self.matches(TokenKind::Number) {
            let token = self.previous().clone();
            let value = normalize_digits(&token.lexeme)
                .parse::<f64>()
                .map_err(|_| LangError::at("عدد غير صالح", &token))?;
            if !value.is_finite() {
                return Err(LangError::at("يجب أن يكون العدد محدودًا", &token));
            }
            return Ok(Expr::Literal(Value::Number(value)));
        }
        if self.matches(TokenKind::String) {
            let token = self.previous().clone();
            let opener = token.lexeme.chars().next().unwrap_or('"');
            let closer = if opener == '«' { '»' } else { '"' };
            let value = token
                .lexeme
                .trim_start_matches(opener)
                .trim_end_matches(closer)
                .replace("\\\"", "\"")
                .replace("\\\\", "\\");
            return Ok(Expr::Literal(Value::String(value)));
        }
        if self.matches(TokenKind::Identifier) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.matches(TokenKind::LeftParen) {
            let expr = self.expression()?;
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
    Nil,
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) if n.fract() == 0.0 => write!(f, "{n:.0}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Bool(true) => write!(f, "صحيح"),
            Self::Bool(false) => write!(f, "خطأ"),
            Self::Nil => write!(f, "لاشيء"),
        }
    }
}
struct Interpreter {
    environment: HashMap<String, Value>,
    output: Vec<String>,
}
impl Interpreter {
    fn new() -> Self {
        Self {
            environment: HashMap::new(),
            output: Vec::new(),
        }
    }
    fn interpret(mut self, statements: &[Stmt]) -> Result<Vec<String>, LangError> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(self.output)
    }
    fn execute(&mut self, statement: &Stmt) -> Result<(), LangError> {
        match statement {
            Stmt::Let { name, initializer } => {
                let value = self.evaluate(initializer)?;
                self.environment.insert(name.lexeme.clone(), value);
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?.to_string();
                self.output.push(value);
            }
            Stmt::Expression(expression) => {
                self.evaluate(expression)?;
            }
        }
        Ok(())
    }
    fn evaluate(&mut self, expr: &Expr) -> Result<Value, LangError> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Variable(name) => self
                .environment
                .get(&name.lexeme)
                .cloned()
                .ok_or_else(|| LangError::at(format!("متغير غير معرّف: {}", name.lexeme), name)),
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
        }
    }
    fn binary(&self, left: Value, op: &Token, right: Value) -> Result<Value, LangError> {
        use TokenKind as K;
        match (op.kind, left, right) {
            (K::Plus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (K::Plus, Value::String(a), Value::String(b)) => Ok(Value::String(a + &b)),
            (K::Minus, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (K::Star, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (K::Slash, Value::Number(_), Value::Number(0.0)) => {
                Err(LangError::at("القسمة على صفر غير مسموحة", op))
            }
            (K::Slash, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a / b)),
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
    let tokens = Lexer::new(source).scan()?;
    let statements = Parser::new(tokens).parse()?;
    Interpreter::new().interpret(&statements)
}

#[cfg(test)]
mod tests {
    use super::run;

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
    fn respects_precedence_and_parentheses() {
        assert_eq!(
            run("اطبع(2 + 3 * 4)؛ اطبع((2 + 3) * 4)؛").unwrap(),
            ["14", "20"]
        );
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
        assert!(
            run("اطبع(@)؛")
                .unwrap_err()
                .message
                .contains("محرف غير مدعوم")
        );
        assert!(run("اطبع(\"نص)؛").unwrap_err().message.contains("غير مغلق"));
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
