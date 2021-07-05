use super::rand_string;
use crate::db::DBConnection;
use crate::prelude::*;
#[cfg(feature = "sqlite-db")]

impl Users {
    /// It creates a `Users` instance by connecting  it to a redis database.
    /// If the database does not yet exist it will be created. By default,
    /// sessions will be stored on a concurrent HashMap. In order to have persistent sessions see
    /// the method [`open_redis`](User::open_redis).
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
    #[cfg(feature = "sqlite-db")]
    pub async fn open_sqlite(path: &str) -> Result<Self> {
        use sqlx::Connection;
        use tokio::sync::Mutex;
        let conn = sqlx::SqliteConnection::connect(path).await?;
        let users: Users = Mutex::new(conn).into();
        users.conn.init().await?;
        Ok(users)
    }

    /// Opens a redis connection. It allows for sessions to be stored persistently across
    /// different launches.
    /// ```rust,no_run
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
    #[cfg(feature = "redis-session")]
    pub fn open_redis(&mut self, path: impl redis::IntoConnectionInfo) -> Result<()> {
        let client = redis::Client::open(path)?;
        self.sess = Box::new(client);
        Ok(())
    }

    /// It opens a postgres database connection. I've got to admit I haven't tested this feature yet, so
    /// don't waste your time debugging if it doesn't work.
    /// ```rust, no_run
    /// # use rocket_auth::{Error, Users};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Error> {
    /// let users = Users::open_sqlite("database.db").await?;
    ///
    /// rocket::build()
    ///     .manage(users)
    ///     .launch();
    /// # Ok(()) }
    ///
    /// ```
    #[cfg(feature = "postgres-db")]
    pub async fn open_postgres(path: &str) -> Result<Self> {
        use sqlx::PgPool;
        let conn = PgPool::connect(path).await?;

        conn.init().await?;

        let users = Users {
            conn: Box::new(conn),
            sess: Box::new(chashmap::CHashMap::new()),
        };
        Ok(users)
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
    pub async fn get_by_email(&self, email: &str) -> Result<User> {
        self.conn.get_user_by_email(email).await
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
    pub async fn get_by_id(&self, user_id: i32) -> Result<User> {
        self.conn.get_user_by_id(user_id).await
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
    pub async fn create_user(&self, email: &str, password: &str, is_admin: bool) -> Result<()> {
        let password = password.as_bytes();
        let salt = rand_string(30);
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt.as_bytes(), &config).unwrap();
        self.conn.create_user(email, &hash, is_admin).await?;
        Ok(())
    }

    /// Deletes a user from de database. Note that this method won't delete the session.
    /// To do that use [`Auth::delete`](crate::Auth::delete).
    /// #[get("/delete_user/<id>")]
    /// fn delete_user(id: i32, users: State<Users>) -> Result<String> {
    ///     users.delete(id)?;
    ///     Ok("The user has been deleted.")
    /// }
    pub async fn delete(&self, id: i32) -> Result<()> {
        self.sess.remove(id)?;
        self.conn.delete_user_by_id(id).await?;
        Ok(())
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
    pub async fn modify(&self, user: &User) -> Result<()> {
        self.conn.update_user(user).await?;
        Ok(())
    }
}

/// A `Users` instance can also be created from a database connection.
/// ```
/// # use rocket_auth::{Users, Error};
/// # use tokio_postgres::NoTls;
/// # async fn func() -> Result<(), Error> {
/// let (client, connection) = tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;
/// let users: Users = client.into();
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
/// ```
/// # use rocket_auth::{Users, Error};
/// # extern crate tokio_postgres;
/// # use tokio_postgres::NoTls;
/// # extern crate redis;
/// # async fn func(postgres_path: &str, redis_path: &str) -> Result<(), Error> {
/// let (db_client, connection) = tokio_postgres::connect(postgres_path, NoTls).await?;
/// let redis_client = redis::Client::open(redis_path)?;
///
/// let users: Users = (db_client, redis_client).into();
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

// #[cfg(feature="sqlite-db")]
// #[cfg(feature="redis-session")]
// impl From<(sqlite::Connection, redis::Client)> for Users {
//     fn from((db, ss): (tokio_postgres::Client, redis::Client)) -> Users {
//         Users {
//             conn: Box::from(db),
//             sess: Box::from(ss)
//         }
//     }
// }

#[cfg(feature = "sqlite-db")]
#[cfg(feature = "redis-session")]
#[cfg(test)]
mod tests {
    async fn func(postgres_path: &str, redis_path: &str) {
        use crate::Users;
        use tokio_postgres::NoTls;
        let (db_client, connection) = tokio_postgres::connect(postgres_path, NoTls).await.unwrap();
        let redis_client = redis::Client::open(redis_path).unwrap();
        let users: Users = Users::from((db_client, redis_client));
        // let users: Users = (db_client, redis_client).into();
    }
}
