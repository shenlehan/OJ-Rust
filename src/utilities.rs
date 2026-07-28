pub fn elem_in_arr<T: std::cmp::PartialEq>(elem: &T, arr: &Vec<T>) -> bool {
    for t in arr {
        if *t == *elem {
            return true;
        }
    }
    false
}

pub fn normalize(s: &str) -> Vec<String> {
    let mut lines: Vec<String> = s
        .lines()
        .map(|line| line.trim_end().to_string())
        .collect();

    while matches!(lines.last(), Some(line) if line.is_empty()) {
        lines.pop();
    }

    lines
}

pub fn standard_equal(output: &str, answer: &str) -> bool {
    normalize(output) == normalize(answer)
}
