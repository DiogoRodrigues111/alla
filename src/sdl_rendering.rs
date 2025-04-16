use importer::SceneImporter;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Instant;
use russimp::{mesh::Mesh, scene::{PostProcess, Scene}};
use na::{Matrix4, Perspective3, Point3, Vector3};

mod shader_utils;
mod importer;

/// Main function to run the SDL2 OpenGL application
/// This function initializes SDL2, creates a window, and sets up OpenGL context.
/// It also handles user input for camera movement and renders a 3D scene with a model.
/// # Example
/// ```rust```
/// fn main() {
///    _main_with_gl();
/// }
/// ```
/// # Panics
/// This function may panic if there are issues with SDL2 initialization, window creation, or OpenGL context creation.
/// It may also panic if there are issues with loading OpenGL functions or creating shaders.
/// # Errors
/// This function may return errors related to SDL2 initialization, window creation, OpenGL context creation, or shader compilation.
/// It may also return errors related to importing the 3D model using Assimp.
pub fn _main_with_gl() {
    // --- Inicialização SDL2 ---
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let mut window = video
        .window("Rust SDL2 OpenGL", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // --- Definindo cubo ---
    #[allow(unused)]
    let vertices: [f32; 90] = [
        // positions
        -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5,
        -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5,
        -0.5,  0.5,  0.5, -0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5, -0.5,  0.5,  0.5,
         0.5, -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,
        -0.5, -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5, -0.5, -0.5, -0.5,
    ];

    // --- Importer the model, assimp into the 3D scene ---
    /*** 
    #### The accord with manual forcesing the user-creator say: ####
        vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType]
            ).unwrap(); 
    ####
        Myself:
            PostProcess::Triangulate 
            , PostProcess::JoinIdenticalVertices 
            , PostProcess::ValidateDataStructure
            , PostProcess::OptimizeMeshes 
        ***/
    let post_process_steps = vec![
            PostProcess::CalculateTangentSpace,
            PostProcess::Triangulate,
            PostProcess::JoinIdenticalVertices,
            PostProcess::SortByPrimitiveType];

    let scene_importer: SceneImporter = SceneImporter {
        mesh: Mesh::default(),
        pos: [0.0, 0.0, 0.0],
        path_to_file: String::new(),
    };
    let mut importer_to_scene = SceneImporter::new(&scene_importer);
    importer_to_scene.path_to_file = String::from("arch/room/room.obj");
    let path_archive = importer_to_scene.path_to_file.to_string();
    let scene_from_file = Scene::from_file(&path_archive, post_process_steps).unwrap();

    let mesh = &scene_from_file.meshes[0];
    let vertices: Vec<SceneImporter> = mesh.vertices.iter().map(|v| SceneImporter {
        mesh: Mesh::default(),
        pos: [v.x, v.y, v.z],
        path_to_file: String::new(),
    }).collect();
    let indices: Vec<u32> = mesh.faces.iter().flat_map(|f| f.0.clone()).collect();

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, std::mem::size_of::<SceneImporter>() as i32, std::ptr::null());
        gl::BindVertexArray(0);

        gl::Enable(gl::DEPTH_TEST);
    }

    let shader = shader_utils::Shader::new("src/sdl_rendering/shader_utils/vertex_shader.glsl", "src/sdl_rendering/shader_utils/fragment_shader.glsl");

    // --- Câmera FPS ---
    let mut camera_pos = Point3::new(0.0, 0.0, 3.0);
    let mut camera_front = Vector3::new(0.0, 0.0, -1.0);
    let camera_up = Vector3::y();

    let mut yaw = -90.0_f32;
    let mut pitch = 0.0_f32;
    let _last_mouse_x = 400.0;
    let _last_mouse_y = 300.0;
    let sensitivity = 0.1;
    let _first_mouse = true;

    let mut event_pump = sdl.event_pump().unwrap();
    let mut delta_time;
    let mut last_frame = Instant::now();

    window.set_mouse_grab(true);

    'running: loop {
        delta_time = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { xrel, yrel, .. } => {
                    let xoffset = xrel as f32 * sensitivity;
                    let yoffset = -(yrel as f32) * sensitivity;

                    yaw += xoffset;
                    pitch += yoffset;
                    pitch = pitch.clamp(-89.0, 89.0);

                    let front = Vector3::new(
                        yaw.to_radians().cos() * pitch.to_radians().cos(),
                        pitch.to_radians().sin(),
                        yaw.to_radians().sin() * pitch.to_radians().cos(),
                    );
                    camera_front = front.normalize();
                }
                _ => {}
            }
        }

        let keyboard_state = event_pump.keyboard_state();
        let camera_speed = 2.5 * delta_time;
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) {
            camera_pos += camera_front * camera_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) {
            camera_pos -= camera_front * camera_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) {
            camera_pos -= camera_front.cross(&camera_up).normalize() * camera_speed;
        }
        if keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) {
            camera_pos += camera_front.cross(&camera_up).normalize() * camera_speed;
        }

        // --- Renderização ---
        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        shader.use_program();

        // Matriz de view e projeção
        let view = Matrix4::look_at_rh(&camera_pos, &(camera_pos + camera_front), &camera_up);
        let projection = Perspective3::new(800.0 / 600.0, 45.0_f32.to_radians(), 0.1, 100.0);
        shader.set_mat4("view", &view);
        shader.set_mat4("projection", projection.as_matrix());

        // Model
        let model = Matrix4::<f32>::identity();
        shader.set_mat4("model", &model);

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }

        window.gl_swap_window();
    }
}