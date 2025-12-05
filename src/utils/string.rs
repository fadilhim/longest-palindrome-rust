pub fn add_plural(word: &str, count: i32) -> String {
    if count > 1 {
        return format!("{}s", word);
    }
    word.to_owned()
}
