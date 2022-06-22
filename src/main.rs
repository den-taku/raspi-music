use ears::{Sound, AudioController};

fn main() {
    let mut sound = Sound::new("./sample/sample.wav").unwrap();
    sound.play();

    while sound.is_playing() {}
}
