///
/// 入力文字の何文字目から何文字目までかを表す構造体。ただし、数値は0始まり。
/// 例えばLocation(5, 8)は6文字目から9文字目までを表す。
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location(usize, usize);

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
pub struct Annotation<T> {
    value: T,
    location: Location,
}

impl<T> Annotation<T> {
    ///
    /// アノテーションを作成する
    ///
    pub fn new(value: T, location: Location) -> Self {
        Self { value, location }
    }
}

///
/// トークンの種類
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
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
pub type Token = Annotation<TokenKind>;

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
pub enum LexErrorKind {
    /// 無効な文字
    InvalidChar(char),
    /// 文字列の終わり
    Eof,
}

/// LexErrorKindを持つアノテーションをLexErrorとして定義する
pub type LexError = Annotation<LexErrorKind>;

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
pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();

    // バイト配列のスライスへ入力を変換
    let input_bytes = input.as_bytes();
    // バイトスライスの位置
    let mut index = 0;

    while index < input_bytes.len() {
        match input_bytes[index] {
            // 四則演算
            b'+' => lex_one_byte(input_bytes, &mut index, b'+', &mut tokens)?,
            b'-' => lex_one_byte(input_bytes, &mut index, b'-', &mut tokens)?,
            b'*' => lex_one_byte(input_bytes, &mut index, b'*', &mut tokens)?,
            b'/' => lex_one_byte(input_bytes, &mut index, b'/', &mut tokens)?,
            // かっこ
            b'(' => lex_one_byte(input_bytes, &mut index, b'(', &mut tokens)?,
            b')' => lex_one_byte(input_bytes, &mut index, b')', &mut tokens)?,
            // 上記以外の文字の場合
            b => {
                if is_number(b) {
                    // 数値
                    lex_number(input_bytes, &mut index, &mut tokens);
                } else if is_space(b) {
                    // 空白文字
                    skip_spaces(input_bytes, &mut index);
                } else {
                    return Err(LexError::invalid_char(
                        b as char,
                        Location(index, index + 1),
                    ));
                }
            }
        }
    }
    Ok(tokens)
}

/// 数値を解析する
fn lex_number(input: &[u8], index_address: &mut usize, tokens: &mut Vec<Token>) {
    use std::str::from_utf8;

    let start = *index_address;
    while *index_address < input.len() && is_number(input[*index_address]) {
        *index_address += 1;
    }

    // 数値の文字列を実際の数値へ変換する
    let numbber: u64 = from_utf8(&input[start..*index_address])
        // バイト配列から文字列への変換はここでは失敗することはないので無条件にunwrapする
        .unwrap()
        .parse()
        // 文字列から数値への変換もここでは失敗することはないので無条件にunwrapする
        .unwrap();

    tokens.push(Token::number(numbber, Location(start, *index_address)));
}

fn is_number(byte: u8) -> bool {
    b'0' <= byte && byte <= b'9'
}

/// 空白文字（半角スペース、改行、タブ）を無視する
fn skip_spaces(input: &[u8], index_address: &mut usize) {
    while *index_address < input.len() && is_space(input[*index_address]) {
        *index_address += 1;
    }
}

fn is_space(byte: u8) -> bool {
    byte == b' ' || byte == b'\t' || byte == b'\n'
}

/// 1文字のトークンを解析する
fn lex_one_byte(
    input: &[u8],
    index_address: &mut usize,
    byte: u8,
    tokens: &mut Vec<Token>,
) -> Result<(), LexError> {
    let start = *index_address;
    consume_byte(input, index_address, byte)?;
    tokens.push(create_one_byte_token(byte, start, *index_address));
    Ok(())
}

fn create_one_byte_token(byte: u8, start_index: usize, end_index: usize) -> Token {
    match byte {
        b'+' => Token::plus(Location(start_index, end_index)),
        b'-' => Token::minus(Location(start_index, end_index)),
        b'*' => Token::asterisk(Location(start_index, end_index)),
        b'/' => Token::slash(Location(start_index, end_index)),
        b'(' => Token::lparen(Location(start_index, end_index)),
        b')' => Token::rparen(Location(start_index, end_index)),
        b => panic!("unexpected byte : {}", b),
    }
}

///
/// 引数に渡されたバイトスライスのposの位置が期待するバイト外の場合、エラーを返す。
/// それ以外の場合、インデックスを1進める。
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

#[cfg(test)]
mod test {
    use super::*;
    
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
}
