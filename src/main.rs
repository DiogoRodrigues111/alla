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
    //postgresqlsrc::create::_pg_crate_database_first();
    //postgresqlsrc::table::_pg_create_table();
    //postgresqlsrc::insert::_pg_insert_data_to_current_table();
    //config::_ini_config_read_main();

    sdl_rendering::_main_with_gl();
}
