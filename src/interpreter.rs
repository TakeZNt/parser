use std::error::Error;
use std::fmt;

use super::lexer::*;
use super::parser::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpreterErrorKind {
    DivisionByZero,
}

pub type InterpreterError = Annotation<InterpreterErrorKind>;

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => write!(f, "ゼロで除算できません"),
        }
    }
}

impl Error for InterpreterError {
    fn description(&self) -> &str {
        use self::InterpreterErrorKind::*;
        match self.value {
            DivisionByZero => "the right hand expression of the division evaluates to zero",
        }
    }
}

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        // エラー情報を簡単に表示し
        eprintln!("{}", self);
        // エラー位置を指示する
        print_annote(input, self.location.clone());
    }
}

/// 評価器を表すデータ型
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: &Ast) -> Result<i64, InterpreterError> {
        use self::AstKind::*;
        match expr.value {
            Num(n) => Ok(n as i64),
            Unary {
                ref operator, // match式は値を可能な限り所有しようとする。それでは都合が悪い場合、"ref" で参照する。
                ref operand,
            } => {
                let operand = self.eval(operand)?;
                Ok(self.eval_uniop(operator, operand))
            }
            Binary {
                ref operator,
                ref left,
                ref right,
            } => {
                let left = self.eval(left)?;
                let right = self.eval(right)?;
                self.eval_binop(operator, left, right)
                    .map_err(|e| InterpreterError::new(e, expr.location.clone()))
            }
        }
    }

    fn eval_uniop(&mut self, operator: &UnaryOperator, operand: i64) -> i64 {
        use super::parser::UnaryOperatorKind::*;
        match operator.value {
            Plus => operand,
            Minus => -operand,
        }
    }

    fn eval_binop(
        &mut self,
        operator: &BinaryOperator,
        left: i64,
        right: i64,
    ) -> Result<i64, InterpreterErrorKind> {
        use super::parser::BinaryOperatorKind::*;
        match operator.value {
            Add => Ok(left + right),
            Sub => Ok(left - right),
            Multi => Ok(left * right),
            Div => {
                if right == 0 {
                    Err(InterpreterErrorKind::DivisionByZero)
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}
