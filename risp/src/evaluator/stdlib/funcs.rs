use crate::parser::Object;

pub fn to_str(objs: &Vec<Object>) -> Result<Object, String> {
    match objs.len() {
        1 => to_str_obj(&objs[0]),
        _ => Err(format!("Expected 1 argument, found {}", objs.len())),
    }
}

fn to_str_obj(obj: &Object) -> Result<Object, String> {
    match obj {
        Object::String(_) => Ok(obj.clone()),
        Object::Integer(i) => Ok(Object::String(i.to_string())),
        _ => Err(format!("Invalid argument \'{}\', cannot convert to str", obj)),
    }
}

pub fn concat_str(objs: &Vec<Object>) -> Result<Object, String> {
    match objs.len() {
        0 => Err("Expected at least 1 argument, found 0".to_string()),
        _ => {
            let mut result = String::new();
            for obj in objs.iter() {
                match obj {
                    Object::String(s) => result.push_str(s),
                    _ => {
                        return Err(format!("Expected String, found {}", obj));
                    }
                }
            }
            Ok(Object::String(result))
        }
    }
}

pub fn list_take_first(objs: &Vec<Object>) -> Result<Object, String> {
    match objs.len() {
        1 => {
            match &objs[0] {
                Object::List(list) => {
                    if list.len() == 0 {
                        return Err("Cannot take first element of empty list".to_string());
                    }
                    Ok(list[0].clone())
                }
                _ => Err(format!("Expected List, found {}", objs[0])),
            }
        }
        _ => Err(format!("Expected 1 argument, found {}", objs.len())),
    }
}

pub fn print_ln(obj: &Vec<Object>) -> Result<Object, String> {
    for obj in obj.iter() {
        print!("{}", obj);
    }
    Ok(Object::Void)
}
