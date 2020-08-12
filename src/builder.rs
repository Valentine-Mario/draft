use std::ffi::CString;
use std::ptr;
use llvm_sys::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::parser::Expression;

pub unsafe fn codegen(input: Vec<Expression>) {
    let context = llvm::core::LLVMContextCreate();
    let module = llvm::core::LLVMModuleCreateWithName(b"example_module\0".as_ptr() as *const _);
    let builder = llvm::core::LLVMCreateBuilderInContext(context);

    // In LLVM, you get your types from functions.
    let int_type = llvm::core::LLVMInt64TypeInContext(context);
    let function_type = llvm::core::LLVMFunctionType(int_type, ptr::null_mut(), 0, 0);
    let function = llvm::core::LLVMAddFunction(module, b"main\0".as_ptr() as *const _, function_type);

    let entry_name = CString::new("entry").unwrap();
    let bb = llvm::core::LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr());
    llvm::core::LLVMPositionBuilderAtEnd(builder, bb);

    let mut names = HashMap::new();
    insert_allocations(context, builder, &mut names, &input);

    let int_type = llvm::core::LLVMInt64TypeInContext(context);
    let zero = llvm::core::LLVMConstInt(int_type, 0, 0);

    let mut return_value = zero; // return value on empty program
    for expr in input {
        return_value = codegen_expr(context, builder, function, &mut names, expr);
    }
    llvm::core::LLVMBuildRet(builder, return_value);

    // Instead of dumping to stdout, let's write out the IR to `out.ll`
    let out_file = CString::new("out.ll").unwrap();
    llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

    llvm::core::LLVMDisposeBuilder(builder);
    llvm::core::LLVMDisposeModule(module);
    llvm::core::LLVMContextDispose(context);

}







unsafe fn insert_allocations(context: LLVMContextRef, builder: LLVMBuilderRef, names: &mut HashMap<String, LLVMValueRef>, exprs: &[Expression]) {
    let mut variable_names = HashSet::new();

for expr in exprs {
    match *expr {
            Expression::Assign(ref name, _) => {
                variable_names.insert(name);
            },

            _ => {},
        }
    }   

    for variable_name in variable_names {
        let int_type = llvm::core::LLVMInt64TypeInContext(context);
        let name = CString::new(variable_name.as_bytes()).unwrap();
        let pointer = llvm::core::LLVMBuildAlloca(builder, int_type, name.as_ptr());

        names.insert(variable_name.to_owned(), pointer);
    }
}


unsafe fn codegen_expr(context: LLVMContextRef, builder: LLVMBuilderRef, func: LLVMValueRef, names: &mut HashMap<String, LLVMValueRef>, expr: Expression) -> LLVMValueRef {
    match expr {
        Expression::Number(int_literal) => {
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            llvm::core::LLVMConstInt(int_type, int_literal, 0)
        },

        Expression::Sum(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, func, names, *lhs);
            let rhs = codegen_expr(context, builder, func, names, *rhs);

            let name = CString::new("addtmp").unwrap();
            llvm::core::LLVMBuildAdd(builder, lhs, rhs, name.as_ptr())
        },

        Expression::Subtract(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, func, names, *lhs);
            let rhs = codegen_expr(context, builder, func, names, *rhs);

            let name = CString::new("subtmp").unwrap();
            llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr())
        },

        Expression::Product(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, func, names, *lhs);
            let rhs = codegen_expr(context, builder, func, names, *rhs);

            let name = CString::new("multmp").unwrap();
            llvm::core::LLVMBuildMul(builder, lhs, rhs, name.as_ptr())
        },

        Expression::Division(lhs, rhs) => {
            let lhs = codegen_expr(context, builder, func, names, *lhs);
            let rhs = codegen_expr(context, builder, func, names, *rhs);

            let name = CString::new("divtmp").unwrap();
            llvm::core::LLVMBuildUDiv(builder, lhs, rhs, name.as_ptr())
        },
        Expression::Ref(name) => {
            let pointer = names.get(&name).unwrap();
            let name = CString::new(name).unwrap();
            llvm::core::LLVMBuildLoad(builder, *pointer, name.as_ptr())
        },

        Expression::Assign(name, expr) => {
            let new_value = codegen_expr(context, builder, func, names, *expr);
            let pointer = names.get(&name).unwrap();
            llvm::core::LLVMBuildStore(builder, new_value, *pointer);
            new_value
        },

        Expression::If(condition, then_body, else_body) => {
            let condition_value = codegen_expr(context, builder, func, names, *condition);
            let int_type = llvm::core::LLVMInt64TypeInContext(context);
            let zero = llvm::core::LLVMConstInt(int_type, 0, 0);

            // `is_nonzero` is a 1-bit integer
            let name = CString::new("is_nonzero").unwrap();
            let is_nonzero = llvm::core::LLVMBuildICmp(builder, llvm::LLVMIntPredicate::LLVMIntNE, condition_value, zero, name.as_ptr());

            let entry_name = CString::new("entry").unwrap();
            let then_block = llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
            let else_block = llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());
            let merge_block = llvm::core::LLVMAppendBasicBlockInContext(context, func, entry_name.as_ptr());

            llvm::core::LLVMBuildCondBr(builder, is_nonzero, then_block, else_block);

            llvm::core::LLVMPositionBuilderAtEnd(builder, then_block);
            let mut then_return = zero;
            for expr in then_body {
                then_return = codegen_expr(context, builder, func, names, expr);
            }
            llvm::core::LLVMBuildBr(builder, merge_block);
            let then_block = llvm::core::LLVMGetInsertBlock(builder);

            llvm::core::LLVMPositionBuilderAtEnd(builder, else_block);
            let mut else_return = zero;
            for expr in else_body {
                else_return = codegen_expr(context, builder, func, names, expr);
            }
            llvm::core::LLVMBuildBr(builder, merge_block);
            let else_block = llvm::core::LLVMGetInsertBlock(builder);

            // Insert the phi node for branching
            llvm::core::LLVMPositionBuilderAtEnd(builder, merge_block);
            let phi_name = CString::new("iftmp").unwrap();
            let phi = llvm::core::LLVMBuildPhi(builder, int_type, phi_name.as_ptr());

            let mut values = vec![then_return, else_return];
            let mut blocks = vec![then_block, else_block];

            llvm::core::LLVMAddIncoming(phi, values.as_mut_ptr(), blocks.as_mut_ptr(), 2);
            phi
        }
    }
}