use super::rand_string;
use crate::db::DBConnection;
use crate::prelude::*;

#[cfg(feature = "rusqlite")]
use std::path::Path;

impl Users {
    /// It creates a `Users` instance by connecting  it to a sqlite database.
    /// This method uses the [`sqlx`] crate.
    /// If the database does not yet exist it will return an Error. By default,
    /// sessions will be stored on a concurrent HashMap. In order to have persistent sessions see
    /// the method [`open_redis`](crate::Users::open_redis).
    /// ```rust, no_run
    /// # use rocket_auth::{Error, Users};
    /// # #[tokio::main]
    /// # async fn main() -> Result <(), Error> {
    /// let users = Users::open_sqlite("database.db").await?;
    ///
    /// rocket::build()
    ///     .manage(users)
    ///     .launch()
    ///     .await;
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "sqlx-sqlite")]
    #[throws(Error)]
    pub async fn open_sqlite(path: &str) -> Self {
        let conn = sqlx::SqlitePool::connect(path).await?;
        let users: Users = conn.into();
        users.create_table().await?;
        users
    }
    /// Initializes the user table in the database. It won't drop the table if it already exists.
    /// It is necessary to call it explicitly when casting the `Users` struct from an already
    /// stablished database connection and if the table hasn't been created yet. If the table
    /// already exists then this step is not necessary.
    /// ```rust,
    /// # use sqlx::{sqlite::SqlitePool, Connection};
    /// # use rocket_auth::{Users, Error};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let mut conn = SqlitePool::connect("database.db").await?;
    /// let mut users: Users = conn.into();
    /// users.open_redis("redis://127.0.0.1/")?;
    /// users.create_table().await?;
    /// # Ok(()) }
    /// ```
    #[throws(Error)]
    pub async fn create_table(&self) {
        self.conn.init().await?
    }
    /// Opens a redis connection. It allows for sessions to be stored persistently across
    /// different launches. Note that persistent sessions also require a `secret_key` to be set in the [Rocket.toml](https://rocket.rs/v0.5-rc/guide/configuration/#configuration) configuration file.
    /// ```rust,
    /// # use rocket_auth::{Users, Error};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let mut users = Users::open_sqlite("database.db").await?;
    /// users.open_redis("redis://127.0.0.1/")?;
    ///
    /// rocket::build()
    ///     .manage(users)
    ///     .launch();
    ///
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "redis")]
    #[throws(Error)]
    pub fn open_redis(&mut self, path: impl redis::IntoConnectionInfo) {
        let client = redis::Client::open(path)?;
        self.sess = Box::new(client);
    }

    /// It creates a `Users` instance by connecting  it to a sqlite database.
    /// This method uses the [`rusqlite`] crate.
    /// If the database does not yet exist it will attempt to create it. By default,
    /// sessions will be stored on a concurrent HashMap. In order to have persistent sessions see
    /// the method [`open_redis`](Users::open_redis).
    /// ```rust, no_run
    /// # use rocket_auth::{Error, Users};
    /// # #[tokio::main]
    /// # async fn main() -> Result <(), Error> {
    /// let users = Users::open_rusqlite("database.db")?;
    ///
    /// rocket::build()
    ///     .manage(users)
    ///     .launch()
    ///     .await;
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "rusqlite")]
    #[throws(Error)]
    pub fn open_rusqlite(path: impl AsRef<Path>) -> Self {
        use tokio::sync::Mutex;
        let users = Users {
            conn: Box::new(Mutex::new(rusqlite::Connection::open(path)?)),
            sess: Box::new(chashmap::CHashMap::new()),
        };
        futures::executor::block_on(users.conn.init())?;
        users
    }

    /// It creates a `Users` instance by connecting  it to a mysql database.
    /// This method uses the [`sqlx`] crate.
    ///
    /// ```rust, no_run
    /// # use rocket_auth::{Error, Users};
    /// # async fn func(DATABASE_URL: &str) -> Result<(), Error> {
    /// let users = Users::open_mysql(DATABASE_URL).await?;
    ///
    /// rocket::build()
    ///     .manage(users)
    ///     .launch();
    /// # Ok(()) }
    ///
    /// ```

    #[cfg(feature = "sqlx-mysql")]
    #[throws(Error)]
    pub async fn open_mysql(path: &str) -> Self {
        let conn = sqlx::MySqlPool::connect(path).await?;
        let users: Users = conn.into();
        users.create_table().await?;
        users
    }

    /// It creates a `Users` instance by connecting  it to a postgres database.
    /// This method uses the [`sqlx`] crate.
    ///
    /// ```rust, no_run
    /// # use rocket_auth::{Error, Users};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let users = Users::open_postgres("postgres://postgres:password@localhost/test").await?;
    ///
    /// rocket::build()
    ///     .manage(users)
    ///     .launch();
    /// # Ok(()) }
    ///
    /// ```
    #[cfg(feature = "sqlx-postgres")]
    #[throws(Error)]
    pub async fn open_postgres(path: &str) -> Self {
        use sqlx::PgPool;
        let conn = PgPool::connect(path).await?;
        conn.init().await?;
        let users = Users {
            conn: Box::new(conn),
            sess: Box::new(chashmap::CHashMap::new()),
        };
        users
    }
    /// It querys a user by their email.
    /// ```
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Users};
    /// #[get("/user-information/<email>")]
    /// async fn user_information(email: String, users: &State<Users>) -> Result<String, Error> {
    ///        
    ///     let user = users.get_by_email(&email).await?;
    ///     Ok(format!("{:?}", user))
    /// }
    /// # fn main() {}
    /// ```
    #[throws(Error)]
    pub async fn get_by_email(&self, email: &str) -> User {
        self.conn.get_user_by_email(email).await?
    }

    /// It querys a user by their email.
    /// ```
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Users};
    /// # #[get("/user-information/<email>")]
    /// # async fn user_information(email: String, users: &State<Users>) -> Result<(), Error> {
    ///  let user = users.get_by_id(3).await?;
    ///  format!("{:?}", user);
    /// # Ok(())
    /// # }
    /// # fn main() {}
    /// ```
    #[throws(Error)]
    pub async fn get_by_id(&self, user_id: i32) -> User {
        self.conn.get_user_by_id(user_id).await?
    }

    /// Inserts a new user in the database. It will fail if the user already exists.
    /// ```rust
    /// # use rocket::{State, get};
    /// # use rocket_auth::{Error, Users};
    /// #[get("/create_admin/<email>/<password>")]
    /// async fn create_admin(email: String, password: String, users: &State<Users>) -> Result<String, Error> {
    ///     users.create_user(&email, &password, true).await?;
    ///     Ok("User created successfully".into())
    /// }
    /// # fn main() {}
    /// ```
    #[throws(Error)]
    pub async fn create_user(&self, email: &str, password: &str, is_admin: bool) {
        let password = password.as_bytes();
        let salt = rand_string(30);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, salt.as_bytes(), &config).unwrap();
        self.conn.create_user(email, &hash, is_admin).await?;
    }

    /// Deletes a user from de database. Note that this method won't delete the session.
    /// To do that use [`Auth::delete`](crate::Auth::delete).
    /// ```
    /// #[get("/delete_user/<id>")]
    /// fn delete_user(id: i32, users: State<Users>) -> Result<String> {
    ///     users.delete(id)?;
    ///     Ok("The user has been deleted.")
    /// }
    /// ```
    #[throws(Error)]
    pub async fn delete(&self, id: i32) {
        self.sess.remove(id)?;
        self.conn.delete_user_by_id(id).await?;
    }

    /// Modifies a user in the database.
    /// ```
    /// # use rocket_auth::{Users, Error};
    /// # async fn func(users: Users) -> Result<(), Error> {
    /// let mut user = users.get_by_id(4).await?;
    /// user.set_email("new@email.com");
    /// user.set_password("new password");
    /// users.modify(&user).await?;
    /// # Ok(())}
    /// ```
    #[throws(Error)]
    pub async fn modify(&self, user: &User) {
        self.conn.update_user(user).await?;
    }
}

/// A `Users` instance can also be created from a database connection.
/// ```rust
/// # use rocket_auth::{Users, Error};
/// # use tokio_postgres::NoTls;
/// # async fn func() -> Result<(), Error> {
/// let (client, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
/// let users: Users = client.into();
/// // we create the user table in the
/// // database if it does not exists.
/// users.create_table();
/// # Ok(())}
/// ```

impl<Conn: 'static + DBConnection> From<Conn> for Users {
    fn from(db: Conn) -> Users {
        Users {
            conn: Box::from(db),
            sess: Box::new(chashmap::CHashMap::new()),
        }
    }
}

/// Additionally, `Users` can be created from a tuple,
/// where the first element is a database connection, and the second is a redis connection.
/// ```rust
/// # use rocket_auth::{Users, Error};
/// # extern crate tokio_postgres;
/// # use tokio_postgres::NoTls;
/// # extern crate redis;
/// # async fn func(postgres_path: &str, redis_path: &str) -> Result<(), Error> {
/// let (db_client, connection) = tokio_postgres::connect(postgres_path, NoTls).await?;
/// let redis_client = redis::Client::open(redis_path)?;
///
/// let users: Users = (db_client, redis_client).into();
/// // we create the user table in the
/// // database if it does not exists.
/// users.create_table();
/// # Ok(())}
/// ```
impl<T0: 'static + DBConnection, T1: 'static + SessionManager> From<(T0, T1)> for Users {
    fn from((db, ss): (T0, T1)) -> Users {
        Users {
            conn: Box::from(db),
            sess: Box::new(ss),
        }
    }
}
