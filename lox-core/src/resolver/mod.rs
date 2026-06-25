use crate::data::{HashMap, Symbol};
use crate::expr::{
    Assign, Binary, Call, Expr, ExprElement, ExprVisitor, Get, Grouping, Literal, Logical, Set,
    Super, This, Unary, Variable,
};
use crate::interpreter::INIT_METHOD;
use crate::stmt::{
    Block, Class, Expression, Function, If, Print, Return, Stmt, StmtElement, StmtVisitor, Var,
    While,
};
use crate::token::Token;
use miette::SourceSpan;
use rustc_hash::FxBuildHasher;
use std::fmt::Display;
use std::{fmt, mem};

pub trait Resolve {
    fn resolve(self) -> Result<(Vec<Stmt>, ResolutionMap), Vec<ResolverError>>;
}

impl<T> Resolve for T
where
    T: IntoIterator<Item = Stmt>,
{
    fn resolve(self) -> Result<(Vec<Stmt>, ResolutionMap), Vec<ResolverError>> {
        let statements = self.into_iter().collect::<Vec<_>>();
        let resolution_map = Resolver::default().resolve(&statements)?;
        Ok((statements, resolution_map))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ResolutionMap {
    distances: HashMap<Token, usize>,
}

impl ResolutionMap {
    pub const fn new() -> Self {
        Self {
            distances: HashMap::with_hasher(FxBuildHasher),
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
pub enum ResolverErrorCode {
    ReadLocalVariableInInitializer,
    RedeclaredVariableInSameScope,
    ReturnFromOutsideFunction,
    ReturnValueFromInitializer,
    ThisUsedOutsideOfClass,
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
            ResolverErrorCode::ReadLocalVariableInInitializer => {
                write!(f, "can not read local variable in its own initializer")
            },
            ResolverErrorCode::RedeclaredVariableInSameScope => {
                write!(
                    f,
                    "variable with name '{}' already declared in this scope",
                    self.token.lexeme()
                )
            },
            ResolverErrorCode::ReturnFromOutsideFunction => {
                write!(f, "can not return from outside of any function")
            },
            ResolverErrorCode::ReturnValueFromInitializer => {
                write!(f, "can not return a value from an initializer")
            },
            ResolverErrorCode::ThisUsedOutsideOfClass => {
                write!(f, "'this' used outside of a class")
            },
        }
    }
}

impl From<ResolverError> for Vec<ResolverError> {
    fn from(error: ResolverError) -> Self {
        vec![error]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VarState {
    Declared,
    Initialized,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionKind {
    #[default]
    None,
    Function,
    Initializer,
    Method,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
enum ClassKind {
    #[default]
    None,
    Class,
}

#[derive(Default)]
pub struct Resolver {
    scopes: Vec<HashMap<Symbol, VarState>>,
    resolution_map: ResolutionMap,
    current_function: FunctionKind,
    current_class: ClassKind,
}

impl Resolver {
    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<ResolutionMap, Vec<ResolverError>> {
        self.scopes.clear();
        self.resolve_stmts(statements)?;
        Ok(mem::take(&mut self.resolution_map))
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::with_hasher(FxBuildHasher));
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Token) -> Result<(), Vec<ResolverError>> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme()) {
                return Err(vec![ResolverError {
                    code: ResolverErrorCode::RedeclaredVariableInSameScope,
                    token: name,
                    location: name.location,
                }]);
            }
            scope.insert(name.lexeme(), VarState::Declared);
        }
        Ok(())
    }

    fn define(&mut self, name: Token) {
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

    fn resolve_function(
        &mut self,
        function: &Function,
        kind: FunctionKind,
    ) -> Result<(), Vec<ResolverError>> {
        let enclosing_function = mem::replace(&mut self.current_function, kind);
        self.begin_scope();
        for param in function.parameters() {
            self.declare(*param)?;
            self.define(*param);
        }
        self.resolve_stmts(function.body())?;
        self.end_scope();
        self.current_function = enclosing_function;
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

    fn visit_get_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Get) -> Self::Output {
        self.resolve_expr(expr.object())
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

    fn visit_set_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &Set) -> Self::Output {
        self.resolve_expr(expr.value())?;
        self.resolve_expr(expr.object())?;
        Ok(())
    }

    fn visit_super_expr(&mut self, _rtc: &mut Self::Context<'_>, _expr: &Super) -> Self::Output {
        todo!()
    }

    fn visit_this_expr(&mut self, _rtc: &mut Self::Context<'_>, expr: &This) -> Self::Output {
        match self.current_class {
            ClassKind::None => Err(vec![ResolverError {
                code: ResolverErrorCode::ThisUsedOutsideOfClass,
                token: expr.keyword(),
                location: expr.keyword().location,
            }]),
            ClassKind::Class => {
                self.resolve_local(expr.keyword());
                Ok(())
            },
        }
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
                code: ResolverErrorCode::ReadLocalVariableInInitializer,
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

    fn visit_class_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &Class) -> Self::Output {
        let enclosing_class = mem::replace(&mut self.current_class, ClassKind::Class);
        self.declare(stmt.name())?;
        self.define(stmt.name());
        self.begin_scope();
        self.scopes
            .last_mut()
            .expect("scope should be present, as we just pushed it with Self::begin_scope")
            .insert(Symbol::from("this"), VarState::Initialized);
        for method in stmt.methods() {
            let declaration = if method.name().lexeme.as_str() == INIT_METHOD {
                FunctionKind::Initializer
            } else {
                FunctionKind::Method
            };
            self.resolve_function(method, declaration)?;
        }
        self.end_scope();
        self.current_class = enclosing_class;
        Ok(())
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
        self.declare(stmt.name())?;
        self.define(stmt.name());
        self.resolve_function(stmt, FunctionKind::Function)
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
        match self.current_function {
            FunctionKind::None => {
                return Err(vec![ResolverError {
                    code: ResolverErrorCode::ReturnFromOutsideFunction,
                    token: stmt.keyword(),
                    location: stmt.keyword().location,
                }]);
            },
            FunctionKind::Initializer if stmt.value().is_some() => {
                return Err(vec![ResolverError {
                    code: ResolverErrorCode::ReturnValueFromInitializer,
                    token: stmt.keyword(),
                    location: stmt.keyword().location,
                }]);
            },
            _ => {},
        }
        if let Some(value) = stmt.value() {
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, _rtc: &mut Self::Context<'_>, stmt: &Var) -> Self::Output {
        self.declare(stmt.name())?;
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
