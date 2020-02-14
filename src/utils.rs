
#[macro_export]
macro_rules! anime {
    ($($n :expr), *) => {
        {
            $(
                print!("{}", $n);
            )*
            println!("");
        }
    }
}