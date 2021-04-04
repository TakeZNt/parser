use super::lexer::*;

use std::error::Error;
use std::fmt;
use std::iter::Iterator;
use std::iter::Peekable;
use std::str::FromStr;

/// 単項演算子の種類
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperatorKind {
    Plus,
    Minus,
}

pub type UnaryOperator = Annotation<UnaryOperatorKind>;

impl UnaryOperator {
    pub fn plus(location: Location) -> Self {
        Self::new(UnaryOperatorKind::Plus, location)
    }

    pub fn minus(location: Location) -> Self {
        Self::new(UnaryOperatorKind::Minus, location)
    }
}

/// 二項演算子の種類
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperatorKind {
    Add,
    Sub,
    Multi,
    Div,
}

pub type BinaryOperator = Annotation<BinaryOperatorKind>;

impl BinaryOperator {
    pub fn add(location: Location) -> Self {
        Self::new(BinaryOperatorKind::Add, location)
    }
    pub fn sub(location: Location) -> Self {
        Self::new(BinaryOperatorKind::Sub, location)
    }
    pub fn multi(location: Location) -> Self {
        Self::new(BinaryOperatorKind::Multi, location)
    }
    pub fn div(location: Location) -> Self {
        Self::new(BinaryOperatorKind::Div, location)
    }
}

/// 抽象構文木の種類
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AstKind {
    Num(u64),
    Unary {
        operator: UnaryOperator,
        operand: Box<Ast>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}

pub type Ast = Annotation<AstKind>;

impl Ast {
    pub fn num(number: u64, location: Location) -> Self {
        Self::new(AstKind::Num(number), location)
    }
    pub fn unary(operator: UnaryOperator, operand: Ast, location: Location) -> Self {
        Self::new(
            AstKind::Unary {
                operator,
                operand: Box::new(operand),
            },
            location,
        )
    }
    pub fn binary(operator: BinaryOperator, left: Ast, right: Ast, location: Location) -> Self {
        Self::new(
            AstKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            location,
        )
    }
}

/// str::parse::<Ast>()を使えるようにする
impl FromStr for Ast {
    type Err = ApplicationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = lex(s)?;
        let ast = parse(tokens)?;
        Ok(ast)
    }
}

/// 構文解析のエラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    /// 予期せぬトークンが現れた
    UnexpectedToken(Token),
    /// 式を期待したが、それ以外のものが現れた
    NotExpression(Token),
    /// 演算子を期待したが、それ以外のものが現れた
    NotOperator(Token),
    /// かっこが閉じられていない
    UnclosedOpenParen(Token),
    /// 式の解析が終わったが、余計なトークンが現れた
    RedundantExpression(Token),
    /// 解析の途中で入力が終わった
    Eof,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            UnexpectedToken(tok) => write!(f, "{}: '{}' is not expected", tok.location, tok.value),
            NotExpression(tok) => write!(
                f,
                "{}: '{}' is not start of expression",
                tok.location, tok.value
            ),
            NotOperator(tok) => write!(f, "{}: '{}' is not an operator", tok.location, tok.value),
            UnclosedOpenParen(tok) => write!(f, "{}: '{}' is not closed", tok.location, tok.value),
            RedundantExpression(tok) => write!(
                f,
                "{}: expression after '{}' is redundant",
                tok.location, tok.value
            ),
            Eof => write!(f, "End of file"),
        }
    }
}

impl Error for ParseError {}

/// エラーを統一的に扱うエラー型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationError {
    Lexer(LexError),
    Parser(ParseError),
}

impl From<LexError> for ApplicationError {
    fn from(e: LexError) -> Self {
        ApplicationError::Lexer(e)
    }
}

impl From<ParseError> for ApplicationError {
    fn from(e: ParseError) -> Self {
        ApplicationError::Parser(e)
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parse error")
    }
}

impl Error for ApplicationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ApplicationError::*;
        match self {
            Lexer(lex_error) => Some(lex_error),
            Parser(parse_error) => Some(parse_error),
        }
    }
}

impl ApplicationError {
    /// エラーの詳細を表示する
    pub fn show_diagnostic(&self, input: &str) {
        let (e, loc): (&Error, Location) = match self {
            ApplicationError::Lexer(e) => (e, e.location.clone()),
            ApplicationError::Parser(e) => {
                let loc = match e {
                    ParseError::UnexpectedToken(Token { location, .. })
                    | ParseError::NotExpression(Token { location, .. })
                    | ParseError::NotOperator(Token { location, .. })
                    | ParseError::UnclosedOpenParen(Token { location, .. }) => location.clone(),
                    // 冗長なトークンがある場合、それ以降のすべてが冗長である
                    ParseError::RedundantExpression(Token { location, .. }) => {
                        Location(location.0, input.len())
                    }
                    ParseError::Eof => Location(input.len(), input.len() + 1),
                };
                (e, loc)
            }
        };
        println!("{}", e);
        print_annote(input, loc);
    }
}

fn print_annote(input: &str, loc: Location) {
    eprintln!("{}", input);
    eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

/// トークンのリストの構文を解析する
pub fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    // LL(1)パーサであるため、Peekableなイテレータを作成する
    let mut tokens_iter = tokens.into_iter().peekable();
    // 式の評価
    let ret = parse_expr(&mut tokens_iter)?;
    // 式の評価の後は何もないはず
    match tokens_iter.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ret),
    }
}

/// EXPR = EXPR3 ;
fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr3(tokens)
}

/// EXPR3 = EXPR3, ("+" | "-"), EXPR2 | EXPR2 ;
fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    fn parse_expr3_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinaryOperator, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens
            .peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Plus => Ok(BinaryOperator::add(tok.location.clone())),
                TokenKind::Minus => Ok(BinaryOperator::sub(tok.location.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }

    parse_left_binop(tokens, parse_expr2, parse_expr3_op)
}

/// EXPR2 = EXPR2, ("*" | "/"), EXPR1 | EXPR1 ;
fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    fn parse_expr2_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinaryOperator, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens
            .peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Asterisk => Ok(BinaryOperator::multi(tok.location.clone())),
                TokenKind::Slash => Ok(BinaryOperator::div(tok.location.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }

    parse_left_binop(tokens, parse_expr1, parse_expr2_op)
}

/// 左結合の二項演算子を解析する
fn parse_left_binop<Tokens>(
    tokens: &mut Peekable<Tokens>,
    subexpr_parser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
    op_parser: fn(&mut Peekable<Tokens>) -> Result<BinaryOperator, ParseError>,
) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut left = subexpr_parser(tokens)?;
    loop {
        match tokens.peek() {
            Some(_) => {
                let op = match op_parser(tokens) {
                    Ok(op) => op,
                    Err(_) => break,
                };
                let right = subexpr_parser(tokens)?;
                let loc = left.location.merge(&right.location);
                left = Ast::binary(op, left, right, loc);
            }
            _ => break,
        }
    }
    Ok(left)
}

/// EXPR1 = ("+" | "-"), ATOM | ATOM ;
fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek().map(|tok| tok.value) {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
            let op = match tokens.next() {
                Some(Token {
                    value: TokenKind::Plus,
                    location, // locationは何でもよい
                }) => UnaryOperator::plus(location),
                Some(Token {
                    value: TokenKind::Minus,
                    location,
                }) => UnaryOperator::minus(location),
                _ => unreachable!(),
            };
            // ATOM
            let atom = parse_atom(tokens)?;
            let loc = op.location.merge(&atom.location);
            Ok(Ast::unary(op, atom, loc))
        }
        // | ATOM
        _ => parse_atom(tokens),
    }
}

/// ATOM = UNUMBER | "(", EXPR3, ")" ;
fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .next()
        .ok_or(ParseError::Eof) // 次が無ければエラー
        .and_then(|tok| match tok.value {
            // UNUMBER
            TokenKind::Number(n) => Ok(Ast::num(n, tok.location)),
            // "(" EXPR3 ")"
            TokenKind::LParen => {
                let exp = parse_expr(tokens)?;
                match tokens.next() {
                    // ")"の場合
                    Some(Token {
                        value: TokenKind::RParen,
                        .. // 他のフィールドは何でもよい
                    }) => Ok(exp),
                    // ")"以外の何かの場合
                    Some(t) => Err(ParseError::RedundantExpression(t)),
                    // 次のトークンがない場合
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        // 1 + 2 * 3 - -10
        let ast = parse(vec![
            Token::number(1, Location(0, 1)),
            Token::plus(Location(2, 3)),
            Token::number(2, Location(4, 5)),
            Token::asterisk(Location(6, 7)),
            Token::number(3, Location(8, 9)),
            Token::minus(Location(10, 11)),
            Token::minus(Location(12, 13)),
            Token::number(10, Location(13, 15)),
        ]);
        assert_eq!(
            ast,
            Ok(Ast::binary(
                BinaryOperator::sub(Location(10, 11)),
                Ast::binary(
                    BinaryOperator::add(Location(2, 3)),
                    Ast::num(1, Location(0, 1)),
                    Ast::binary(
                        BinaryOperator::new(BinaryOperatorKind::Multi, Location(6, 7)),
                        Ast::num(2, Location(4, 5)),
                        Ast::num(3, Location(8, 9)),
                        Location(4, 9)
                    ),
                    Location(0, 9),
                ),
                Ast::unary(
                    UnaryOperator::minus(Location(12, 13)),
                    Ast::num(10, Location(13, 15)),
                    Location(12, 15)
                ),
                Location(0, 15)
            ))
        )
    }

    #[test]
    fn test_parse_atom_num() {
        let tokens = vec![Token::number(1, Location(0, 1))];
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(parse_atom(&mut iter), Ok(Ast::num(1, Location(0, 1))));
    }
}
