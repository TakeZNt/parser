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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

///
/// 字句解析器
///
fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    // バイト配列のスライスへ入力を変換
    let input_bytes = input.as_bytes();
    // バイトスライスの位置
    let mut pos = 0;

    while pos < input_bytes.len() {
        match input_bytes[pos] {
            // 数値
            b'0'..=b'9' => {
                let (tok, p) = lex_number(input_bytes, pos);
                tokens.push(tok);
                pos = p;
            }
            // 四則演算
            b'+' => match lex_plus(input_bytes, pos) {
                Ok((tok, p)) => {
                    tokens.push(tok);
                    pos = p;
                }
                Err(e) => return Err(e),
            },
            b'-' => match lex_minus(input_bytes, pos) {
                Ok((tok, p)) => {
                    tokens.push(tok);
                    pos = p;
                }
                Err(e) => return Err(e),
            },
            b'*' => match lex_asterisk(input_bytes, pos) {
                Ok((tok, p)) => {
                    tokens.push(tok);
                    pos = p;
                }
                Err(e) => return Err(e),
            },
            b'/' => match lex_slash(input_bytes, pos) {
                Ok((tok, p)) => {
                    tokens.push(tok);
                    pos = p;
                }
                Err(e) => return Err(e),
            },
            // かっこ
            b'(' => match lex_lparen(input_bytes, pos) {
                Ok((tok, p)) => {
                    tokens.push(tok);
                    pos = p;
                }
                Err(e) => return Err(e),
            },
            b')' => match lex_rparen(input_bytes, pos) {
                Ok((tok, p)) => {
                    tokens.push(tok);
                    pos = p;
                }
                Err(e) => return Err(e),
            },
            // 空白文字
            b' ' | b'\n' | b'\t' => {
                let ((), p) = skip_spaces(input_bytes, pos);
                pos = p;
            }
            // 上記以外の文字の場合
            b => return Err(LexError::invalid_char(b as char, Location(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

/// 数値を解析する
fn lex_number(input: &[u8], start: usize) -> (Token, usize) {
    use std::str::from_utf8;

    let mut pos = start;
    while pos < input.len() && b"0123456789".contains(&input[pos]) {
        pos += 1;
    }

    let n: u64 = from_utf8(&input[start..pos])
        // バイト配列から文字列への変換はここでは失敗することはないので無条件にunwrapする
        .unwrap()
        .parse()
        // 文字列から数値への変換もここでは失敗することはないので無条件にunwrapする
        .unwrap();

    (Token::number(n, Location(start, pos)), pos)
}

/// 空白文字（半角スペース、改行、タブ）を無視する
fn skip_spaces(input: &[u8], start: usize) -> ((), usize) {
    let mut pos = start;
    while pos < input.len() && b" \t\n".contains(&input[pos]) {
        pos += 1;
    }
    // 空白は無視する
    ((), pos)
}

/// '+'を解析する
fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Location(start, end)), end))
}

/// '-'を解析する
fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Location(start, end)), end))
}

/// '*'を解析する
fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Location(start, end)), end))
}

/// '/'を解析する
fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Location(start, end)), end))
}

/// '('を解析する
fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Location(start, end)), end))
}

/// ')'を解析する
fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Location(start, end)), end))
}

///
/// 引数に渡されたバイトスライスのposの位置が期待するバイトの場合、posに1を加算して返す。
/// それ以外の場合、エラーを返す
///
fn consume_byte(input: &[u8], pos: usize, expected: u8) -> Result<(u8, usize), LexError> {
    if input.len() <= pos {
        return Err(LexError::eof(Location(pos, pos)));
    }

    if input[pos] != expected {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Location(pos, pos + 1),
        ));
    }

    Ok((expected, pos + 1))
}

#[test]
fn test_lexer() {
    assert_eq!(
        lex("1 + 2 * 3 - -10"),
        Ok(vec![
            Token::number(1, Location(0, 1)),
            Token::plus(Location(2, 3)),
            Token::number(2, Location(4, 5)),
            Token::asterisk(Location(6, 7)),
            Token::number(3, Location(8, 9)),
            Token::minus(Location(10, 11)),
            Token::minus(Location(12, 13)),
            Token::number(10, Location(13, 15)),
        ])
    )
}

fn main() {
    let result = lex("1 + 2 * 3 - -10");
}
