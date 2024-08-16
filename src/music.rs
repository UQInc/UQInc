use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let file = File::open("C:\\Users\\Bangu\\OneDrive\\Documents\\Hackathon\\UQInc\\src\\theme.mp3").unwrap();  // Replace with your audio file path
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
