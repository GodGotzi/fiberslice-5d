use macros::NumEnum;

#[cfg(test)]
#[derive(Debug, NumEnum, PartialEq, Eq)]
enum TestEnum {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[test]
fn test_num_enum_macro() {
    assert_eq!(TestEnum::from(0), TestEnum::A);
    assert_eq!(TestEnum::from(1), TestEnum::B);
    assert_eq!(TestEnum::from(2), TestEnum::C);
    assert_eq!(TestEnum::from(3), TestEnum::D);
    assert_eq!(TestEnum::from(4), TestEnum::E);
}
