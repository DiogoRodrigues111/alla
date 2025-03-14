use std::io;
use std::fs;
use std::path::Path;

#[allow(unused)]
pub trait FileSystem {
    fn list_dir(&self, path: &str) -> io::Result<String>;
    fn list_simple_dir(&self, path: &str) -> io::Result<String>;
}
#[allow(unused)]
struct FileSystemConfig {
    root: String,
}
impl FileSystem for FileSystemConfig {
    fn list_dir(&self, path: &str) -> io::Result<String> {
        let full_path = Path::new(&self.root).join(path);
        let mut entities = Vec::new();
        for entry in fs::read_dir(&full_path)? {
            let entry = entry?;
            entities.push(entry.path().display().to_string());
            println!("{:?}", entry.path());
        }
        Ok(path.to_string())
    }

    fn list_simple_dir(&self, path: &str) -> io::Result<String> {
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            println!("{:?}", entry.path());
        }
        Ok(path.to_string())
    }
}

#[allow(unused)]
pub fn only_simple_test() {
    let fs_root_instance = FileSystemConfig {
        root: String::from("."),
    };

    println!("\n\n\n");
    fs_root_instance.list_dir(".").unwrap();
    println!("\n\n\n");
    fs_root_instance.list_simple_dir(".").unwrap();
}

/// # Test the filesystem module
/// It is important to test the module to ensure that it works as expected.
/// The test function is only compiled when the test feature is enabled.
/// This is marked by the `#[cfg(test)]` attribute.
#[cfg(test)]
fn _test_filesystem_main() {
    let fs_root_instance = FileSystemConfig {
        root: String::from("."),
    };

    println!("\n\n\n");
    fs_root_instance.list_dir(".").unwrap();
    println!("\n\n\n");
    fs_root_instance.list_simple_dir(".").unwrap();
}