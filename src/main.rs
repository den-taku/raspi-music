use ears::{AudioController, Sound};

fn main() {
    let mut sound = Sound::new("./samples/sample.wav").unwrap();
    loop {
        sound.play();

        while sound.is_playing() {}
    }
}
