///
/// 入力文字の何文字目から何文字目までかを表す構造体。ただし、数値は0始まり。
/// 例えばLocation(5, 8)は6文字目から9文字目までを表す。
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location(usize, usize);

impl Location {
    ///
    ///　位置情報をマージする
    ///
    fn merge(&self, other: Location) -> Location {
        use std::cmp::{max, min};
        Location(min(self.0, other.0), max(self.1, other.1))
    }
}

///
/// トークンの種類などの値と位置情報を持つアノテーション。
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annotation<T> {
    value: T,
    location: Location,
}

impl<T> Annotation<T> {
    ///
    /// アノテーションを作成する
    ///
    fn new(value: T, location: Location) -> Self {
        Self { value, location }
    }
}

///
/// トークンの種類
///
enum TokenKind {
    /// [0-9][0-9]*
    Number(u64),
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LParen,
    /// )
    RParen,
}

/// TokenKindを持つアノテーションをTokenとして定義する
type Token = Annotation<TokenKind>;

/// ファクトリメソッドをトークン種類ごとに用意する
impl Token {
    fn number(n: u64, location: Location) -> Self {
        Self::new(TokenKind::Number(n), location)
    }
    fn plus(location: Location) -> Self {
        Self::new(TokenKind::Plus, location)
    }
    fn minus(location: Location) -> Self {
        Self::new(TokenKind::Minus, location)
    }
    fn asterisk(location: Location) -> Self {
        Self::new(TokenKind::Asterisk, location)
    }
    fn slash(location: Location) -> Self {
        Self::new(TokenKind::Slash, location)
    }
    fn lparen(location: Location) -> Self {
        Self::new(TokenKind::LParen, location)
    }
    fn rparen(location: Location) -> Self {
        Self::new(TokenKind::RParen, location)
    }
}

///
/// 字句解析エラーの種類
///
enum LexErrorKind {
    /// 無効な文字
    InvalidChar(char),
    /// 文字列の終わり
    Eof,
}

/// LexErrorKindを持つアノテーションをLexErrorとして定義する
type LexError = Annotation<LexErrorKind>;

/// ファクトリメソッドを字句解析エラー種類ごとに用意する
impl LexError {
    fn invalid_char(c: char, location: Location) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), location)
    }
    fn eof(location: Location) -> Self {
        Self::new(LexErrorKind::Eof, location)
    }
}

fn main() {
    println!("hello parser!");
}
