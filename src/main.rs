use std::path::Path;
use whisper_rs;

fn main() {
    let jfk_audio = Path::new("./audio/jfk.wav");
    let mut reader = hound::WavReader::open(jfk_audio).expect("failed to open file");
    println!("Wave spec: {:#?}", reader.spec());
    // Convert the audio to floating point samples.
    let mut audio = whisper_rs::convert_integer_to_float_audio(
        &reader
            .samples::<i16>()
            .map(|s| s.expect("invalid sample"))
            .collect::<Vec<_>>(),
    );
}
