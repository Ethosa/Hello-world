
#[derive(Debug)]
struct Anime {
    title: String,
    episode: u16,
    episodes: u16
}

impl Anime {
    fn drop(&mut self) {
        self.title = self.title.clone() + &" - dropped";
        self.episode = 0;
    }
}

fn main() {
    let mut ya = Anime { title: "Boku no Pico".to_string(), episode: 10000, episodes: 4 };

    println!("{:?}", ya);

    ya.drop();

    println!("{:?}", ya);
}
