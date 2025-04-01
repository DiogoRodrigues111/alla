use assimp::Importer;
use nalgebra::{Vector3, Vector2};
use std::{any::TypeId, ops::Index};

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

        let ii = i.clone() as u32;

        let _position = Vector3::new(
            ai_mesh.get_vertex(ii),
            ai_mesh.get_vertex(ii),
            ai_mesh.get_vertex(ii),
        );

        let i3 = i.clone() as u32;

        let normal: na::Matrix<f32, na::Const<3>, na::Const<1>, na::ArrayStorage<f32, 3, 1>>  = if let Some(norm) = ai_mesh.get_normal(i) {
            Vector3::new(norm.x, norm.y, norm.z)
        } else {
            Vector3::zeros()
        };

        
        unsafe {
            let _slice_index: &[f32] = std::slice::from_raw_parts(i3 as *const _, std::u32::MAX.try_into().unwrap());

            let tex_coords = if let Some(_tex) = ai_mesh.has_texture_coords(0).then_some(|tc: na::ArrayStorage<f32, 3, 1>| tc) {
                Vector2::new(0f32, 0f32)
            } else {
                Vector2::new(0f32, 0f32)
            };

            let vector_vertex: na::Matrix<f32, na::Const<3>, na::Const<1>, na::ArrayStorage<f32, 3, 1>> = na::Vector3::from_vec(Vec::new());
            let vector_vertx_pos = vector_vertex; // na::Matrix<f32, na::Const<3>, na::Const<1>, na::ArrayStorage<f32, 3, 1>>
            let vertex = Vertex {
                position: vector_vertx_pos,
                normal: normal,
                tex_coords: tex_coords,
            };
            vertices.push(vertex);
        }

    }

    // Processar os índices
    for face in ai_mesh.face_iter() {
        if face.num_indices == 3 {
            indices.push(*face.index(0));
            indices.push(*face.index(1));
            indices.push(*face.index(2));
        }
    }

    Ok(Mesh { vertices, indices })
}
