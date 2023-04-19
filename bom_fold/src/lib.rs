//! This library provides utilties for folding and rendering various abstract item lists.
//! The prototypical use-case being conversion of BOMs with "levels" into a tree-like structure for
//! parsing/manipulation.
//!
//! For example, the input might be a bom flat file that looks like
//!
//! |---------|--------|--------|
//! |  num    |  name  |  level |
//! |---------|--------|--------|
//! |  Foo    |  Foo   |    1   |
//! |---------|--------|--------|
//! |  Bar    |  Bar   |    2   |
//! |---------|--------|--------|
//! |  Baz    |  Baz   |    1   |
//! |---------|--------|--------|
//!
//! Which indicates that "Foo" and "Baz" are top level items, whereas Bar is a child of of Foo:
//!
//! The main entrypoint to this library is the [`transform`](./transform/mod.rs#transform)
//! function.

mod parse;

mod transform;
pub use transform::*;

mod materialize;
pub use materialize::*;
