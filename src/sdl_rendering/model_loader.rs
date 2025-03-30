use assimp::Importer;
use nalgebra::{Vector3, Vector2};
use std::ops::Index;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}

pub fn load_fbx(file_path: &str) -> Result<Mesh, String> {
    let mut importer = Importer::new();

    // Importar a cena do FBX
    let scene = importer.read_file(file_path).expect("Falha ao carregar arquivo FBX");

    // Pegando o primeiro mesh
    let ai_mesh = scene.mesh(0).ok_or("Nenhuma malha encontrada")?;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Processar os vértices
    for i in 0..ai_mesh.num_vertices() {
        // let position: na::Matrix<f32, na::Const<3>, na::Const<1>, na::ArrayStorage<f32, 3, 1>> = Vector3::new(
        //     Some(ai_mesh.get_vertex(i)),
        //     Some(ai_mesh.get_vertex(i)),
        //     Some(ai_mesh.get_vertex(i)),
        // ).try_cast().unwrap();

        let position = Vector3::new(
            ai_mesh.get_vertex(i),
            ai_mesh.get_vertex(i),
            ai_mesh.get_vertex(i),
        );

        let normal: na::Matrix<f32, na::Const<3>, na::Const<1>, na::ArrayStorage<f32, 3, 1>>  = if let Some(norm) = ai_mesh.get_normal(i) {
            Vector3::new(norm.x, norm.y, norm.z)
        } else {
            Vector3::zeros()
        };

        let tex_coords = if let Some(tex) = ai_mesh.has_texture_coords(0).then_some(|tc: na::ArrayStorage<f32, 3, 1>| tc.0.get(i as usize)) {
            let tex= na::Vector2 { ..Default::default() };
            Vector2::new(tex.x, tex.y)
        } else {
            Vector2::new(0.0, 0.0)
        };

        let mut vector_vertex = Vector3::new(position.x, position.y, position.z);
        let vertex = Vertex {
            position: vector_vertex,
            normal: normal,
            tex_coords: tex_coords,
        };
        vertices.push(vertex);
    }

    // Processar os índices
    for mut face in ai_mesh.face_iter() {
        if face.num_indices == 3 {
            indices.push(*face.index(0));
            indices.push(*face.index(1));
            indices.push(*face.index(2));
        }
    }

    Ok(Mesh { vertices, indices })
}
