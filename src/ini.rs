use std::{fs::File, io::Read};

pub trait IniConfigInfo {
    fn read_of(&mut self, input_file: &str) -> Self;
}
pub struct IniConfig {
    buffer: String,
}
impl IniConfigInfo for IniConfig {
    fn read_of(&mut self, input_file: &str) -> Self {
        let file = File::open(input_file);
        if file.is_err() {
            eprintln!("File could not open");
        }
        else {
            println!("Config.ini loaded");
        }
        let _ = file.map(|mut f| {
            let rd_print_buf = f.read_to_string(&mut self.buffer);
            let _ = match rd_print_buf {
                Ok(r) => println!("{:?}", r),
                Err(err) => { eprintln!("ERROR {}", err) }
            };
        });
        Self {
            buffer: String::new(),
        }
    }
}

pub fn _ini_config_read_main() {
    let mut ini_structure_info = IniConfig { buffer: String::new() };
    ini_structure_info.read_of("target/debug/config.ini");
}