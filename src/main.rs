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

fn main() {
    println!("hello parser!");
}
