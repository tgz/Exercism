#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }

    if first_in_second(_first_list, _second_list) {
        return Comparison::Sublist;
    }

    if first_in_second(_second_list, _first_list) {
        return Comparison::Superlist;
    }
    
    Comparison::Unequal
}

fn first_in_second<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    if _first_list.is_empty() {
        return true;
    } else if _first_list.len() > _second_list.len() {
        return false;
    }
    for (i, _x) in _second_list.iter().enumerate() {
        if _first_list.len() > _second_list.len()- i {
            return false;
        }
        if begins_width(_first_list, &_second_list[i..]) {
            return true;
        }
    }
    false
}

fn begins_width<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    for(i, _x) in _first_list.iter().enumerate() {
        if _first_list[i] != _second_list[i] {
            return false;
        }
    }
    true
}
