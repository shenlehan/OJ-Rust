pub fn elem_in_arr<T: std::cmp::PartialEq>(elem: &T, arr: &Vec<T>) -> bool {
    for t in arr {
        if *t == *elem {
            return true;
        }
    }
    false
}