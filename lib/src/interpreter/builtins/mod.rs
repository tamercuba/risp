use super::value::Value;

mod data_structures;
mod math;
mod sequences;
mod stdio;
#[cfg(test)]
mod test_data_structures;
#[cfg(test)]
mod test_hof;
#[cfg(test)]
mod test_math;
#[cfg(test)]
mod test_sequences;

pub fn builtins() -> Vec<(&'static str, Value)> {
    math::builtins()
        .into_iter()
        .chain(stdio::builtins())
        .chain(data_structures::builtins())
        .chain(sequences::builtins())
        .collect()
}
