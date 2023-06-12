pub trait TypeEq {
    fn type_eq(&self, other: dyn TypeEq) -> bool;
}
