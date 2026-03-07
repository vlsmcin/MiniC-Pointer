# MiniC

**[→ Short summary of MiniC](doc/summary.md)** — language overview, types, and pipeline.

---

## Quick Start

```bash
cargo build
cargo test
```

---

## Architecture

MiniC is organized into these main components:

| Component | Path | Description |
|-----------|------|-------------|
| [**AST**](doc/architecture/ast.md) | `src/ir/` | Abstract syntax tree parameterized by phase (unchecked vs checked). Defines `Expr`, `Statement`, `Program`, and type synonyms (`UncheckedProgram`, `CheckedProgram`, etc.). |
| [**Parser**](doc/architecture/parser.md) | `src/parser/` | Parser combinators using [nom](https://github.com/rust-bakery/nom). Parses literals, expressions, statements, and function declarations into an unchecked AST. |
| [**Semantic**](doc/design/type-checker.md) | `src/semantic/` | Type checker. Consumes unchecked AST, validates types, produces checked AST. Requires `main`; enforces variable declarations and type compatibility. |
| [**Environment**](src/environment/) | `src/environment/` | Symbol table for variable bindings and function signatures. Used by the type checker for name resolution. |

```
src/
├── ir/           # AST (ast.rs, mod.rs)
├── parser/       # Parser (expressions, statements, functions, literals, identifiers)
├── semantic/     # Type checker
└── environment/  # Environment (variable bindings, function signatures)
```

---

## Testing

MiniC uses **integration tests** in the `tests/` directory. All tests use only the public API; there are no `#[cfg(test)]` blocks in source modules.

| Test file | Purpose |
|-----------|---------|
| [**parser.rs**](tests/parser.rs) | Parser unit-style tests: literals, identifiers, expressions, statements. Uses inline strings. |
| [**program.rs**](tests/program.rs) | Full-program parsing from fixture files in `tests/fixtures/`. |
| [**type_checker.rs**](tests/type_checker.rs) | Semantic tests: parse + type-check, assert on success/failure or typed AST. |

**Run all tests:** `cargo test`

For details on test organization, patterns, and how to add new tests, see [**Test Architecture**](doc/architecture/tests.md).

---

## Specifications

Formal specs live under [openspec/specs/](openspec/specs/) and [openspec/changes/](openspec/changes/). Main specs:

- [AST](openspec/specs/ast/spec.md)
- [Functions](openspec/specs/functions/spec.md)
- [Arrays](openspec/specs/arrays/spec.md)
- [Type checker](openspec/specs/type-checker/spec.md)
- [Parser documentation](openspec/specs/parser-docs/spec.md)
