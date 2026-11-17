//@ compile-flags: -g -Copt-level=3

// TODO description

#![crate_type = "lib"]

pub const C: i32 = 123;

pub fn test() -> i32 {
    C
}

// CHECK-DAG: ![[EXPR:[0-9]+]] = !DIGlobalVariableExpression(var: ![[VAR:[0-9]+]], expr: !DIExpression(DW_OP_constu, 123, DW_OP_stack_value))
// CHECK-DAG: ![[VAR]] = distinct !DIGlobalVariable(name: "C",{{.*}}line: 7,{{.*}}type: ![[TYPE:[0-9]+]]
// CHECK-DAG: ![[TYPE]] = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)

// CHECK-DAG: !DICompileUnit({{.*}}globals: ![[GLOBALS:[0-9]+]]
// CHECK-DAG: ![[GLOBALS]] = !{![[EXPR]]}
