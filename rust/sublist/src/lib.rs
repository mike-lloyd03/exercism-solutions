#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else if check_sublist(_first_list, _second_list) {
        Comparison::Sublist
    } else if check_sublist(_second_list, _first_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

/// Checks if l1 is a sublist of l2
fn check_sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> bool {
    if l1.len() > l2.len() {
        return false;
    }

    if l1.is_empty() {
        return true;
    }
    for i in 0..=l2.len() - l1.len() {
        if l1 == &l2[i..i + l1.len()] {
            return true;
        }
    }
    false
}
