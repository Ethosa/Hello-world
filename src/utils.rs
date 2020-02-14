
#[allow(dead_code)]
mod utils {
    pub fn string_to_str() {}
}

#[allow(dead_code)]
pub fn string_to_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[macro_export]
macro_rules! anime {
    ( $( $n :expr ),* ) => {
        {
            $(
                print!("{} ", $n);
            )*
            println!("");
        }
    }
}