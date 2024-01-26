#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Rectangle<T:PartialOrd+PartialEq>(T, T, T, T);