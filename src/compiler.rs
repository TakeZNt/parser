use super::parser::*;

/// 逆ポーランド記法へのコンパイラ
pub struct RpnCompiler;

impl RpnCompiler {
    pub fn new() -> Self {
        RpnCompiler
    }

    ///
    /// 抽象構文木を解析し、逆ポーランド記法の文字列へ変換して返す
    ///
    pub fn compile(&mut self, expr: &Ast) -> String {
        let mut buf = String::new();
        self.compile_inner(expr, &mut buf);
        buf
    }

    fn compile_inner(&mut self, expr: &Ast, buf: &mut String) {
        use super::parser::AstKind::*;
        match expr.value {
            Num(n) => buf.push_str(&n.to_string()),
            Unary {
                ref operator,
                ref operand,
            } => {
                self.compile_uniop(operator, buf);
                self.compile_inner(operand, buf);
            }
            Binary {
                ref operator,
                ref left,
                ref right,
            } => {
                self.compile_inner(left, buf);
                buf.push_str(" ");
                self.compile_inner(right, buf);
                buf.push_str(" ");
                self.compile_binop(operator, buf);
            }
        }
    }

    /// 単項演算子を処理する
    fn compile_uniop(&mut self, operator: &UnaryOperator, buf: &mut String) {
        use super::parser::UnaryOperatorKind::*;
        match operator.value {
            Plus => buf.push_str("+"),
            Minus => buf.push_str("-"),
        }
    }

    /// 二項演算子を処理する
    fn compile_binop(&mut self, operator: &BinaryOperator, buf: &mut String) {
        use super::parser::BinaryOperatorKind::*;
        match operator.value {
            Add => buf.push_str("+"),
            Sub => buf.push_str("-"),
            Multi => buf.push_str("*"),
            Div => buf.push_str("/"),
        }
    }
}
