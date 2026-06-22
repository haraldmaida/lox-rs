use crate::data::Symbol;
use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use crate::program::Program;
use crate::stmt::{
    Block, Class, Expression, Function, If, Print, Return, Stmt, StmtElement, StmtVisitor, Var,
    While,
};
use crate::token::Token;
use hashbrown::HashMap;
use miette::SourceSpan;
use std::fmt::Display;
use std::{fmt, mem};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ResolutionMap {
    distances: HashMap<Token, usize>,
}

impl ResolutionMap {
    pub fn new() -> Self {
        Self {
            distances: HashMap::new(),
        }
    }

    pub fn get_distance(&self, token: Token) -> Option<usize> {
        self.distances.get(&token).copied()
    }

    pub fn insert_distance(&mut self, token: Token, distance: usize) {
        self.distances.insert(token, distance);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarState {
    Declared,
    Initialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverErrorCode {
    CannotReadLocalVariableInInitializer,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub struct ResolverError {
    code: ResolverErrorCode,
    token: Token,
    location: SourceSpan,
}

impl Display for ResolverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code {
            ResolverErrorCode::CannotReadLocalVariableInInitializer => {
                write!(f, "can't read local variable in its own initializer")
            },
        }
    }
}

impl From<ResolverError> for Vec<ResolverError> {
    fn from(error: ResolverError) -> Self {
        vec![error]
    }
}

#[derive(Default)]
pub struct Resolver {
    scopes: Vec<HashMap<Symbol, VarState>>,
    resolution_map: ResolutionMap,
}

impl Resolver {
    pub fn resolve(
        &mut self,
        program: impl Into<Program>,
    ) -> Result<ResolutionMap, Vec<ResolverError>> {
        self.scopes.clear();
        let program = program.into();
        self.resolve_stmts(program.statements())?;
        Ok(mem::take(&mut self.resolution_map))
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) {
        debug_assert!(
            !self.scopes.is_empty(),
            "can't declare a symbol outside of a scope"
        );
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme(), VarState::Declared);
        }
    }

    fn define(&mut self, name: Token) {
        debug_assert!(
            !self.scopes.is_empty(),
            "can't define a symbol outside of a scope"
        );
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme(), VarState::Initialized);
        }
    }

    fn resolve_local(&mut self, name: Token) {
        for (distance, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name.lexeme()) {
                self.resolution_map.insert_distance(name, distance);
                return;
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Vec<ResolverError>> {
        expr.accept(self, &mut ())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), Vec<ResolverError>> {
        stmt.accept(self, &mut ())
    }

    fn resolve_stmts(&mut self, stmts: &[Stmt]) -> Result<(), Vec<ResolverError>> {
        let mut errors = Vec::new();
        for stmt in stmts {
            if let Err(error) = self.resolve_stmt(stmt) {
                errors.extend(error);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn resolve_function(&mut self, function: &Function) -> Result<(), Vec<ResolverError>> {
        self.begin_scope();
        for param in function.parameters() {
            self.define(*param);
        }
        self.resolve_stmts(function.body())?;
        self.end_scope();
        Ok(())
    }
}

impl ExprVisitor for Resolver {
    type Context<'c> = ();
    type Output = Result<(), Vec<ResolverError>>;

    fn visit_assign_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Assign) -> Self::Output {
        self.resolve_expr(expr.value())?;
        self.resolve_local(expr.name());
        Ok(())
    }

    fn visit_binary_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Binary) -> Self::Output {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())?;
        Ok(())
    }

    fn visit_call_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Call) -> Self::Output {
        self.resolve_expr(expr.callee())?;
        for arg in expr.arguments() {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_get_expr(&mut self, _rtc: &mut Self::Context<'_>, _expr: &Get) -> Self::Output {
        todo!()
    }

    fn visit_grouping_expr(
        &mut self,
        _rtc: &mut Self::Context<'_>,
        expr: &Grouping,
    ) -> Self::Output {
        self.resolve_expr(expr.expression())
    }

    fn visit_literal_expr(
        &mut self,
        _rtc: &mut Self::Context<'_>,
        _expr: &Literal,
    ) -> Self::Output {
        Ok(())
    }

    fn visit_logical_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Logical) -> Self::Output {
        self.resolve_expr(expr.left())?;
        self.resolve_expr(expr.right())?;
        Ok(())
    }

    fn visit_set_expr(&mut self, _rtc: &mut Self::Context<'_>, _expr: &Set) -> Self::Output {
        todo!()
    }

    fn visit_super_expr(&mut self, _rtc: &mut Self::Context<'_>, _expr: &Super) -> Self::Output {
        todo!()
    }

    fn visit_this_expr(&mut self, _rtc: &mut Self::Context<'_>, _expr: &This) -> Self::Output {
        todo!()
    }

    fn visit_unary_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Unary) -> Self::Output {
        self.resolve_expr(expr.right())
    }

    fn visit_variable_expr(
        &mut self,
        _rtc: &mut Self::Context<'_>,
        expr: &Variable,
    ) -> Self::Output {
        if let Some(scope) = self.scopes.last_mut()
            && scope.get(&expr.name().lexeme) == Some(&VarState::Declared)
        {
            return Err(ResolverError {
                code: ResolverErrorCode::CannotReadLocalVariableInInitializer,
                token: expr.name(),
                location: expr.name().location,
            }
            .into());
        }
        self.resolve_local(expr.name());
        Ok(())
    }
}

impl StmtVisitor for Resolver {
    type Context<'c> = ();
    type Output = Result<(), Vec<ResolverError>>;

    fn visit_block_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &Block) -> Self::Output {
        self.begin_scope();
        if let Err(error) = self.resolve_stmts(stmt.statements()) {
            self.end_scope();
            return Err(error);
        }
        self.end_scope();
        Ok(())
    }

    fn visit_class_stmt(&mut self, _rtc: &mut Self::Context<'_>, _stmt: &Class) -> Self::Output {
        todo!()
    }

    fn visit_expression_stmt(
        &mut self,
        _rtc: &mut Self::Context<'_>,
        stmt: &Expression,
    ) -> Self::Output {
        self.resolve_expr(stmt.expression())
    }

    fn visit_function_stmt(
        &mut self,
        _rtc: &mut Self::Context<'_>,
        stmt: &Function,
    ) -> Self::Output {
        self.define(stmt.name());
        self.resolve_function(stmt)
    }

    fn visit_if_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &If) -> Self::Output {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.then_branch())?;
        if let Some(else_branch) = stmt.else_branch() {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &Print) -> Self::Output {
        self.resolve_expr(stmt.expression())
    }

    fn visit_return_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &Return) -> Self::Output {
        if let Some(value) = stmt.value() {
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &Var) -> Self::Output {
        self.declare(stmt.name());
        if let Some(initializer) = stmt.initializer() {
            self.resolve_expr(initializer)?;
        }
        self.define(stmt.name());
        Ok(())
    }

    fn visit_while_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &While) -> Self::Output {
        self.resolve_expr(stmt.condition())?;
        self.resolve_stmt(stmt.body())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
