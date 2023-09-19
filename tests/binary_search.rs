use binary_search_s::*;

#[test]
fn test_binary_search_s() {
    let b: [i32; 0] = [];
    assert_eq!(b.binary_search_s(&5), Err(0));

    let b = [4];
    assert_eq!(b.binary_search_s(&3), Err(0));
    assert_eq!(b.binary_search_s(&4), Ok(0));
    assert_eq!(b.binary_search_s(&5), Err(1));

    let b = [1, 2, 4, 6, 8, 9];
    assert_eq!(b.binary_search_s(&5), Err(3));
    assert_eq!(b.binary_search_s(&6), Ok(3));
    assert_eq!(b.binary_search_s(&7), Err(4));
    assert_eq!(b.binary_search_s(&8), Ok(4));

    let b = [1, 2, 4, 5, 6, 8];
    assert_eq!(b.binary_search_s(&9), Err(6));

    let b = [1, 2, 4, 6, 7, 8, 9];
    assert_eq!(b.binary_search_s(&6), Ok(3));
    assert_eq!(b.binary_search_s(&5), Err(3));
    assert_eq!(b.binary_search_s(&8), Ok(5));

    let b = [1, 2, 4, 5, 6, 8, 9];
    assert_eq!(b.binary_search_s(&7), Err(5));
    assert_eq!(b.binary_search_s(&0), Err(0));

    let b = [1, 3, 3, 3, 7];
    assert_eq!(b.binary_search_s(&0), Err(0));
    assert_eq!(b.binary_search_s(&1), Ok(0));
    assert_eq!(b.binary_search_s(&2), Err(1));
    assert!(match b.binary_search_s(&3) {
        Ok(1..=3) => true,
        _ => false,
    });
    assert!(match b.binary_search_s(&3) {
        Ok(1..=3) => true,
        _ => false,
    });
    assert_eq!(b.binary_search_s(&4), Err(4));
    assert_eq!(b.binary_search_s(&5), Err(4));
    assert_eq!(b.binary_search_s(&6), Err(4));
    assert_eq!(b.binary_search_s(&7), Ok(4));
    assert_eq!(b.binary_search_s(&8), Err(5));

    let b = [(); usize::MAX];
    assert_eq!(b.binary_search_s(&()), Ok(usize::MAX / 2));
}

// #[test]
// fn test_binary_search_by_overflow() {
//     let b = [(); usize::MAX];
//     assert_eq!(b.binary_search_by(|_| Ordering::Equal), Ok(usize::MAX / 2));
//     assert_eq!(b.binary_search_by(|_| Ordering::Greater), Err(0));
//     assert_eq!(b.binary_search_by(|_| Ordering::Less), Err(usize::MAX));
// }

#[test]
// Test implementation specific behavior when finding equivalent elements.
// It is ok to break this test but when you do a crater run is highly advisable.
fn test_binary_search_implementation_details() {
    let b = [1, 1, 2, 2, 3, 3, 3];
    assert_eq!(b.binary_search_s(&1), Ok(1));
    assert_eq!(b.binary_search_s(&2), Ok(3));
    assert_eq!(b.binary_search_s(&3), Ok(5));
    let b = [1, 1, 1, 1, 1, 3, 3, 3, 3];
    assert_eq!(b.binary_search_s(&1), Ok(4));
    assert_eq!(b.binary_search_s(&3), Ok(7));
    let b = [1, 1, 1, 1, 3, 3, 3, 3, 3];
    assert_eq!(b.binary_search_s(&1), Ok(2));
    assert_eq!(b.binary_search_s(&3), Ok(4));
}