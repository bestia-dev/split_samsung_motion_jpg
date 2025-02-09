// split_samsung_motion_jpg/src/main.rs

#![doc=include_str!("../README.md")]

use log::debug;
use unwrap::unwrap;

// End of picture signature - not JPG footer but "MotionPhoto_Data"
const EOP: &[u8] = b"MotionPhoto_Data";

/// ANSI color
pub const RED: &str = "\x1b[31m";
/// ANSI color
pub const RESET: &str = "\x1b[0m";

/// main scaffolding to catch panic
fn main() {
    // scaffolding for catch panic and log to file
    let _log2 = log2::open("log.txt").size(1 * 1024 * 1024).rotate(3).level("debug").start();

    let version: &'static str = env!("CARGO_PKG_VERSION");
    let app_name: &'static str = env!("CARGO_PKG_NAME");
    log::info!("Start app {app_name} v{version}");

    // catch panics and write to log.txt
    std::panic::set_hook(Box::new(|info| {
        let backtrace = std::backtrace::Backtrace::force_capture();
        handle_panic(info.payload(), backtrace)
    }));
    let _ = std::panic::catch_unwind(|| {
        let _ = main_inner();
    });
}

/// handle panic
fn handle_panic(payload: &(dyn std::any::Any + Send), backtrace: std::backtrace::Backtrace) {
    log::error!("Panicked: ");
    if let Some(string) = payload.downcast_ref::<String>() {
        log::error!("{string}");
    } else if let Some(str) = payload.downcast_ref::<&'static str>() {
        log::error!("{str}")
    } else {
        log::error!("{payload:?}")
    }

    log::error!("Backtrace: {backtrace:#?}");
}

/// true main function for the content without scaffolding
fn main_inner() {
    if std::env::args().len() < 2 {
        let app_name = unwrap!(std::env::args().next());
        println!("Usage for single file:");
        println!(r#"{} "<file>" "#, app_name);
        println!("Usage for all jpg files in recursive sub-folders. Here must use quotes:");
        println!(r#"{} "data/**/*.jpg" "#, app_name);

        return;
    }

    let glob_input = unwrap!(std::env::args().nth(1));
    // accept unix style path patterns
    // glob("data/**/*.jpg")
    // all jpg files in data/ and all of its subdirectories
    // It is forbidden to use `..` for parent folders
    if glob_input.contains("..") {
        eprintln!("{RED}Error: it is forbidden to use double dot '..' inside a glob pattern. {RESET}");
        return;
    }
    // if there is no wildcards * and ?, it is just one file
    if !(glob_input.contains('*') || glob_input.contains('?')) {
        debug!("One file");
        if splitter(&glob_input).is_none() {
            eprintln!("{RED}Error: file {} has no samsung motion photo. {RESET}", glob_input);
        }
    } else {
        debug!("Glob wildcard pattern");
        let mut count_all_files = 0;
        let mut count_motion_files = 0;
        for (i, entry) in glob::glob(&glob_input).expect(&format!("{RED}Failed to read glob pattern {RESET}")).enumerate() {
            match entry {
                Ok(path) => {
                    // don't print error for every file that is not motion jpg,
                    // but only print the files that are splitted
                    if splitter(&path.to_string_lossy()).is_some() {
                        count_motion_files += 1;
                        println!("\n{count_motion_files} - {}", path.to_string_lossy());
                    }
                }
                Err(e) => eprintln!("\n{RED}Error: {:?} {RESET}", e),
            }
            count_all_files = i + 1;
            // progress bar
            if count_all_files % 10 == 0 {
                print!(".");
                use std::io::Write;
                std::io::stdout().flush().unwrap();
            }
        }
        println!("\n");
        println!("Splitted files: {count_motion_files}");
        println!("All files: {count_all_files}");
    }
}

///  Splits video and picture
fn splitter(file_name: &str) -> Option<()> {
    let Some((buffer, magic_samsung)) = check_if_motion_jpg(file_name) else {
        return None;
    };

    let samsung_jpg_offset = magic_samsung + EOP.len();
    let mpeg_start = samsung_jpg_offset;
    let mpeg_end = buffer.len();

    // JPG  here
    let jpg_bytes = &buffer[..samsung_jpg_offset];
    // MP4 here
    // Start in the first byte of the MP4 container
    let mp4_bytes = &buffer[mpeg_start..mpeg_end];

    write_files(file_name, jpg_bytes, mp4_bytes);
    Some(())
}

/// check the file if it is a motion jpg
fn check_if_motion_jpg(file_name: &str) -> Option<(Vec<u8>, usize)> {
    let mut file = std::fs::File::open(file_name).expect(&format!("{RED}Failed to open file {RESET}"));
    let mut buffer = Vec::new();
    // import trait
    use std::io::Read;
    file.read_to_end(&mut buffer).expect(&format!("{RED}Failed to read file {RESET}"));
    // size of the file - len of the samsung magic = processed file
    let magic_samsung = buffer.windows(EOP.len()).position(|w| w == EOP);
    // Do not process if magic is not found, and if found at the end
    if magic_samsung.is_none() {
        return None;
    }
    let magic_samsung = unwrap!(magic_samsung);
    if magic_samsung == buffer.len() - EOP.len() {
        return None;
    }
    Some((buffer, magic_samsung))
}

/// Creates videos and files
fn write_files(file_name: &str, jpg_bytes: &[u8], mp4_bytes: &[u8]) {
    let original_name = file_name.trim_end_matches(".jpg");
    let motion_jpg_name = format!("{}_motion.jpg", original_name);
    let jpg_name = format!("{}.jpg", original_name);
    let mp4_name = format!("{}.mp4", original_name);
    // rename the original file
    unwrap!(std::fs::rename(file_name, &motion_jpg_name));
    // `write` creates a new file or replaces the file if it already exists
    std::fs::write(&jpg_name, jpg_bytes).expect(&format!("{RED}Failed to write JPG file {RESET}"));
    std::fs::write(&mp4_name, mp4_bytes).expect(&format!("{RED}Failed to write MP4 file {RESET}"));
}
