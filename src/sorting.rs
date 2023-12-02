
/// Returns the indices that would sort a vector.
pub fn argsort<T: Ord>(v: &[T]) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..v.len()).collect();
    indices.sort_by(|&i, &j| v[i].cmp(&v[j]));
    indices
}