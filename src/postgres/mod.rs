mod error;

pub use self::error::DatabaseError;
pub use self::error::DatabaseResult;
use postgres::config::SslMode;
use postgres::Client;
use postgres::NoTls;

#[derive(Debug)]
pub struct PostgreSQL {
    server: String,
    port: u16,
    user: String,
    password: String,
}

impl PostgreSQL {
    pub fn new(server: &str, port: u16, user: &str, password: &str) -> PostgreSQL {
        PostgreSQL {
            server: server.into(),
            port,
            user: user.into(),
            password: password.into(),
        }
    }

    pub fn database_list<F, T>(&self, callback: F) -> DatabaseResult<Vec<T>>
    where
        F: Fn(&str, i64, u64) -> T,
    {
        let mut connection = self.connect()?;
        let mut result = Vec::new();

        for row in connection
            .query(include_str!("database_list.sql"), &[])
            .map_err(DatabaseError::query_execution_error)?
            .iter()
        {
            let name: String = row.get(0);
            let modified: i64 = row.get(1);
            let size: i64 = row.get(2);

            result.push(callback(&name, modified, size as u64));
        }

        Ok(result)
    }

    pub fn drop_database(&self, database_name: &str) -> DatabaseResult<()> {
        let mut connection = self.connect()?;

        connection
            .execute(include_str!("teminate_backends.sql"), &[&database_name])
            .map_err(DatabaseError::query_execution_error)?;

        connection
            .execute(
                &format!("drop database \"{}\"", database_name.replace("\"", "\"\"")),
                &[],
            )
            .map_err(DatabaseError::query_execution_error)?;

        Ok(())
    }

    fn connect(&self) -> DatabaseResult<Client> {
        Client::configure()
            .ssl_mode(SslMode::Disable)
            .host(&self.server)
            .port(self.port)
            .user(&self.user)
            .user(&self.password)
            .dbname("postgres")
            .connect(NoTls)
            .map_err(DatabaseError::connection_error)
    }
}
