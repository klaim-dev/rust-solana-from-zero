pub fn build_vec_no_cap(n: usize) -> Vec<u64> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

pub fn build_vec_with_cap(n: usize) -> Vec<u64> {
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        v.push(i as u64);
    }
    v
}

pub fn build_string_no_cap(repeats: usize, chunk: &str) -> String {
    let mut s = String::new();
    for _ in 0..repeats {
        s.push_str(chunk);
    }
    s
}

pub fn build_string_with_cap(repeats: usize, chunk: &str) -> String {
    let mut s = String::with_capacity(repeats * chunk.len());
    for _ in 0..repeats {
        s.push_str(chunk);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_vec_with_cap_len_matches() {
        let v = build_vec_with_cap(10);
        assert_eq!(v.len(), 10);
    }

    #[test]
    fn build_vec_no_cap_len_matches() {
        let v = build_vec_no_cap(10);
        assert_eq!(v.len(), 10);
    }
}
