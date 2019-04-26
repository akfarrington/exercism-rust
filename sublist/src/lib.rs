/*
At the very beginning of this file I need to admit defeat. I couldn't figure this one out
until I cheated and looked at the example.rs file in the github repository. The solution there
was perfect. I waited about a week to rewrite that example from memory, and this is the result.
Did I succeed? No. Did I learn something? Yes. And isn't that what I'm here for?
*/

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
    } else if check_sub(_first_list, _second_list) == true {
        Comparison::Superlist
    } else if check_sub(_second_list, _first_list) == true {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

// checks if list_1 is a superlist of list_2
fn check_sub<T: PartialEq>(list_1: &[T], list_2: &[T]) -> bool {
    if list_1.len() < list_2.len(){
        return false;
    } 
    if list_1.starts_with(list_2) {
        return true;
    }

    check_sub(&list_1[1..], &list_2)
}