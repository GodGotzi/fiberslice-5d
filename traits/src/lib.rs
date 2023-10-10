pub trait TypeEq<T> {
    fn type_eq(&self, other: T) -> bool;
}
