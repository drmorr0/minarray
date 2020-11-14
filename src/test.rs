use super::*;

#[test]
fn push_element() {
    let mut array: MinArray<u32> = MinArray::new();
    array.push(7, 24).expect("the array should not be full");
    assert_eq!(array.min, 7);
    assert_eq!(array.len, 1);
}

#[test]
fn take_less_than_empty() {
    let mut array: MinArray<u32> = MinArray::new();
    for (key, _) in array.take_less_than(10) {
        assert!(key < 10);
    }
    assert_eq!(array.min, 0xffffffff);
    assert_eq!(array.len, 0);
}

#[test]
fn take_less_than_one_element() {
    let mut array: MinArray<u32> = MinArray::new();
    array.push(7, 24).expect("the array should not be full");
    for (key, _) in array.take_less_than(10) {
        assert!(key < 10);
    }
    assert_eq!(array.min, 0xffffffff);
    assert_eq!(array.len, 0);
}

#[test]
fn take_less_than_many_elements() {
    let mut array: MinArray<u32> = MinArray::new();
    array.push(7, 24).expect("the array should not be full");
    array.push(19, 24).expect("the array should not be full");
    array.push(9, 24).expect("the array should not be full");
    array.push(6, 24).expect("the array should not be full");
    for (key, _) in array.take_less_than(10) {
        assert!(key < 10);
    }
    assert_eq!(array.min, 19);
    assert_eq!(array.len, 1);
}

#[test]
fn interleave_push_and_take() {
    let mut array: MinArray<u32> = MinArray::new();
    array.push(7, 24).expect("the array should not be full");
    array.push(19, 24).expect("the array should not be full");
    for (key, _) in array.take_less_than(10) {
        assert!(key < 10);
    }
    array.push(9, 24).expect("the array should not be full");
    array.push(6, 24).expect("the array should not be full");
    for (key, _) in array.take_less_than(7) {
        assert!(key < 10);
    }
    assert_eq!(array.min, 9);
    assert_eq!(array.len, 2);
}
