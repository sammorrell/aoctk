/// A function that returns the indices of all instances of a substring in a string.
#[inline]
pub fn find_instances_of(haystack: &str, needle: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut start = 0;
    while let Some(i) = haystack[start..].find(needle) {
        start += i + needle.len();
        indices.push(start - needle.len());
    }
    indices
}