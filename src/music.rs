use rodio::{Decoder, OutputStream, source::Source, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

// Play theme music
pub fn music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Get audio source
    let file = File::open("src\\media\\sounds\\theme.mp3").unwrap();  // Replace with your audio file path
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Create a looping audio source
    let repeating_source = source.repeat_infinite();

    // Play the looping audio in the background
    stream_handle.play_raw(repeating_source.convert_samples()).unwrap();

    // Keep the thread alive to prevent the audio from stopping
    loop {
        std::thread::sleep(Duration::from_secs(10));
    }
}

// Play sounds effect at given path for given lengt
pub fn sound_effect(filePath: &str, _len: i32 ) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = File::open(filePath).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_secs(5));
}