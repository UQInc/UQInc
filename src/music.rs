use rodio::{source::Source, Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;
use std::time::Duration;
use std::path::PathBuf;
use std::io::BufReader;
use std::fs::File;

// Play theme music
pub fn music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Assuming you've loaded the theme music as Vec<u8> using rust-embed
    let theme_music = include_bytes!("media/sounds/theme.mp3");
    
    // Create a cursor from the byte slice
    let cursor = Cursor::new(theme_music);
    
    // Decode the audio from the cursor
    let source = Decoder::new(cursor).unwrap();

    // Turn down the volume of the source
    let lower_vol_source = source.amplify(0.2);

    // Loop the source
    let repeating_source = lower_vol_source.repeat_infinite();

    // Play loop
    stream_handle
        .play_raw(repeating_source.convert_samples())
        .unwrap();

    // Keep the thread alive to prevent the audio from stopping
    loop {
        std::thread::sleep(Duration::from_secs(80));
    }
}

// Play sound effect from bytes for the given length
pub fn sound_effect_from_bytes(sound_data: Vec<u8>, len: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Keep the thread alive to let the sound effect play for the specified length
    std::thread::sleep(Duration::from_secs(len));
}

// Play sound effect at given path for given length (if you still need this function)
pub fn sound_effect(file_path: PathBuf, len: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_secs(len));
}

