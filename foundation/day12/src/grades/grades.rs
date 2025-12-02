pub fn curve_scores(scores: &[u32], bonus: i32) -> Vec<u32> {
    scores
        .iter()
        .map(|x| {
            let sum = *x as i32 + bonus;
            let clamped = sum.clamp(0, 100);
            clamped as u32
        })
        .collect::<Vec<u32>>()
}

pub fn passing(scores: &[u32]) -> Vec<u32> {
    scores
        .iter()
        .copied()
        .filter(|x| *x >= 60)
        .collect::<Vec<u32>>()
}

pub fn first_failing(scores: &[u32]) -> Option<u32> {
    scores.iter().copied().find(|x| *x < 60)
}

pub fn window(scores: &[u32], start: usize, len: usize) -> Option<&[u32]> {
    if start > scores.len() {
        return None;
    }

    if start + len > scores.len() {
        return None;
    }

    if len == 0 {
        return Some(&scores[start..start]);
    }

    scores.get(start..start + len)
}

pub fn clamp_in_place(scores: &mut [u32]) {
    scores.iter_mut().for_each(|x| *x = (*x).clamp(0, 100));
}

pub fn average(scores: &[u32]) -> Option<f64> {
    if scores.is_empty() {
        return None;
    }

    let sum = scores.iter().copied().map(|x| x as f64).sum::<f64>();

    Some(sum / scores.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn curve_scores_happy_path() {
        assert_eq!(curve_scores(&[66, 15, 32], 10), vec![76, 25, 42]);
    }

    #[test]
    fn curve_scores_bigger_than_100() {
        assert_eq!(curve_scores(&[90, 85, 32], 12), vec![100, 97, 44]);
    }

    #[test]
    fn curve_scores_smaller_than_0() {
        assert_eq!(curve_scores(&[66, 9, 32], -10), vec![56, 0, 22]);
    }

    #[test]
    fn curve_scores_input_empty() {
        assert_eq!(curve_scores(&[], -10), vec![]);
    }

    #[test]
    fn passing_filters_below_60() {
        assert_eq!(passing(&vec![32]), vec![]);
    }

    #[test]
    fn passing_empty_input() {
        assert_eq!(passing(&vec![]), vec![]);
    }

    #[test]
    fn passing_all_fail() {
        assert_eq!(passing(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), vec![]);
    }

    #[test]
    fn first_failing_none_when_all_passing() {
        assert_eq!(first_failing(&vec![90, 86, 66, 134]), None);
    }

    #[test]
    fn first_failing_works_for_mixed() {
        assert_eq!(first_failing(&vec![90, 3, 90]), Some(3));
    }

    #[test]
    fn first_failing_empty_input() {
        assert_eq!(first_failing(&[]), None)
    }

    #[test]
    fn window_start_plus_len_bigger_than_scores_len() {
        assert_eq!(window(&vec![1, 2, 3], 3, 1), None);
        assert_eq!(window(&vec![1, 2, 3], 2, 5), None);
    }

    #[test]
    fn window_len_eq_zero() {
        assert_eq!(window(&vec![1, 2, 3], 1, 0), Some(&[][..]));
    }

    #[test]
    fn window_scores_is_empty() {
        assert_eq!(window(&vec![], 0, 0), Some(&[][..]));
        assert_eq!(window(&vec![], 0, 1), None);
        assert_eq!(window(&vec![], 1, 0), None);
    }

    #[test]
    fn window_happy_path() {
        assert_eq!(window(&vec![1, 2, 3], 0, 1), Some(&[1][..]));
    }

    #[test]
    fn clamp_in_place_happy_path() {
        let mut xs = vec![195, 166, 90];
        clamp_in_place(&mut xs);
        assert_eq!(xs, &[100, 100, 90]);
    }

    #[test]
    fn clamp_in_place_scores_is_empty() {
        let mut xs = vec![];
        clamp_in_place(&mut xs);
        assert_eq!(xs, &[]);
    }

    #[test]
    fn clamp_in_place_allready_path() {
        let mut xs = vec![100, 80, 90];
        clamp_in_place(&mut xs);
        assert_eq!(xs, &[100, 80, 90]);
    }

    #[test]
    fn average_empty_returns_none() {
        let xs = vec![];
        assert_eq!(average(&xs), None);
    }

    #[test]
    fn average_single_element() {
        assert_eq!(average(&[1]), Some(1f64))
    }

    #[test]
    fn average_two_elements() {
        assert_eq!(average(&[1, 2]), Some(1.5))
    }
}
