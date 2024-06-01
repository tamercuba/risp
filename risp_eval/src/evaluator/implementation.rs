use std::{ cell::RefCell, rc::Rc };
use super::{ stdlib, SysCallWrapper };

use crate::{ env::Env, lexer::Token, parser::Object };

pub struct Evaluator {
    env: Rc<RefCell<Env>>,
}
impl Evaluator {
    pub fn new(with_stdlib: bool) -> Self {
        let env = Env::new();
        let ev = Evaluator { env };
        if with_stdlib {
            ev.add_stdlib();
        }
        return ev;
    }

    fn add_stdlib(&self) {
        self.env
            .borrow_mut()
            .set("str", Object::SysCall(SysCallWrapper::new("str", stdlib::to_str)));
        self.env
            .borrow_mut()
            .set(
                "concatenate",
                Object::SysCall(SysCallWrapper::new("concatenate", stdlib::concat_str))
            );
        self.env
            .borrow_mut()
            .set("first", Object::SysCall(SysCallWrapper::new("first", stdlib::list_take_first)))
    }

    pub fn eval(&mut self, statement: &str) -> Result<Object, String> {
        let tokens = Token::tokenize(statement).map_err(|e| format!("{}", e))?;
        let parsed_list = Object::from_tokens(tokens);
        return match parsed_list {
            Ok(_) => self.eval_obj(&parsed_list.unwrap()),
            Err(_) => Err(format!("{}", parsed_list.err().unwrap())),
        };
    }

    fn eval_obj(&mut self, obj: &Object) -> Result<Object, String> {
        match obj {
            Object::List(list) => {
                let result = self.eval_list(list);
                match result {
                    Ok(evalued_list) => {
                        match evalued_list {
                            Object::List(result_list) => {
                                return self.eval_list(&result_list);
                            }
                            _ => {
                                return Ok(evalued_list);
                            }
                        }
                    }
                    Err(_) => result,
                }
            }
            Object::Void => Ok(Object::Void),
            Object::String(s) => Ok(Object::String(s.clone())),
            Object::Function(_args, _dec) => Ok(Object::Void),
            Object::Lambda(params, body) => Ok(Object::Lambda(params.clone(), body.clone())),
            Object::Bool(_) => Ok(obj.clone()),
            Object::Integer(n) => Ok(Object::Integer(*n)),
            Object::Symbol(s) => self.eval_symbol(s),
            _ => { Err(format!("Invalid object: {:?}", obj)) }
        }
    }

    fn eval_function(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        if list.len() != 4 {
            return Err(format!("Invalid number of arguments for defun"));
        }
        match &list[1] {
            Object::Symbol(func_name) => {
                let params = match &list[2] {
                    Object::List(list) => {
                        let mut params = Vec::new();
                        for param in list {
                            match param {
                                Object::Symbol(s) => params.push(s.clone()),
                                _ => {
                                    return Err(format!("Invalid defun parameter"));
                                }
                            }
                        }
                        params
                    }
                    _ => {
                        return Err(format!("Invalid defun"));
                    }
                };

                let body = match &list[3] {
                    Object::List(list) => list.clone(),
                    _ => {
                        return Err(format!("Invalid defun"));
                    }
                };
                let f = Object::Function(params, body);
                self.env.borrow_mut().set(func_name.as_str(), f);
                Ok(Object::Void)
            }
            _ => {
                return Err(format!("Invalid defun"));
            }
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
                    "let" => self.eval_let(&list),
                    "defun" => self.eval_function(&list),
                    "if" => self.eval_if(&list),
                    "lambda" => self.eval_lambda(&list),
                    "true" | "false" => Ok(Object::Bool(s == "true")),
                    // TODO: Add a basic std lib
                    _ => self.eval_func_call(&s, &list),
                }
            Object::Lambda(params, body) => self.eval_anon_func_call(params, body, &list),
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
        match symbol {
            "true" => {
                return Ok(Object::Bool(true));
            }
            "false" => {
                return Ok(Object::Bool(false));
            }
            _ => {}
        }
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

    fn eval_let(&mut self, list: &Vec<Object>) -> Result<Object, String> {
        if list.len() != 3 {
            return Err(format!("Invalid number of arguments for let"));
        }

        let sym = match &list[1] {
            Object::Symbol(s) => s.clone(),
            _ => {
                return Err(format!("Invalid let"));
            }
        };
        let val = self.eval_obj(&list[2])?;
        match val {
            Object::Lambda(params, body) => {
                self.env.borrow_mut().set(sym.as_str(), Object::Function(params, body));
                return Ok(Object::Void);
            }
            _ => {}
        }

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
        let result = Object::Lambda(params, body);
        Ok(result)
    }

    fn eval_func_call(&mut self, func_name: &str, list: &Vec<Object>) -> Result<Object, String> {
        let func_result = self.env.borrow_mut().get(func_name);
        if func_result.is_none() {
            return Err(format!("Function not defined: {}", func_name));
        }

        let func = func_result.unwrap();
        match func {
            Object::Function(params, body) => {
                self.env = Env::new_scope(self.env.clone());
                for (i, param) in params.iter().enumerate() {
                    let val = self.eval_obj(&list[i + 1])?;
                    self.env.borrow_mut().set(param.as_str(), val);
                }
                let result = self.eval_obj(&Object::List(body));
                self.env.borrow_mut().remove_scope();
                return result;
            }
            Object::SysCall(sys_call) => {
                let mut args = Vec::new();
                for arg in list.iter().skip(1) {
                    let val = self.eval_obj(arg)?;
                    args.push(val.clone());
                }
                return sys_call.run(&args);
            }
            _ => {
                return Err(format!("Not a function: {}", func_name));
            }
        }
    }

    fn eval_anon_func_call(
        &mut self,
        params: &Vec<String>,
        body: &Vec<Object>,
        list: &Vec<Object>
    ) -> Result<Object, String> {
        if params.len() != list.len() - 1 {
            return Err(format!("Invalid number of arguments for lambda"));
        }

        self.env = Env::new_scope(self.env.clone());
        for (i, param) in params.iter().enumerate() {
            let val = self.eval_obj(&list[i + 1])?;
            self.env.borrow_mut().set(param, val);
        }
        let result = self.eval_obj(&Object::List(body.clone()));
        self.env.borrow_mut().remove_scope();
        return result;
    }
}
