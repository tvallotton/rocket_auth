use super::rand_string;
use crate::prelude::*;

#[cfg(feature = "sqlite-db")]
use std::path::Path;

impl Users {
    /// It creates a `Users instance by connecting  it to a redis database.
    /// If the database does not yet exist it will be created. By default,
    /// sessions will be stored on a concurrent HashMap. In order to have persistent sessions see
    /// the method (User::open_redis)[`open_redis`].
    /// ```rust
    /// let users = Users::open_sqlite("database.db")?;
    ///
    /// rocket::ignite()
    ///     .manage(users)
    ///     .launch();
    /// ```
    #[cfg(feature = "sqlite-db")]
    pub fn open_sqlite(path: impl AsRef<Path>) -> Result<Self> {
        use std::sync::Mutex;
        let users = Users {
            conn: Box::new(Mutex::new(rusqlite::Connection::open(path)?)),
            sess: Box::new(chashmap::CHashMap::new()),
        };
        users.conn.init()?;
        Ok(users)
    }

    /// Opens a redis connection. It allows for sessions to be stored persistently across
    /// different launches.
    /// ```rust
    /// let users = Users::open_sqlite("database.db")?;
    /// users.open_redis("redis://127.0.0.1/")?;
    ///
    /// rocket::ignite()
    ///     .manage(users)
    ///     .launch();
    /// ```
    #[cfg(feature = "redis-session")]
    pub fn open_redis(&mut self, path: impl redis::IntoConnectionInfo) -> Result<()> {
        let client = redis::Client::open(path)?;
        self.sess = Box::new(client);
        Ok(())
    }

    /// It opens a postgres database connection. I've got to admit I haven't tested this feature yet, so
    /// don't waste your time if it doesn't. 
    /// ````rust
    /// let users = Users::open_sqlite("database.db")?;
    /// 
    /// rocket::ignite()
    ///     .manage(users)
    ///     .launch();
    /// ```
    #[cfg(feature = "postgres-db")]
    pub fn open_postgres(path: &str) -> Result<Self> {
        use tokio::runtime::Builder;
        use tokio_postgres::{connect, NoTls};
        let rt = Builder::new_current_thread().enable_io().build()?;
        let (client, conn) = rt.block_on(async { connect(path, NoTls).await })?;

        std::thread::spawn(move || {
            rt.block_on(async {
                if let Err(e) = conn.await {
                    eprintln!("Postgresql error: {}", e);
                }
            })
        });
        client.init()?;
        let users = Users {
            conn: Box::new(client),
            sess: Box::new(chashmap::CHashMap::new()),
        };
        Ok(users)
    }

    pub fn get_by_id(&self, user_id: u32) -> Result<User> {
        self.conn.get_user_by_id(user_id)
    }

    /// It querys a user by their email.
    /// #[get("/user-information/<email>")]
    /// fn user_information(email: String) -> String {
    ///     
    /// }
    pub fn get_by_email(&self, email: &str) -> Result<User> {
        self.conn.get_user_by_email(email)
    }

    /// Inserts a user in the database.
    /// # Example
    /// ```rust
    /// #[get("/create_admin/<email>/<pasword>")]
    /// fn create_admin(email: String, password: String, users: State<Users>) -> Result<String> {
    ///     users.create_user(email, password, true)?;
    ///     Ok("User created successfully")
    /// }
    /// ```
    pub fn create_user(&self, email: &str, password: &str, is_admin: bool) -> Result<()> {
        let password = password.as_bytes();
        let salt = rand_string(10);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.conn.create_user(email, &hash, is_admin)?;
        Ok(())
    }

    /// Deletes a user from de database. Note that this method won't delete the session.
    /// To do that use [`Auth::delete`].
    /// #[get("/delete_user/<id>")]
    /// fn delete_user(id: u32, users: State<Users>) -> Result<String> {
    ///     users.delete(id)?;
    ///     Ok("The user has been deleted.")
    /// }
    pub fn delete(&self, id: u32) -> Result<()> {
        self.sess.remove(id)?;
        self.conn.delete_user_by_id(id)?;
        Ok(())
    }

    pub fn modify(&self, user: User) -> Result<()> {
        self.conn.update_user(user)?;
        Ok(())
    }
}

// #[cfg(feature = "postgres-db")]
// impl From<client>
