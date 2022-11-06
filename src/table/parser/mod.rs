///
/// contentの文字列を分割する
pub fn line_format(content: &String) -> Vec<&str> {
    return content
        .split(' ')
        .filter_map(|c| {
            let trim = c.trim();
            if trim.is_empty() {
                None
            } else {
                Some(trim)
            }
        })
        .collect();
}
