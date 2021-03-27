use crate::prelude::*;
use argon2::verify_encoded as verify;
use std::time::Duration;
use super::rand_string;
use std::path::Path;

impl Users {
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


    #[cfg(feature = "redis-session")]
    pub fn open_redis(&mut self, path: impl redis::IntoConnectionInfo) -> Result<()> {
        let client = redis::Client::open(path)?;
        self.sess = Box::new(client);
        Ok(())
    }

    

    /// Logs a user in for the amout of time specified. 
    pub fn login_for(&self, form: &Login, time: Duration) -> Result<String> {
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email)?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key_for(user.id, time)?;
            Ok(key)
        } else {
            raise(ErrorKind::Unauthorized, "Incorrect password.")
        }
    }

    
    pub fn get_by_id(&self, user_id: u32) -> Result<User> {
        self.conn.get_user_by_id(user_id)
    }

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
        self.conn.delete_user_by_id(id)?;
        Ok(())
    }

    
    pub fn modify(&self, user: User) -> Result<()> {
        self.conn.update_user(user)?;
        Ok(())
    }

    

}
