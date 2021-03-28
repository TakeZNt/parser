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
    let mut index = 0;

    while index < input_bytes.len() {
        match input_bytes[index] {
            // 数値
            b'0'..=b'9' => {
                let tok = lex_number(input_bytes, &mut index);
                tokens.push(tok);
            }
            // 四則演算
            b'+' => match lex_plus(input_bytes, &mut index) {
                Ok(tok) => {
                    tokens.push(tok);
                }
                Err(e) => return Err(e),
            },
            b'-' => match lex_minus(input_bytes, &mut index) {
                Ok(tok) => {
                    tokens.push(tok);
                }
                Err(e) => return Err(e),
            },
            b'*' => match lex_asterisk(input_bytes, &mut index) {
                Ok(tok) => {
                    tokens.push(tok);
                }
                Err(e) => return Err(e),
            },
            b'/' => match lex_slash(input_bytes, &mut index) {
                Ok(tok) => {
                    tokens.push(tok);
                }
                Err(e) => return Err(e),
            },
            // かっこ
            b'(' => match lex_lparen(input_bytes, &mut index) {
                Ok(tok) => {
                    tokens.push(tok);
                }
                Err(e) => return Err(e),
            },
            b')' => match lex_rparen(input_bytes, &mut index) {
                Ok(tok) => {
                    tokens.push(tok);
                }
                Err(e) => return Err(e),
            },
            // 空白文字
            b' ' | b'\n' | b'\t' => {
                skip_spaces(input_bytes, &mut index);
            }
            // 上記以外の文字の場合
            b => {
                return Err(LexError::invalid_char(
                    b as char,
                    Location(index, index + 1),
                ))
            }
        }
    }
    Ok(tokens)
}

/// 数値を解析する
fn lex_number(input: &[u8], index_address: &mut usize) -> Token {
    use std::str::from_utf8;

    let start = *index_address;
    while *index_address < input.len() && b"0123456789".contains(&input[*index_address]) {
        *index_address += 1;
    }

    let n: u64 = from_utf8(&input[start..*index_address])
        // バイト配列から文字列への変換はここでは失敗することはないので無条件にunwrapする
        .unwrap()
        .parse()
        // 文字列から数値への変換もここでは失敗することはないので無条件にunwrapする
        .unwrap();

    Token::number(n, Location(start, *index_address))
}

/// 空白文字（半角スペース、改行、タブ）を無視する
fn skip_spaces(input: &[u8], index_address: &mut usize) {
    while *index_address < input.len() && b" \t\n".contains(&input[*index_address]) {
        *index_address += 1;
    }
}

/// '+'を解析する
fn lex_plus(input: &[u8], index_address: &mut usize) -> Result<Token, LexError> {
    let start = *index_address;
    consume_byte(input, index_address, b'+').map(|()| Token::plus(Location(start, *index_address)))
}

/// '-'を解析する
fn lex_minus(input: &[u8], index_address: &mut usize) -> Result<Token, LexError> {
    let start = *index_address;
    consume_byte(input, index_address, b'-').map(|()| Token::minus(Location(start, *index_address)))
}

/// '*'を解析する
fn lex_asterisk(input: &[u8], index_address: &mut usize) -> Result<Token, LexError> {
    let start = *index_address;
    consume_byte(input, index_address, b'*')
        .map(|()| Token::asterisk(Location(start, *index_address)))
}

/// '/'を解析する
fn lex_slash(input: &[u8], index_address: &mut usize) -> Result<Token, LexError> {
    let start = *index_address;
    consume_byte(input, index_address, b'/').map(|()| Token::slash(Location(start, *index_address)))
}

/// '('を解析する
fn lex_lparen(input: &[u8], index_address: &mut usize) -> Result<Token, LexError> {
    let start = *index_address;
    consume_byte(input, index_address, b'(')
        .map(|()| Token::lparen(Location(start, *index_address)))
}

/// ')'を解析する
fn lex_rparen(input: &[u8], index_address: &mut usize) -> Result<Token, LexError> {
    let start = *index_address;
    consume_byte(input, index_address, b')')
        .map(|()| Token::rparen(Location(start, *index_address)))
}

///
/// 引数に渡されたバイトスライスのposの位置が期待するバイト外の場合、エラーを返す
///
fn consume_byte(input: &[u8], index_address: &mut usize, expected: u8) -> Result<(), LexError> {
    if input.len() <= *index_address {
        return Err(LexError::eof(Location(*index_address, *index_address)));
    }

    if input[*index_address] != expected {
        return Err(LexError::invalid_char(
            input[*index_address] as char,
            Location(*index_address, *index_address + 1),
        ));
    }
    *index_address += 1;
    Ok(())
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
