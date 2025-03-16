pub trait PgSqlInsert {
    fn new_to_table(&self, name: String, current_table: String) -> Self;
}
pub struct PgSqlData {
    new_data: String,
    email: String,
    password: String,
    current_table: String,
}
impl PgSqlInsert for PgSqlData {
    /// Create an new table in Posgres
    /// Params Optional: name is optional (dont have any effect)
    /// Params Needed: _current_table is required
    fn new_to_table(&self, name: String, _current_table: String) -> Self {
        let _current_table = self.current_table.to_string();
        let table = crate::postgresqlsrc::table::PgSqlColumn {
            name: _current_table,
            column_type: String::new(),
        };
        let database = crate::postgresqlsrc::create::PgSqlDatabase {
            namedb: String::from("diogorodrigues_db"),
            user: String::from("postgres"),
            password: String::from("root"),
            host: String::from("localhost"),
            port: 5432,
        };
        let conn = format!(
            "postgresql://{}:{}@{}:{}/{}",
            database.user, database.password, database.host, database.port, database.namedb
        );
        let mut conn = match postgres::Client::connect(&conn, postgres::NoTls) {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error: {}", e);
                return Self {
                    new_data: String::new(),
                    email: String::new(),
                    password: String::new(),
                    current_table: table.name
                }
            }
        };
        let query_format = std::format!("INSERT INTO {} (name, email, password) VALUES ('{}', '{}', '{}')", table.name, &self.new_data, &self.email, &self.password);
        match conn.execute(&query_format, &[]) {
            Ok(_) => println!("New Data {}, {}, {} Inserted", &self.new_data, &self.email, &self.password),
            Err(e) => println!("Error: {}", e),
        }
        Self {
            new_data: name,
            email: String::new(),
            password: String::new(),
            current_table: table.name,
        }
    }
}

/// Main to PostgreSQL insert data
pub fn _pg_insert_data_to_current_table() {
    let to_table = PgSqlData {
        new_data: String::from("amigo 2"),
        email: String::from("MyEmail@Pro.com"),
        password: String::from("Minha Senha Segura"),
        current_table: String::from("users"),
    };
    to_table.new_to_table(String::new(), String::from("users"));
}