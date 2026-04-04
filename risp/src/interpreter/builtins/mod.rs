use super::value::Value;

mod data_structures;
mod math;
mod stdio;
#[cfg(test)]
mod test_math;

pub fn builtins() -> Vec<(&'static str, Value)> {
    math::math_builtins()
        .into_iter()
        .chain(stdio::stdio_builtins())
        .chain(data_structures::ds_builtins())
        .collect()
}
