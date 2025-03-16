#[allow(unused)]
pub trait PgSql {
    fn create_table_pgsql(&self);
}
pub struct PgSqlColumn {
    pub name: String,
    #[allow(unused)]
    pub column_type: String,
}
impl PgSqlColumn {
    #[allow(unused)]
    fn new(name: &str, column_type: postgres::types::Type) -> Self {
        PgSqlColumn {
            name: name.to_string(),
            column_type: column_type.to_string(),
        }
    }
    #[allow(unused)]
    fn new_with_id_default(name: &str, column_type: &str) -> Self {
        PgSqlColumn {
            name: name.to_string(),
            column_type: column_type.to_string(),
        }
    }
    #[allow(unused)]
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.column_type)
    }
}
#[allow(unused)]
struct PgSqlTable {
    name: String,
    columns: Vec<PgSqlColumn>,
    constraints: Vec<postgres::Row>,
}
impl PgSql for PgSqlTable {
    fn create_table_pgsql(&self) {
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
                return;
            }
        };
        let query_format = std::format!("SELECT 1 FROM information_schema.tables WHERE table_name = '{}'", self.name);
        let table_exists = match conn.query(query_format.as_str(), &[]) {
            Ok(row) => !row.is_empty(),
            Err(err) => {
                println!("Error in {}", err); 
                return;
            }
        };
        if table_exists {
            println!("Table already exists with name {}", self.name);
        }
        else {
            let mut query = format!("CREATE TABLE {} (", self.name);
            for column in &self.columns {
                query.push_str(&mut &column.to_string());
                query.push_str(", ");
            }
            for constraint in &self.constraints {
                query.push_str(&constraint.columns().iter().map(|c| c.name().to_string()).collect::<String>());
                query.push_str(", ");
            }
            query.pop();
            query.pop();
            query.push(')');
            match conn.execute(&query, &[]) {
                Ok(_) => println!("Table {} created", self.name),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

/// Postgres for creation table in the database
pub fn _pg_create_table() {
    let table = PgSqlTable {
        name: String::from("users"),
        columns: vec![
            // Id: creation in pgAdmin
            PgSqlColumn::new_with_id_default("name", "VARCHAR"),
            PgSqlColumn::new_with_id_default("email", "VARCHAR"),
            PgSqlColumn::new_with_id_default("password", "VARCHAR"),
        ],
        constraints: vec![],
    };
    table.create_table_pgsql();
}