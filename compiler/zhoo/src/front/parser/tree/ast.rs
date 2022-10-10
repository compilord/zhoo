use crate::front::parser::tree::pbox::PBox;
use crate::front::parser::tree::ty::Ty;
use crate::util::span::{Span, Spanned};

#[derive(Clone, Debug)]
pub enum Public {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Async {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Unsafe {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Wasm {
  Yes(Span),
  No,
}

#[derive(Clone, Debug)]
pub enum Mutability {
  Yes,
  No,
}

#[derive(Clone, Debug)]
pub struct Pattern {
  pub kind: PatternKind,
  pub span: Span,
}

impl Pattern {
  pub const fn new(kind: PatternKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum PatternKind {
  Underscore,
  Identifier(PBox<Expr>),
  Lit(PBox<Expr>),
}

#[derive(Clone, Debug)]
pub struct Program {
  pub stmts: Vec<PBox<Stmt>>,
}

impl Program {
  pub const fn new(stmts: Vec<PBox<Stmt>>) -> Self {
    Self { stmts }
  }
}

#[derive(Clone, Debug)]
pub struct Stmt {
  pub kind: StmtKind,
  pub span: Span,
}

impl Stmt {
  pub const fn new(kind: StmtKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum StmtKind {
  Ext(PBox<Ext>),
  TyAlias(PBox<TyAlias>),
  Enum(PBox<Enum>),
  Struct(PBox<Struct>),
  Val(PBox<Decl>),
  Fun(PBox<Fun>),
  Unit(PBox<Unit>),
}

#[derive(Clone, Debug)]
pub struct Ext {
  pub public: Public,
  pub prototype: Prototype,
  pub body: Option<PBox<Block>>,
  pub span: Span,
}

impl Ext {
  pub const fn new(
    public: Public,
    prototype: Prototype,
    body: Option<PBox<Block>>,
    span: Span,
  ) -> Self {
    Self {
      public,
      prototype,
      body,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct TyAlias {
  pub public: Public,
  pub name: PBox<Expr>,
  pub kind: TyAliasKind,
  pub span: Span,
}

impl TyAlias {
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    kind: TyAliasKind,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      kind,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum TyAliasKind {
  Single(PBox<Ty>),
  Group(Vec<PBox<TyAliasField>>),
}

#[derive(Clone, Debug)]
pub struct TyAliasField {
  pub name: PBox<Expr>,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl TyAliasField {
  pub const fn new(name: PBox<Expr>, ty: PBox<Ty>, span: Span) -> Self {
    Self { name, ty, span }
  }
}

#[derive(Clone, Debug)]
pub struct Enum {
  pub public: Public,
  pub name: PBox<Expr>,
  pub variants: Vec<PBox<EnumVariant>>,
  pub span: Span,
}

impl Enum {
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    variants: Vec<PBox<EnumVariant>>,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      variants,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
  pub name: PBox<Expr>,
  pub arg: Option<PBox<EnumVariantArg>>,
  pub span: Span,
}

impl EnumVariant {
  pub const fn new(
    name: PBox<Expr>,
    arg: Option<PBox<EnumVariantArg>>,
    span: Span,
  ) -> Self {
    Self { name, arg, span }
  }
}

#[derive(Clone, Debug)]
pub struct EnumVariantArg {
  pub value: PBox<Expr>,
  pub span: Span,
}

impl EnumVariantArg {
  pub const fn new(value: PBox<Expr>, span: Span) -> Self {
    Self { value, span }
  }
}

#[derive(Clone, Debug)]
pub struct Struct {
  pub public: Public,
  pub name: PBox<Expr>,
  pub kind: StructKind,
  pub span: Span,
}

impl Struct {
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    kind: StructKind,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      kind,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum StructKind {
  Init,
  Decl(Vec<PBox<StructDeclField>>),
  Tuple(Vec<PBox<StructTupleField>>),
}

#[derive(Clone, Debug)]
pub struct StructDeclField {
  pub public: Public,
  pub name: PBox<Expr>,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl StructDeclField {
  pub const fn new(
    public: Public,
    name: PBox<Expr>,
    ty: PBox<Ty>,
    span: Span,
  ) -> Self {
    Self {
      public,
      name,
      ty,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct StructTupleField {
  pub public: Public,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl StructTupleField {
  pub const fn new(public: Public, ty: PBox<Ty>, span: Span) -> Self {
    Self { public, ty, span }
  }
}

#[derive(Clone, Debug)]
pub struct Decl {
  pub mutability: Mutability,
  pub kind: DeclKind,
  pub pattern: Pattern,
  pub ty: Option<PBox<Ty>>,
  pub value: PBox<Expr>,
  pub span: Span,
}

impl Decl {
  pub const fn new(
    mutability: Mutability,
    kind: DeclKind,
    pattern: Pattern,
    ty: Option<PBox<Ty>>,
    value: PBox<Expr>,
    span: Span,
  ) -> Self {
    Self {
      mutability,
      kind,
      pattern,
      ty,
      value,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub enum DeclKind {
  Val,
  Imu,
  Mut,
}

#[derive(Clone, Debug)]
pub struct Fun {
  pub public: Public,
  pub asyncness: Async,
  pub unsafeness: Unsafe,
  pub wasm: Wasm,
  pub prototype: Prototype,
  pub body: PBox<Block>,
  pub span: Span,
}

impl Fun {
  pub const fn new(
    public: Public,
    asyncness: Async,
    unsafeness: Unsafe,
    wasm: Wasm,
    prototype: Prototype,
    body: PBox<Block>,
    span: Span,
  ) -> Self {
    Self {
      public,
      asyncness,
      unsafeness,
      wasm,
      prototype,
      body,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Prototype {
  pub pattern: PBox<Expr>,
  pub inputs: Vec<PBox<Arg>>,
  pub output: ReturnTy,
  pub span: Span,
}

impl Prototype {
  pub const fn new(
    pattern: PBox<Expr>,
    inputs: Vec<PBox<Arg>>,
    output: ReturnTy,
    span: Span,
  ) -> Self {
    Self {
      pattern,
      inputs,
      output,
      span,
    }
  }

  pub fn as_inputs_tys(&self) -> Vec<PBox<Ty>> {
    self
      .inputs
      .iter()
      .map(|input| input.ty.to_owned())
      .collect::<Vec<_>>()
  }
}

#[derive(Clone, Debug)]
pub struct Arg {
  pub pattern: Pattern,
  pub ty: PBox<Ty>,
  pub span: Span,
}

impl Arg {
  pub const fn new(pattern: Pattern, ty: PBox<Ty>, span: Span) -> Self {
    Self { pattern, ty, span }
  }
}

#[derive(Clone, Debug)]
pub enum ReturnTy {
  Default(Span),
  Ty(PBox<Ty>),
}

#[derive(Clone, Debug)]
pub struct Block {
  pub exprs: Vec<PBox<Expr>>,
  pub span: Span,
}

impl Block {
  pub const fn new(exprs: Vec<PBox<Expr>>, span: Span) -> Self {
    Self { exprs, span }
  }
}

#[derive(Clone, Debug)]
pub struct Unit {
  pub binds: Vec<PBox<Stmt>>,
  pub mocks: Vec<PBox<Fun>>,
  pub tests: Vec<PBox<Fun>>,
  pub span: Span,
}

impl Unit {
  pub const fn new(
    binds: Vec<PBox<Stmt>>,
    mocks: Vec<PBox<Fun>>,
    tests: Vec<PBox<Fun>>,
    span: Span,
  ) -> Self {
    Self {
      binds,
      mocks,
      tests,
      span,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Expr {
  pub kind: ExprKind,
  pub span: Span,
}

impl Expr {
  pub const fn new(kind: ExprKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum ExprKind {
  Stmt(PBox<Stmt>),
  Decl(PBox<Decl>),
  Lit(PBox<Lit>),
  Identifier(String),
  UnOp(UnOp, PBox<Expr>),
  BinOp(PBox<Expr>, BinOp, PBox<Expr>),
  Call(PBox<Expr>, Vec<PBox<Expr>>),
  Assign(PBox<Expr>, BinOp, PBox<Expr>),
  AssignOp(PBox<Expr>, BinOp, PBox<Expr>),
  Return(Option<PBox<Expr>>),
  Block(PBox<Block>),
  Loop(PBox<Block>),
  While(PBox<Expr>, PBox<Block>),
  Until(PBox<Expr>, PBox<Block>),
  Break(Option<PBox<Expr>>),
  Continue,
  Raise(Option<PBox<Expr>>),
  When(PBox<Expr>, PBox<Expr>, PBox<Expr>),
  IfElse(PBox<Expr>, PBox<Expr>, Option<PBox<Expr>>),
  Lambda(Vec<PBox<Expr>>, PBox<Expr>),
  Array(Vec<PBox<Expr>>),
  Index(PBox<Expr>, PBox<Expr>),
  Tuple(Vec<PBox<Expr>>),
  TupleAccess(PBox<Expr>, PBox<Expr>),
  MemberAccess(PBox<Expr>, PBox<Expr>),
}

#[derive(Clone, Debug)]
pub struct Lit {
  pub kind: LitKind,
  pub span: Span,
}

impl Lit {
  pub const fn new(kind: LitKind, span: Span) -> Self {
    Self { kind, span }
  }
}

#[derive(Clone, Debug)]
pub enum LitKind {
  Bool(bool),
  Int(i64),
  Real(f64),
  Str(String),
}

pub type UnOp = Spanned<UnOpKind>;

#[derive(Clone, Debug)]
pub enum UnOpKind {
  Not,
  Neg,
}

pub type BinOp = Spanned<BinOpKind>;

#[derive(Clone, Debug)]
pub enum BinOpKind {
  Add,    // +
  Sub,    // -
  Mul,    // *
  Div,    // /
  Rem,    // %
  And,    // &&
  Or,     // ||
  Lt,     // <
  Gt,     // >
  Le,     // <=
  Ge,     // >=
  Eq,     // ==
  Ne,     // !=
  Shl,    // <<
  Shr,    // >>
  BitAnd, // &
  BitOr,  // |
  BitXor, // ^
  As,     // as
  Range,  // ..
}

impl BinOpKind {
  pub fn is_assign_op(&self) -> bool {
    matches!(
      self,
      Self::Add
        | Self::Sub
        | Self::Mul
        | Self::Div
        | Self::Rem
        | Self::BitXor
        | Self::BitAnd
        | Self::BitOr
    )
  }
}
