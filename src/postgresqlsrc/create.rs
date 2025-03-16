
pub trait PgSql {
    #[allow(unused)]
    fn create_database_pgsql(&self);
}
pub struct PgSqlDatabase {
    pub namedb: String,
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
}
impl PgSql for PgSqlDatabase {
    fn create_database_pgsql(&self) {
        let conn = format!(
            "postgresql://{}:{}@{}:{}/postgres",
            self.user, self.password, self.host, self.port
        );
        let mut conn = match postgres::Client::connect(&conn, postgres::NoTls) {
            Ok(conn) => conn,
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        };
        let check_query = format!("SELECT 1 FROM pg_database WHERE datname = '{}'", self.namedb);
        let db_exists = match conn.query(check_query.as_str(), &[]) {
            Ok(row) => !row.is_empty(),
            Err(err) => {
                println!("Error {}", err);
                return;
            }
        };
        if db_exists {
            println!("Database in {} already exists", self.namedb);
        }
        else {
            let query = format!("CREATE DATABASE {}", self.namedb);
            match conn.execute(&query, &[]) {
                Ok(_) => println!("Database {} created", self.namedb),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

/// Create database first in Postgres
pub fn _pg_crate_database_first() {
    let pgsql = PgSqlDatabase {
        namedb: String::from("diogorodrigues_db"),
        user: String::from("postgres"),
        password: String::from("root"),
        host: String::from("localhost"),
        port: 5432,
    };
    pgsql.create_database_pgsql();
}