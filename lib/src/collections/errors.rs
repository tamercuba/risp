#[derive(Debug)]
pub enum CollectionError {
    IndexOutOfBounds { value: usize },
}
