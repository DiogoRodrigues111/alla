pub struct SceneImporter {
    pub pos: [f32; 3],
    pub path_to_file: String,
}

#[allow(unused)]
pub enum SceneImporterInfo {
    Imported,
    Failure,
    ImportedWithWarnings,
}

impl SceneImporter {
    
    pub fn new() -> Self {
        Self {
            pos: [0.0, 0.0, 0.0],
            path_to_file: String::new(),
        }
    }
}