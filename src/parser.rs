use super::lexer;

/// 単項演算子の種類
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnaryOperatorKind {
    Plus,
    Minus,
}

pub type UnaryOperator = lexer::Annotation<UnaryOperatorKind>;

impl UnaryOperator {
    pub fn plus(location: lexer::Location) -> Self {
        Self::new(UnaryOperatorKind::Plus, location)
    }

    pub fn minus(location: lexer::Location) -> Self {
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

pub type BinaryOperator = lexer::Annotation<BinaryOperatorKind>;

impl BinaryOperator {
    pub fn add(location: lexer::Location) -> Self {
        Self::new(BinaryOperatorKind::Add, location)
    }
    pub fn sub(location: lexer::Location) -> Self {
        Self::new(BinaryOperatorKind::Sub, location)
    }
    pub fn multi(location: lexer::Location) -> Self {
        Self::new(BinaryOperatorKind::Multi, location)
    }
    pub fn div(location: lexer::Location) -> Self {
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

pub type Ast = lexer::Annotation<AstKind>;

impl Ast {
    pub fn num(number: u64, location: lexer::Location) -> Self {
        Self::new(AstKind::Num(number), location)
    }
    pub fn unary(operator: UnaryOperator, operand: Ast, location: lexer::Location) -> Self {
        Self::new(
            AstKind::Unary {
                operator,
                operand: Box::new(operand),
            },
            location,
        )
    }
    pub fn binary(
        operator: BinaryOperator,
        left: Ast,
        right: Ast,
        location: lexer::Location,
    ) -> Self {
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

/// 構文解析のエラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    /// 予期せぬトークンが現れた
    UnexpectedToken(lexer::Token),
    /// 式を期待したが、それ以外のものが現れた
    NotExpression(lexer::Token),
    /// 演算子を期待したが、それ以外のものが現れた
    NotOperator(lexer::Token),
    /// かっこが閉じられていない
    UnclosedOpenParen(lexer::Token),
    /// 式の解析が終わったが、余計なトークンが現れた
    RedundantExpression(lexer::Token),
    /// 解析の途中で入力が終わった
    Eof,
}
