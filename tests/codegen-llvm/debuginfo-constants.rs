//@ compile-flags: -g -Copt-level=3

// TODO description

#![crate_type = "lib"]

pub const A: u8 = 123;
pub const B: i32 = -1;
pub const C: f32 = 1.5;
pub const D: i128 = -1;
pub const E: u128 = 456;

pub fn test() -> i32 {
    A as i32 + B + C as i32 + D as i32 + E as i32
}

// CHECK-DAG: ![[EXPR_A:[0-9]+]] = !DIGlobalVariableExpression(var: ![[VAR_A:[0-9]+]], expr: !DIExpression(DW_OP_constu, 123, DW_OP_stack_value))
// CHECK-DAG: ![[VAR_A]] = distinct !DIGlobalVariable(name: "A",{{.*}}line: 7,{{.*}}type: ![[TYPE_A:[0-9]+]]
// CHECK-DAG: ![[TYPE_A]] = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)

// CHECK-DAG: ![[EXPR_B:[0-9]+]] = !DIGlobalVariableExpression(var: ![[VAR_B:[0-9]+]], expr: !DIExpression(DW_OP_constu, 4294967295, DW_OP_stack_value))
// CHECK-DAG: ![[VAR_B]] = distinct !DIGlobalVariable(name: "B",{{.*}}line: 8,{{.*}}type: ![[TYPE_B:[0-9]+]]
// CHECK-DAG: ![[TYPE_B]] = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)

// CHECK-DAG: ![[EXPR_C:[0-9]+]] = !DIGlobalVariableExpression(var: ![[VAR_C:[0-9]+]], expr: !DIExpression(DW_OP_constu, 1069547520, DW_OP_stack_value))
// CHECK-DAG: ![[VAR_C]] = distinct !DIGlobalVariable(name: "C",{{.*}}line: 9,{{.*}}type: ![[TYPE_C:[0-9]+]]
// CHECK-DAG: ![[TYPE_C]] = !DIBasicType(name: "f32", size: 32, encoding: DW_ATE_float)

// CHECK-DAG: ![[EXPR_D:[0-9]+]] = !DIGlobalVariableExpression(var: ![[VAR_D:[0-9]+]], expr: !DIExpression())
// CHECK-DAG: ![[VAR_D]] = distinct !DIGlobalVariable(name: "D",{{.*}}line: 10,{{.*}}type: ![[TYPE_D:[0-9]+]]
// CHECK-DAG: ![[TYPE_D]] = !DIBasicType(name: "i128", size: 128, encoding: DW_ATE_signed)
// CHECK-DAG: @[[CONST_D:[^ ]+]] = private unnamed_addr constant i128 -1, align {{[0-9]+}}, !dbg ![[EXPR_D]]

// CHECK-DAG: ![[EXPR_E:[0-9]+]] = !DIGlobalVariableExpression(var: ![[VAR_E:[0-9]+]], expr: !DIExpression(DW_OP_constu, 456, DW_OP_stack_value))
// CHECK-DAG: ![[VAR_E]] = distinct !DIGlobalVariable(name: "E",{{.*}}line: 11,{{.*}}type: ![[TYPE_E:[0-9]+]]
// CHECK-DAG: ![[TYPE_E]] = !DIBasicType(name: "u128", size: 128, encoding: DW_ATE_unsigned)

// CHECK-DAG: @llvm.used = appending global [1 x ptr] [ptr @[[CONST_D]]], section "llvm.metadata"
// CHECK-DAG: !DICompileUnit({{.*}}globals: ![[GLOBALS:[0-9]+]]
// CHECK-DAG: ![[GLOBALS]] = !{![[EXPR_A]], ![[EXPR_B]], ![[EXPR_C]], ![[EXPR_D]], ![[EXPR_E]]}
