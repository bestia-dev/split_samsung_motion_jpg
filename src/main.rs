// split_samsung_motion_jpg/src/main.rs

#![doc=include_str!("../README.md")]

// End of picture signature - not JPEG footer but "MotionPhoto_Data"
const EOP: &[u8] = b"MotionPhoto_Data";

fn main() {
    if std::env::args().len() < 2 {
        eprintln!("Usage: {} <file>", std::env::args().next().unwrap());
        return;
    }

    let file_name = std::env::args().nth(1).unwrap();
    splitter(&file_name);
}

/// Creates videos and files
fn write_files(file_name: &str, jpg_bytes: &[u8], mp4_bytes: &[u8]) {
    let original_name = file_name.trim_end_matches(".jpg");
    let motion_jpg_name = format!("{}_motion.jpg", original_name);
    let jpg_name = format!("{}.jpg", original_name);
    let mp4_name = format!("{}.mp4", original_name);
    // rename the original file
    std::fs::rename(file_name, &motion_jpg_name).unwrap();
    // `write` creates a new file or replaces the file if it already exists
    std::fs::write(&jpg_name, jpg_bytes).expect("Failed to write JPEG file");
    std::fs::write(&mp4_name, mp4_bytes).expect("Failed to write MP4 file");
}

///  Splits video and picture
fn splitter(file_name: &str) {
    let mut file = std::fs::File::open(file_name).expect("Failed to open file");
    let mut buffer = Vec::new();
    // import trait
    use std::io::Read;
    file.read_to_end(&mut buffer).expect("Failed to read file");
    // size of the file - len of the samsung magic = processed file
    let magic_samsung = buffer.windows(EOP.len()).position(|w| w == EOP);
    // Do not process if magic is not found, and if found at the end
    if magic_samsung.is_none() || magic_samsung.unwrap() == buffer.len() - EOP.len() {
        eprintln!("Error: file {} has no samsung motion photo.", file_name);
        std::process::exit(1);
    }

    let magic_samsung = magic_samsung.unwrap();
    let samsung_jpeg_offset = magic_samsung + EOP.len();
    let mpeg_start = samsung_jpeg_offset;
    let mpeg_end = buffer.len();

    // JPEG  here
    let jpeg = &buffer[..samsung_jpeg_offset];
    // MP4 here
    // Start in the first byte of the MP4 container
    let mp4 = &buffer[mpeg_start..mpeg_end];

    write_files(file_name, jpeg, mp4);
}
