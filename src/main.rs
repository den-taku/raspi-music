use ears::{AudioController, Sound};

fn main() {
    let mut sound = Sound::new("./samples/sample.m4a").unwrap();
    loop {
        sound.play();
        while sound.is_playing() {}
    }
}
