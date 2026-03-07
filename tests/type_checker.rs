//! Integration tests for the MiniC type checker.

use nom::combinator::all_consuming;
use mini_c::ir::ast::{CheckedProgram, Type};
use mini_c::parser::program;
use mini_c::semantic::type_check;

fn parse_and_type_check(src: &str) -> Result<CheckedProgram, mini_c::semantic::TypeError> {
    let (_, prog) = all_consuming(program)(src).map_err(|_| {
        mini_c::semantic::TypeError {
            message: "parse failed".to_string(),
        }
    })?;
    type_check(&prog)
}

#[test]
fn test_type_check_simple_assign() {
    let result = parse_and_type_check("void main() int x = 1");
    assert!(result.is_ok());
}

#[test]
fn test_type_check_int_float_coercion() {
    let result = parse_and_type_check("void main() float x = 1 + 3.14");
    assert!(result.is_ok());
    let prog = result.unwrap();
    let main_fn = prog.functions.iter().find(|f| f.name == "main").unwrap();
    if let mini_c::ir::ast::Statement::Decl { ref init, .. } = main_fn.body.stmt {
        assert_eq!(init.ty, Type::Float);
    } else {
        panic!("expected Decl");
    }
}

#[test]
fn test_type_check_undeclared_var() {
    let result = parse_and_type_check("void main() x = y");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("undeclared"));
}

#[test]
fn test_type_check_bool_condition() {
    let result = parse_and_type_check("void main() if true then int x = 1");
    assert!(result.is_ok());
}

#[test]
fn test_type_check_array_literal() {
    let result = parse_and_type_check("void main() int[] x = [1, 2, 3]");
    assert!(result.is_ok());
}

#[test]
fn test_type_check_array_index() {
    let result = parse_and_type_check("void main() { int[] arr = [1, 2]; int x = arr[0] }");
    assert!(result.is_ok());
}

#[test]
fn test_type_check_call_arg_type_mismatch() {
    let result = parse_and_type_check("void foo(int x) x = x\nvoid main() foo(true)");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("argument"));
}

#[test]
fn test_type_check_missing_main() {
    let result = parse_and_type_check("void foo() int x = 1");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("main"));
}

#[test]
fn test_type_check_decl_then_assign() {
    let result = parse_and_type_check("void main() { int x = 1; x = 2 }");
    assert!(result.is_ok());
}

#[test]
fn test_type_check_assign_undeclared() {
    let result = parse_and_type_check("void main() x = 1");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("undeclared"));
}

#[test]
fn test_type_check_redeclaration() {
    let result = parse_and_type_check("void main() { int x = 1; int x = 2 }");
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("redeclaration"));
}
