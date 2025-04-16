use russimp::{mesh::Mesh, scene::{PostProcess, Scene}};

pub struct SceneImporter {
    #[allow(unused)]
    pub mesh: Mesh,
    #[allow(unused)]
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
    
    pub fn new(&self) -> Self {
        Self {
            mesh: Mesh::default(),
            pos: [0.0, 0.0, 0.0],
            path_to_file: String::new(),
        }
    }
}