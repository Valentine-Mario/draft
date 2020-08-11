
extern crate peg;
extern crate llvm_sys as llvm;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;
use peg::parser;
use llvm_sys::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expression {
    Number(u64),
    Sum(Box<Expression>, Box<Expression>),
    Product(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Ref(String),
    Assign(String, Box<Expression>)
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("in.drf").unwrap();
    f.read_to_string(&mut input).unwrap();
    //lli-3.9 out.ll ; echo $?

    let parsed_input = arithmetic::expression(&input).unwrap();
    println!("{:?}", parsed_input);
    
    unsafe {
        codegen(parsed_input);
    }
}


parser!{
    /// Doc comment
    grammar arithmetic() for str {
        /// Top level parser rule
        /// This doc comment has multiple lines to test support for that as well
        pub rule expression() -> Vec<Expression>
            = statements()
    
        rule _() = [' ' | '\n']*

        rule statements() -> Vec<Expression>
           = e:(equal() ** ("\n" _)) "\n"? { e }

        rule equal()->Expression
            =i:identifier() _ "=" _ s:sum() {Expression::Assign(i, Box::new(s))} 
            /sum()
           
    
        rule sum() -> Expression
            = l:product() _ "+" _ r:product() { Expression::Sum(Box::new(l), Box::new(r)) }
            / l:product() _ "-" _ r:product() {Expression::Subtract(Box::new(l), Box::new(r))}
            /"inc" _ l:atom() {Expression::Sum(Box::new(l), Box::new(Expression::Number(1)))}
            /"dec" _ l:atom() {Expression::Subtract(Box::new(l), Box::new(Expression::Number(1)))}
            / product()
           
    
        rule product() -> Expression
            = l:atom() _ "*" _ r:atom() { Expression::Product(Box::new(l), Box::new(r)) }
            / l:atom() _ "/" _ r:atom() { Expression::Division(Box::new(l), Box::new(r)) }
            /"square" _ l:atom() { Expression::Product(Box::new(l.clone()), Box::new(l)) }
            / atom()
            

        rule atom() -> Expression
            = reference()
            / "(" _ v:sum() _ ")" { v }

        rule reference()->Expression
            =l:identifier() {Expression::Ref(l)}
            /number()

        rule identifier()->String
            = n:$(['a'..='z'|'A'..='Z']+){n.to_string()}
            
    
        rule number() -> Expression
            = n:$(['0'..='9']+) { Expression::Number(n.parse().unwrap()) }
    }}
    
    unsafe fn codegen(input: Vec<Expression>) {
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
            return_value = codegen_expr(context, builder, &mut names, expr);
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


    unsafe fn codegen_expr(context: LLVMContextRef, builder: LLVMBuilderRef, names: &mut HashMap<String, LLVMValueRef>, expr: Expression) -> LLVMValueRef {
        match expr {
            Expression::Number(int_literal) => {
                let int_type = llvm::core::LLVMInt64TypeInContext(context);
                llvm::core::LLVMConstInt(int_type, int_literal, 0)
            },
    
            Expression::Sum(lhs, rhs) => {
                let lhs = codegen_expr(context, builder, names, *lhs);
                let rhs = codegen_expr(context, builder, names, *rhs);
    
                let name = CString::new("addtmp").unwrap();
                llvm::core::LLVMBuildAdd(builder, lhs, rhs, name.as_ptr())
            },
    
            Expression::Subtract(lhs, rhs) => {
                let lhs = codegen_expr(context, builder, names, *lhs);
                let rhs = codegen_expr(context, builder, names, *rhs);
    
                let name = CString::new("subtmp").unwrap();
                llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr())
            },
    
            Expression::Product(lhs, rhs) => {
                let lhs = codegen_expr(context, builder, names, *lhs);
                let rhs = codegen_expr(context, builder, names, *rhs);
    
                let name = CString::new("multmp").unwrap();
                llvm::core::LLVMBuildMul(builder, lhs, rhs, name.as_ptr())
            },
    
            Expression::Division(lhs, rhs) => {
                let lhs = codegen_expr(context, builder, names, *lhs);
                let rhs = codegen_expr(context, builder, names, *rhs);
    
                let name = CString::new("divtmp").unwrap();
                llvm::core::LLVMBuildUDiv(builder, lhs, rhs, name.as_ptr())
            },
            Expression::Ref(name) => {
                let pointer = names.get(&name).unwrap();
                let name = CString::new(name).unwrap();
                llvm::core::LLVMBuildLoad(builder, *pointer, name.as_ptr())
            },
    
            Expression::Assign(name, expr) => {
                let new_value = codegen_expr(context, builder, names, *expr);
                let pointer = names.get(&name).unwrap();
                llvm::core::LLVMBuildStore(builder, new_value, *pointer);
                new_value
            },
        }
    }