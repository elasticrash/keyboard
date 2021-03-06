use crate::config::model::Layout;
use std::fs::File;
use std::io::Read;
use std::path::Path;
/**
 * Reads configuration from provided file
 */
pub fn read(filename: &str) -> serde_json::Result<Layout> {
    let mut buffer = String::new();
    let mut config = Ok(Layout::default());
    match File::open(filename) {
        Ok(mut file) => {
            file.read_to_string(&mut buffer).unwrap();
            println!(
                "[{}] - Reading {:?}",
                line!(),
                Path::new(filename).file_name()
            );
            config = serde_json::from_str(&buffer);
        }
        Err(_why) => {},
    };
    config
}
