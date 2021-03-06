mod linecontext;
mod operation;
mod parser;
mod token;
mod types;

use linecontext::LineContext;
use parser::Parser;
use types::Type;

use operation::Operation;
use std::collections::HashMap;

/// The main interpreter struct which contains the state of the interpreter
pub struct Interpreter {
    stack: Vec<Type>,
    names: HashMap<String, Type>,
    op_pointer: usize,
}

impl Interpreter {
    pub fn run(source: String) -> Result<(), ()> {
        let mut s = Self {
            stack: Vec::new(),
            names: HashMap::new(),
            op_pointer: 0,
        };

        let tokens = Parser::run(source);

        for token in tokens.iter() {
            println!("{:?}", token);
        }

        Ok(())
        //s.eval(source)
    }

    fn eval(&mut self, source: String) -> Result<(), ()> {
        for (line_number, line) in source.lines().enumerate() {
            self.eval_line(line, line_number)?;
        }
        Ok(())
    }

    fn eval_line(&mut self, line: &str, line_number: usize) -> Result<(), ()> {
        Ok(())
    }

    fn eval_operations(
        &mut self,
        operations: Vec<Operation>,
        line_context: LineContext,
    ) -> Result<(), ()> {
        while self.op_pointer < operations.len() {
            let operation = &operations[self.op_pointer];
            match operation {
                Operation::StoreName(name) => self.store_name(name, line_context)?,
                Operation::PushName(name) => self.push_name(name, line_context)?,
                Operation::PushConst(cnst) => self.push_const(cnst, line_context)?,
                Operation::BinaryAdd => self.binary_add(line_context)?,
                Operation::BinarySub => self.binary_sub(line_context)?,
                Operation::BinaryMul => self.binary_mul(line_context)?,
                Operation::BinaryDiv => self.binary_div(line_context)?,
                Operation::BinaryMod => self.binary_mod(line_context)?,
                Operation::ComparisonEq => self.comparison_eq(line_context)?,
                Operation::ComparisonGreater => self.comparison_greater(line_context)?,
                Operation::ComparisonGreaterEqual => self.comparison_greater_eq(line_context)?,
                Operation::ComparisonLess => self.comparison_less(line_context)?,
                Operation::ComparisonLessEqual => self.comparison_less_eq(line_context)?,
                Operation::JumpTrue(loc) => {
                    self.jump_true(*loc, line_context)?;
                    // Continue to prevent operation pointer being incremented
                    continue;
                }
                Operation::JumpFalse(loc) => {
                    self.jump_false(*loc, line_context)?;
                    // Continue to prevent operation pointer being incremented
                    continue;
                }

                Operation::IntrinsicPrint => self.intrinsic_print(line_context)?,
            }
            self.op_pointer += 1;
        }
        Ok(())
    }
}

// Operation implementations
impl Interpreter {
    fn store_name(&mut self, name: &str, line_context: LineContext) -> Result<(), ()> {
        // Unwrap is safe because operations are generated by the interpreter and verified before this line
        let value = self.stack.pop().unwrap();
        match self.names.get(name) {
            Some(v) => {
                // Compare the two enum values and not the values that are contained within those enums
                if std::mem::discriminant(v) == std::mem::discriminant(&value) {
                    self.names.insert(name.to_string(), value);
                    Ok(())
                } else {
                    error!(
                        "Incorrect type when assigning to variable \"{}\" line:{}",
                        name, line_context.line_number
                    );
                    Err(())
                }
            }
            None => {
                self.names.insert(name.to_string(), value);
                Ok(())
            }
        }
    }

    fn push_name(&mut self, name: &str, line_context: LineContext) -> Result<(), ()> {
        match self.names.get(name) {
            Some(v) => {
                self.stack.push(v.clone());
                Ok(())
            }
            None => {
                error!(
                    "Undefined variable \"{}\" line:{}",
                    name, line_context.line_number
                );
                Err(())
            }
        }
    }

    fn push_const(&mut self, value: &Type, line_context: LineContext) -> Result<(), ()> {
        self.stack.push(value.clone());
        Ok(())
    }

    fn binary_add(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let left = self.stack.pop().unwrap();
        let right = self.stack.pop().unwrap();
        match left {
            Type::Int(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left + right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 + right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 + right)),
                Type::Char(right) => self.stack.push(Type::Int(left + right as i64)),
                _ => {
                    error!("Cannot add type Int with {:?}", right);
                    return Err(());
                }
            },
            Type::Uint(left) => match right {
                Type::Int(right) => self.stack.push(Type::Uint(left + right as u64)),
                Type::Uint(right) => self.stack.push(Type::Uint(left + right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 + right)),
                Type::Char(right) => self.stack.push(Type::Uint(left + right as u64)),
                _ => {
                    error!("Cannot add type Uint with {:?}", right);
                    return Err(());
                }
            },
            Type::Float(left) => match right {
                Type::Int(right) => self.stack.push(Type::Float(left + right as f64)),
                Type::Uint(right) => self.stack.push(Type::Float(left + right as f64)),
                Type::Float(right) => self.stack.push(Type::Float(left + right)),
                Type::Char(right) => self.stack.push(Type::Float(left + right as f64)),
                _ => {
                    error!("Cannot add type F64 with {:?}", right);
                    return Err(());
                }
            },
            Type::Char(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left as i64 + right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 + right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 + right)),
                Type::Char(right) => self.stack.push(Type::Char(left + right)),
                _ => {
                    error!("Cannot add type Char with {:?}", right);
                    return Err(());
                }
            },
            _ => {
                error!("Cannot add type {:?} with {:?}", left, right);
                return Err(());
            }
        }
        Ok(())
    }

    fn binary_sub(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        match left {
            Type::Int(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left - right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 - right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 - right)),
                Type::Char(right) => self.stack.push(Type::Int(left - right as i64)),
                _ => {
                    error!("Cannot subtract type Int with {:?}", right);
                    return Err(());
                }
            },
            Type::Uint(left) => match right {
                Type::Int(right) => self.stack.push(Type::Uint(left - right as u64)),
                Type::Uint(right) => self.stack.push(Type::Uint(left - right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 - right)),
                Type::Char(right) => self.stack.push(Type::Uint(left - right as u64)),
                _ => {
                    error!("Cannot subtract type Uint with {:?}", right);
                    return Err(());
                }
            },
            Type::Float(left) => match right {
                Type::Int(right) => self.stack.push(Type::Float(left - right as f64)),
                Type::Uint(right) => self.stack.push(Type::Float(left - right as f64)),
                Type::Float(right) => self.stack.push(Type::Float(left - right)),
                Type::Char(right) => self.stack.push(Type::Float(left - right as f64)),
                _ => {
                    error!("Cannot subtract type F64 with {:?}", right);
                    return Err(());
                }
            },
            Type::Char(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left as i64 - right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 - right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 - right)),
                Type::Char(right) => self.stack.push(Type::Char(left - right)),
                _ => {
                    error!("Cannot subtract type Char with {:?}", right);
                    return Err(());
                }
            },
            _ => {
                error!("Cannot subtract type {:?} with {:?}", left, right);
                return Err(());
            }
        }

        Ok(())
    }

    fn binary_mul(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let left = self.stack.pop().unwrap();
        let right = self.stack.pop().unwrap();

        match left {
            Type::Int(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left * right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 * right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 * right)),
                Type::Char(right) => self.stack.push(Type::Int(left * right as i64)),
                _ => {
                    error!("Cannot multiply type Int with {:?}", right);
                    return Err(());
                }
            },
            Type::Uint(left) => match right {
                Type::Int(right) => self.stack.push(Type::Uint(left * right as u64)),
                Type::Uint(right) => self.stack.push(Type::Uint(left * right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 * right)),
                Type::Char(right) => self.stack.push(Type::Uint(left * right as u64)),
                _ => {
                    error!("Cannot multiply type Uint with {:?}", right);
                    return Err(());
                }
            },
            Type::Float(left) => match right {
                Type::Int(right) => self.stack.push(Type::Float(left * right as f64)),
                Type::Uint(right) => self.stack.push(Type::Float(left * right as f64)),
                Type::Float(right) => self.stack.push(Type::Float(left * right)),
                Type::Char(right) => self.stack.push(Type::Float(left * right as f64)),
                _ => {
                    error!("Cannot multiply type F64 with {:?}", right);
                    return Err(());
                }
            },
            Type::Char(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left as i64 * right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 * right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 * right)),
                Type::Char(right) => self.stack.push(Type::Char(left * right)),
                _ => {
                    error!("Cannot multiply type Char with {:?}", right);
                    return Err(());
                }
            },
            _ => {
                error!("Cannot multiply type {:?} with {:?}", left, right);
                return Err(());
            }
        }

        Ok(())
    }

    fn binary_div(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        match left {
            Type::Int(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left / right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 / right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 / right)),
                Type::Char(right) => self.stack.push(Type::Int(left / right as i64)),
                _ => {
                    error!("Cannot divide type Int with {:?}", right);
                    return Err(());
                }
            },
            Type::Uint(left) => match right {
                Type::Int(right) => self.stack.push(Type::Uint(left / right as u64)),
                Type::Uint(right) => self.stack.push(Type::Uint(left / right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 / right)),
                Type::Char(right) => self.stack.push(Type::Uint(left / right as u64)),
                _ => {
                    error!("Cannot divide type Uint with {:?}", right);
                    return Err(());
                }
            },
            Type::Float(left) => match right {
                Type::Int(right) => self.stack.push(Type::Float(left / right as f64)),
                Type::Uint(right) => self.stack.push(Type::Float(left / right as f64)),
                Type::Float(right) => self.stack.push(Type::Float(left / right)),
                Type::Char(right) => self.stack.push(Type::Float(left / right as f64)),
                _ => {
                    error!("Cannot divide type F64 with {:?}", right);
                    return Err(());
                }
            },
            Type::Char(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left as i64 / right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 / right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 / right)),
                Type::Char(right) => self.stack.push(Type::Char(left / right)),
                _ => {
                    error!("Cannot divide type Char with {:?}", right);
                    return Err(());
                }
            },
            _ => {
                error!("Cannot divide type {:?} with {:?}", left, right);
                return Err(());
            }
        }

        Ok(())
    }

    fn binary_mod(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        match left {
            Type::Int(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left % right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 % right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 % right)),
                Type::Char(right) => self.stack.push(Type::Int(left % right as i64)),
                _ => {
                    error!("Cannot mod type Int with {:?}", right);
                    return Err(());
                }
            },
            Type::Uint(left) => match right {
                Type::Int(right) => self.stack.push(Type::Uint(left % right as u64)),
                Type::Uint(right) => self.stack.push(Type::Uint(left % right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 % right)),
                Type::Char(right) => self.stack.push(Type::Uint(left % right as u64)),
                _ => {
                    error!("Cannot mod type Uint with {:?}", right);
                    return Err(());
                }
            },
            Type::Float(left) => match right {
                Type::Int(right) => self.stack.push(Type::Float(left % right as f64)),
                Type::Uint(right) => self.stack.push(Type::Float(left % right as f64)),
                Type::Float(right) => self.stack.push(Type::Float(left % right)),
                Type::Char(right) => self.stack.push(Type::Float(left % right as f64)),
                _ => {
                    error!("Cannot mod type F64 with {:?}", right);
                    return Err(());
                }
            },
            Type::Char(left) => match right {
                Type::Int(right) => self.stack.push(Type::Int(left as i64 % right)),
                Type::Uint(right) => self.stack.push(Type::Uint(left as u64 % right)),
                Type::Float(right) => self.stack.push(Type::Float(left as f64 % right)),
                Type::Char(right) => self.stack.push(Type::Char(left % right)),
                _ => {
                    error!("Cannot mod type Char with {:?}", right);
                    return Err(());
                }
            },
            _ => {
                error!("Cannot mod type {:?} with {:?}", left, right);
                return Err(());
            }
        }

        Ok(())
    }

    fn comparison_eq(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let left = self.stack.pop().unwrap();
        let right = self.stack.pop().unwrap();

        self.stack.push(Type::Bool(left == right));

        Ok(())
    }

    fn comparison_greater(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        self.stack.push(Type::Bool(left > right));

        Ok(())
    }

    fn comparison_greater_eq(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        self.stack.push(Type::Bool(left >= right));

        Ok(())
    }

    fn comparison_less(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        self.stack.push(Type::Bool(left < right));

        Ok(())
    }

    fn comparison_less_eq(&mut self, line_context: LineContext) -> Result<(), ()> {
        // It's safe to unwrap because the operations are generated by the interpreter and
        // verified elsewhere.
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();

        self.stack.push(Type::Bool(left <= right));

        Ok(())
    }

    fn jump_true(&mut self, loc: usize, line_context: LineContext) -> Result<(), ()> {
        let condition = self.stack.pop().unwrap();

        let mut result = false;

        match condition {
            Type::Int(v) => result = v != 0,
            Type::Uint(v) => result = v != 0,
            Type::Float(v) => result = v != 0.0,
            Type::Bool(v) => result = v,
            Type::Char(v) => result = v != 0,
            Type::String(v) => result = !v.is_empty(),
            Type::None => result = false,
        }

        if result {
            self.op_pointer = loc;
        }

        Ok(())
    }

    fn jump_false(&mut self, loc: usize, line_context: LineContext) -> Result<(), ()> {
        let condition = self.stack.pop().unwrap();

        let mut result = false;

        match condition {
            Type::Int(v) => result = v != 0,
            Type::Uint(v) => result = v != 0,
            Type::Float(v) => result = v != 0.0,
            Type::Bool(v) => result = v,
            Type::Char(v) => result = v != 0,
            Type::String(v) => result = !v.is_empty(),
            Type::None => result = false,
        }

        if !result {
            self.op_pointer = loc;
        }

        Ok(())
    }

    fn intrinsic_print(&mut self, line_context: LineContext) -> Result<(), ()> {
        let value = self.stack.pop().unwrap();

        match value {
            Type::Int(v) => println!("{}", v),
            Type::Uint(v) => println!("{}", v),
            Type::Float(v) => println!("{}", v),
            Type::Bool(v) => println!("{}", v),
            Type::Char(v) => println!("{}", v),
            Type::String(v) => println!("{}", v),
            Type::None => println!("None"),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_operations() {
        // TODO: Implement this test
    }

    #[test]
    fn test_binary_add() {
        simple_logger::init_with_level(log::Level::Trace);

        let mut inter = Interpreter {
            stack: Vec::new(),
            names: HashMap::new(),
            op_pointer: 0,
        };

        let operations: Vec<Operation> = vec![
            Operation::PushConst(Type::Int(5)),
            Operation::PushConst(Type::Int(10)),
            Operation::BinaryAdd,
        ];
        inter
            .eval_operations(operations, LineContext { line_number: 5 })
            .unwrap();
        assert_eq!(inter.stack.pop().unwrap(), Type::Int(15));

        inter.op_pointer = 0;
        let operations: Vec<Operation> = vec![
            Operation::PushConst(Type::Int(10)),
            Operation::PushConst(Type::Uint(15)),
            Operation::BinaryAdd,
        ];
        inter
            .eval_operations(operations, LineContext { line_number: 5 })
            .unwrap();
        assert_eq!(inter.stack.pop().unwrap(), Type::Uint(25));

        inter.op_pointer = 0;
        let operations: Vec<Operation> = vec![
            Operation::PushConst(Type::Int(2)),
            Operation::PushConst(Type::Float(3.5)),
            Operation::BinaryAdd,
        ];
        inter
            .eval_operations(operations, LineContext { line_number: 5 })
            .unwrap();
        assert_eq!(inter.stack.pop().unwrap(), Type::Float(5.5));

        inter.op_pointer = 0;
        let operations: Vec<Operation> = vec![
            Operation::PushConst(Type::Int(100)),
            Operation::PushConst(Type::Char('a' as u8)),
            Operation::BinaryAdd,
        ];
        inter
            .eval_operations(operations, LineContext { line_number: 5 })
            .unwrap();
        assert_eq!(inter.stack.pop().unwrap(), Type::Int(197));
    }
}
