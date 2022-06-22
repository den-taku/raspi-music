use ears::{AudioController, Music};

fn main() {
    let mut music = Music::new("./samples/sample.m4a").unwrap();
    loop {
        let tags = music.get_tags();
        println!("{}", tags);
        music.play();
    }
}
