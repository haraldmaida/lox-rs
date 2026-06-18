use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use miette::Diagnostic;
use std::fmt;

pub type Error = AstPrinterError;

#[derive(thiserror::Error, Diagnostic, Debug)]
#[error(transparent)]
pub struct AstPrinterError(#[from] fmt::Error);

pub struct AstPrinter<'a, W> {
    out: &'a mut W,
}

impl<W> AstPrinter<'_, W>
where
    W: fmt::Write,
{
    pub fn print(expr: &Expr, out: &mut W) -> Result<(), Error> {
        let mut printer = AstPrinter { out };
        expr.accept(&mut printer)
    }

    fn write_bool(&mut self, value: bool) -> Result<(), Error> {
        write!(self.out, "{value}").map_err(From::from)
    }

    fn write_f64(&mut self, value: f64) -> Result<(), Error> {
        write!(self.out, "{value}").map_err(From::from)
    }

    fn write_str(&mut self, value: &str) -> Result<(), Error> {
        write!(self.out, "{value}").map_err(From::from)
    }

    fn write_grouped(&mut self, name: &str, expressions: &[&Expr]) -> Result<(), Error> {
        write!(self.out, "({name}")?;
        for expr in expressions {
            write!(self.out, " ")?;
            expr.accept(self)?;
        }
        write!(self.out, ")")?;
        Ok(())
    }
}

impl<W> ExprVisitor for AstPrinter<'_, W>
where
    W: fmt::Write,
{
    type Output = Result<(), Error>;

    fn visit_assign_expr(&mut self, _expr: &Assign) -> Self::Output {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &Binary) -> Self::Output {
        self.write_grouped(expr.operator().lexeme(), &[expr.left(), expr.right()])
    }

    fn visit_call_expr(&mut self, _expr: &Call) -> Self::Output {
        todo!()
    }

    fn visit_get_expr(&mut self, _expr: &Get) -> Self::Output {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Self::Output {
        self.write_grouped("group", &[expr.expression()])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Self::Output {
        match expr {
            Literal::Nil => self.write_str("nil"),
            Literal::Bool(value) => self.write_bool(*value),
            Literal::Number(value) => self.write_f64(*value),
            Literal::String(value) => self.write_str(value.as_str()),
        }
    }

    fn visit_logical_expr(&mut self, _expr: &Logical) -> Self::Output {
        todo!()
    }

    fn visit_set_expr(&mut self, _expr: &Set) -> Self::Output {
        todo!()
    }

    fn visit_super_expr(&mut self, _expr: &Super) -> Self::Output {
        todo!()
    }

    fn visit_this_expr(&mut self, _expr: &This) -> Self::Output {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Self::Output {
        self.write_grouped(expr.operator().lexeme(), &[expr.right()])
    }

    fn visit_variable_expr(&mut self, _expr: &Variable) -> Self::Output {
        todo!()
    }
}
