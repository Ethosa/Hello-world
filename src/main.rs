
fn main() {
    let mut anime: i128 = 10;
    anime = {
        let mut ban: i128 = 2;
        for _i in 0..30 {
            ban *= &anime;
        }
        ban
    };
    println!("{}", anime);
}
