extern crate sdl2;
extern crate nalgebra as na;
extern crate gl;

mod fsio;

mod postgresqlsrc {
    pub mod create;
    pub mod table;
    pub mod insert;
    pub mod delete;
    pub mod update;
}

mod config;

mod sdl_rendering;

fn main() {
    sdl_rendering::_main_with_gl();
}
