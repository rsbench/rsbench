pub fn trim_string(s: &str, leading: usize, trailing: usize) -> &str {
    let start = leading;
    let end = s.len().saturating_sub(trailing);
    &s[start..end]
}
