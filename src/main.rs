mod fsio;

mod postgresqlsrc {
    pub mod create;
    pub mod table;
    pub mod insert;
    pub mod delete;
    pub mod update;
}

fn main() {
    postgresqlsrc::create::_pg_crate_database_first();
    postgresqlsrc::table::_pg_create_table();
}
