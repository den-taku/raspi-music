use ears::{Sound, AudioController};

fn main() {
    let mut sound = Sound::new("./samples/sample.wav").unwrap();
    sound.play();

    while sound.is_playing() {}
}
