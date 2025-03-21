use std::{fs::File, io::Read};

pub trait IniConfigInfo {
    #[allow(unused)]
    fn read_of(&mut self, input_file: &str) -> Result<(), std::io::Error>;
}
pub struct IniConfig {
    buffer: String,
}
impl IniConfigInfo for IniConfig {
    /// Read the content of the file and store in the buffer
    fn read_of(&mut self, input_file: &str) -> Result<(), std::io::Error> {
        let mut file = File::open(input_file)?;
        file.read_to_string(&mut self.buffer)?;
        println!("{}", self.buffer);
        Ok(())
    }
}
/// Read the content of the file and store in the main function
pub fn _ini_config_read_main() {
    let mut ini_structure_info = IniConfig { buffer: String::new() };
    if let Err(err) = ini_structure_info.read_of("config/config.ini") {
        eprintln!("Error to open or read file {}", err);
    }
}