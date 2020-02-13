use rand::Rng;

fn main() {
    let mut matrix = vec![];
    let height = 10;
    let width = 10;

    for _ in 0..width {
        let mut mx = vec![];
        for _ in 0..height {
            mx.push(rand::thread_rng().gen_range(0, 9));
        }
        matrix.push(mx);
    }

    for i in matrix.iter() {
        println!("{:?}", i);
    }
}
