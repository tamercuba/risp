use std::{ cell::RefCell, rc::Rc };

use crate::{ env::Env, parser::{ parse, Object } };

pub struct Evaluator {
    env: Rc<RefCell<Env>>,
}
impl Evaluator {
    pub fn new(env: Rc<RefCell<Env>>) -> Self {
        Evaluator { env }
    }

    pub fn eval(&mut self, statement: &str) -> Result<Object, String> {
        let parsed_list = parse(statement);
        return match parsed_list {
            Ok(_) => self.eval_obj(&parsed_list.unwrap()),
            Err(_) => Err(format!("{}", parsed_list.err().unwrap())),
        };
    }

    fn eval_obj(&mut self, obj: &Object) -> Result<Object, String> {
        match obj {
            Object::List(list) => self.eval_list(list),
            Object::Void => Ok(Object::Void),
            Object::Lambda(_params, _body) => Ok(Object::Void),
            Object::Bool(_) => Ok(obj.clone()),
            Object::Integer(n) => Ok(Object::Integer(*n)),
            Object::Symbol(s) => self.eval_symbol(s),
        }
    }

    fn eval_list(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        let head = &list[0];
        match head {
            Object::Symbol(s) =>
                match s.as_str() {
                    // TODO: Add more binary operators
                    "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => {
                        return self.eval_binary_op(&list);
                    }
                    "define" => self.eval_define(&list),
                    "if" => self.eval_if(&list),
                    "lambda" => self.eval_lambda(&list),
                    // TODO: Add a basic std lib
                    _ => self.eval_func_call(&s, &list),
                }
            _ => {
                let mut new_list = Vec::new();
                for obj in list {
                    let result = self.eval_obj(obj)?;
                    match result {
                        Object::Void => {}
                        _ => new_list.push(result),
                    }
                }
                Ok(Object::List(new_list))
            }
        }
    }

    fn eval_symbol(&mut self, symbol: &str) -> Result<Object, String> {
        let val_opt = self.env.borrow_mut().get(symbol);
        match val_opt {
            Some(val) => Ok(val.clone()),
            None => Err(format!("Unbound symbol: {}", symbol)),
        }
    }

    fn eval_binary_op(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        if list.len() != 3 {
            return Err(format!("Invalid number of arguments for binary operation"));
        }

        let operator = list[0].clone();
        let left = self.eval_obj(&list[1].clone())?;
        let right = self.eval_obj(&list[2].clone())?;
        let left_val = match left {
            Object::Integer(n) => n,
            _ => {
                return Err(format!("Left operand must be an integer {:?}", left));
            }
        };
        let right_val = match right {
            Object::Integer(n) => n,
            _ => {
                return Err(format!("Right operand must be an integer {:?}", right));
            }
        };

        match operator {
            Object::Symbol(s) =>
                match s.as_str() {
                    "+" => Ok(Object::Integer(left_val + right_val)),
                    "-" => Ok(Object::Integer(left_val - right_val)),
                    "*" => Ok(Object::Integer(left_val * right_val)),
                    "/" => {
                        match right_val {
                            0 => {
                                return Err(format!("Division by zero"));
                            }
                            _ => Ok(Object::Integer(left_val / right_val)),
                        }
                    }
                    "<" => Ok(Object::Bool(left_val < right_val)),
                    ">" => Ok(Object::Bool(left_val > right_val)),
                    "=" => Ok(Object::Bool(left_val == right_val)),
                    "!=" => Ok(Object::Bool(left_val != right_val)),
                    _ => Err(format!("Invalid infix operator: {}", s)),
                }
            _ => Err(format!("Operator must be a symbol")),
        }
    }

    fn eval_define(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        if list.len() != 3 {
            return Err(format!("Invalid number of arguments for define"));
        }

        let sym = match &list[1] {
            Object::Symbol(s) => s.clone(),
            _ => {
                return Err(format!("Invalid define"));
            }
        };
        let val = self.eval_obj(&list[2])?;
        self.env.borrow_mut().set(sym.as_str(), val);
        Ok(Object::Void)
    }

    fn eval_if(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        if list.len() != 4 {
            return Err(format!("Invalid number of arguments for if"));
        }

        let cond_obj = self.eval_obj(&list[1])?;
        let cond = match cond_obj {
            Object::Bool(b) => b,
            _ => {
                return Err(format!("Invalid condition"));
            }
        };

        let return_idx = if cond == true { 2 } else { 3 };
        return self.eval_obj(&list[return_idx]);
    }

    fn eval_lambda(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        let params = match &list[1] {
            Object::List(list) => {
                let mut params = Vec::new();
                for param in list {
                    match param {
                        Object::Symbol(s) => params.push(s.clone()),
                        _ => {
                            return Err(format!("Invalid lambda parameter"));
                        }
                    }
                }
                params
            }
            _ => {
                return Err(format!("Invalid lambda"));
            }
        };

        let body = match &list[2] {
            Object::List(list) => list.clone(),
            _ => {
                return Err(format!("Invalid lambda"));
            }
        };
        Ok(Object::Lambda(params, body))
    }

    fn eval_func_call(&mut self, func_name: &str, list: &Vec<Object>) -> Result<Object, String> {
        let lambda = self.env.borrow_mut().get(func_name);
        if lambda.is_none() {
            return Err(format!("Function not found: {}", func_name));
        }

        let func = lambda.unwrap();
        match func {
            Object::Lambda(params, body) => {
                self.env = Env::new_scope(self.env.clone());
                for (i, param) in params.iter().enumerate() {
                    let val = self.eval_obj(&list[i + 1])?;
                    self.env.borrow_mut().set(param, val);
                }
                let result = self.eval_obj(&Object::List(body));
                self.env.borrow_mut().remove_scope();
                return result;
            }
            _ => {
                return Err(format!("Not a lambda: {}", func_name));
            }
        }
    }
}
