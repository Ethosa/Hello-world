mod utils {
    pub fn stss(s: String) {}
}

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}