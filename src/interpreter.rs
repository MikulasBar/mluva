use crate::external::ExternalFunction;
use crate::errors::InterpreterError;
use crate::function_table::FunctionTable;
use crate::instruction::Instruction;
use crate::value::Value;

pub struct Interpreter<'a> {
    function_table: &'a FunctionTable,
    index: usize,
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    slots: Vec<Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(function_table: &'a FunctionTable, instructions: Vec<Instruction>, slot_used: usize) -> Self {
        Self {
            function_table,
            instructions,
            index: 0,
            stack: vec![],
            slots: vec![Value::Void; slot_used],
        }
    }

    pub fn interpret(&mut self) -> Result<(), InterpreterError> {
        while self.index < self.instructions.len() {
            let instruction = &self.instructions[self.index];
            match instruction {
                Instruction::Push(value) => {
                    self.stack.push(value.clone());
                }
                Instruction::Pop => {
                    self.stack.pop().ok_or(InterpreterError::ValueStackUnderflow)?;
                }
                Instruction::Store(slot) => {
                    self.slots[*slot] = self.stack.pop().ok_or(InterpreterError::ValueStackUnderflow)?;
                }
                Instruction::Load(slot) => {
                    self.stack.push(self.slots[*slot].clone());
                }
                Instruction::Call { slot, arg_count } => {
                    let args = self.stack.split_off(self.stack.len() - arg_count);
                    let func = self.function_table.get_fn_by_index(*slot).unwrap();
                    let result = func.call(args)?;
                    self.stack.push(result);
                }
                
                Instruction::Return => {
                    break; // TODO: Handle return logic here
                }

                Instruction::Jump(target) => {
                    // Jump to target
                    self.index = *target;
                    continue; // Skip the index increment below
                }

                _ => todo!(),
            }
            self.index += 1;
        }

        Ok(())
    }

}
    
//     fn interpret_stmts(&mut self, stmts: &[Stmt]) -> Result<(), InterpreterError> {
//         for s in stmts {
//             match s {
//                 Stmt::VarAssign(ident, expr) => {
//                     let value = self.eval_expr(&expr)?;
//                    self.scope.change(&ident, value);
//                 },
    
//                 Stmt::VarDeclare(_, ident, expr) => {
//                     let value = self.eval_expr(&expr)?;
//                    self.scope.insert_new(ident.clone(), value);
//                 },
    
//                 Stmt::If(cond, stmts, else_stmts) => {
//                     let cond = self.eval_expr(&cond)?.expect_bool();
//                     if cond {
//                        self.interpret_stmts(stmts)?;
//                     } else if else_stmts.is_some() {
//                        self.interpret_stmts(else_stmts.as_ref().unwrap())?;
//                     }
//                 },
    
//                 Stmt::While(cond, stmts) => {
//                     while self.eval_expr(&cond)?.expect_bool() {
//                        self.interpret_stmts(stmts)?;
//                     }
//                 },

//                 Stmt::Expr(expr) => {
//                    self.eval_expr(&expr)?;
//                 },
//             }
//         }

//         Ok(())
//     }

//     fn eval_expr(&mut self, expr: &Expr) -> Result<Value, InterpreterError> {
//         let result = match expr {
//             Expr::Literal(lit) => lit.clone().into(),
//             Expr::Var(ident) => {
//                 if let Some(value) = self.scope.get(&ident) {
//                     value.clone()
//                 } else {
//                     return Err(InterpreterError::UndefinedVariable(ident.clone()));
//                 }
//             }
//             Expr::BinOp(op, lhs, rhs) => return self.apply_operator(op, &*lhs, &*rhs),
//             Expr::FuncCall(name, args) => {
//                 let Some(func) = self.functions.get(name.as_str()) else {
//                     return Err(InterpreterError::UndefinedFunction(name.clone()));
//                 };

//                 let args = args.iter()
//                     .map(|arg| self.eval_expr(arg))
//                     .collect::<Result<Vec<_>, _>>()?;

//                 func.call(args)?
//             },
//         };

//         Ok(result)
//     }

//     pub fn apply_operator(&mut self, op: &BinOp, lhs: &Expr, rhs: &Expr) -> Result<Value, InterpreterError> {
//         let lhs = self.eval_expr(lhs)?;
//         let rhs = self.eval_expr(rhs)?;

//         let result = match op {
//             BinOp::Add => apply_numeric_op(lhs, rhs, |l, r| l + r, |l, r| l + r),
//             BinOp::Sub => apply_numeric_op(lhs, rhs, |l, r| l - r, |l, r| l - r),
//             BinOp::Mul => apply_numeric_op(lhs, rhs, |l, r| l * r, |l, r| l * r),
//             BinOp::Div => {
//                 if rhs == Value::Int(0) || rhs == Value::Float(0.0) {
//                     return Err(InterpreterError::ValueError);
//                 }
//                 apply_numeric_op(lhs, rhs, |l, r| l / r, |l, r| l / r)
//             },
//             BinOp::Modulo => {
//                 if rhs == Value::Int(0) || rhs == Value::Float(0.0) {
//                     return Err(InterpreterError::ValueError);
//                 }
//                 apply_numeric_op(lhs, rhs, |l, r| l % r, |l, r| l % r)
//             },
            
//             BinOp::Equal => Ok(Value::Bool(lhs == rhs)),
//             BinOp::NotEqual => Ok(Value::Bool(lhs != rhs)),
//         };

//         result
//     }
// }


// fn apply_numeric_op<FInt, FFloat>(
//     lhs: Value,
//     rhs: Value,
//     int_op: FInt,
//     float_op: FFloat,
// ) -> Result<Value, InterpreterError>
// where
//     FInt: FnOnce(u64, u64) -> u64,
//     FFloat: FnOnce(f64, f64) -> f64,
// {
//     match (lhs, rhs) {
//         (Value::Int(l), Value::Int(r)) => Ok(int_op(l, r).into()),
//         (Value::Float(l), Value::Float(r)) => Ok(float_op(l, r).into()),
//         _ => Err(InterpreterError::TypeError),
//     }
// }