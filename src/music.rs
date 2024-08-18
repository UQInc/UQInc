
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::path::PathBuf;

// Play theme music
pub fn music() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut switch = PathBuf::from("media");
    switch.push("sounds");
    switch.push("theme.mp3");

    // Get audio source
    let file = File::open(switch).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    // Turn down volume of source
    let lower_vol_source = source.amplify(0.2);

    // Loop source
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

// Play sound effect at given path for given length
pub fn sound_effect(file_path: PathBuf, len: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open(file_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    stream_handle.play_raw(source.convert_samples()).unwrap();
    std::thread::sleep(Duration::from_secs(len));
}
