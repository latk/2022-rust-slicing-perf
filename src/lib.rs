use rayon::prelude::*;

#[inline]
pub fn splice(channels: usize, data: &[u8]) -> Vec<Vec<u8>> {
    assert!(channels > 0);
    let each_len = data.len() / channels + if data.len() % channels == 0 { 0 } else { 1 };
    let mut out = vec![Vec::with_capacity(each_len); channels];
    for (i, d) in data.iter().copied().enumerate() {
        out[i % channels].push(d);
    }
    out
}

#[inline]
pub fn splice_stepped(channels: usize, data: &[u8]) -> Vec<Vec<u8>> {
    assert!(channels > 0);
    (0..channels)
        .map(|offset| {
            data.iter()
                .copied()
                .skip(offset)
                .step_by(channels)
                .collect()
        })
        .collect()
}

#[inline]
pub fn splice_parallel(channels: usize, data: &[u8]) -> Vec<Vec<u8>> {
    assert!(channels > 0);
    (0..channels)
        .map(|offset| {
            data.par_iter()
                .copied()
                .skip(offset)
                .step_by(channels)
                .collect()
        })
        .collect()
}

#[cfg(test)]
fn check_splicer(f: impl Fn(usize, &[u8]) -> Vec<Vec<u8>>) {
    let input = [0, 1, 2, 3, 4, 5, 6, 7];
    let expected = [
        vec![], // unused
        vec![vec![0, 1, 2, 3, 4, 5, 6, 7]],
        vec![vec![0, 2, 4, 6], vec![1, 3, 5, 7]],
        vec![vec![0, 3, 6], vec![1, 4, 7], vec![2, 5]],
        vec![vec![0, 4], vec![1, 5], vec![2, 6], vec![3, 7]],
        vec![vec![0, 5], vec![1, 6], vec![2, 7], vec![3], vec![4]],
        vec![vec![0, 6], vec![1, 7], vec![2], vec![3], vec![4], vec![5]],
    ];
    for (channels, expected) in expected.into_iter().enumerate().skip(1) {
        assert_eq!(f(channels, &input), expected, "f({channels}, ...)");
    }
}

#[test]
fn test_splice() {
    check_splicer(splice);
}

#[test]
fn test_splice_stepped() {
    check_splicer(splice_stepped);
}

#[test]
fn test_splice_parallel() {
    check_splicer(splice_parallel);
}
