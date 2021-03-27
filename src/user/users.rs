use crate::prelude::*;
use argon2::verify_encoded as verify;
use std::time::Duration;
use super::rand_string;


impl Users {
    #[cfg(feature = "sqlite-db")]
    pub fn open_sqlite(path: &str) -> Result<Self> {
        use std::sync::Mutex;
        let users = Users {
            conn: Box::new(Mutex::new(rusqlite::Connection::open(path)?)),
            sess: Box::new(chashmap::CHashMap::new()),
        };
        users.conn.init()?;
        Ok(users)
    }

    /******** FORMS ********/
    pub fn login(&self, form: &Login) -> Result<String> {
        
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email)?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key(user.id)?;
            Ok(key)
        } else {
            raise("Contraseña incorrecta.")
        }
    }
    pub fn logout(&self, session: &Session) -> Result<()> {
        if self.is_auth(session) {
            self.sess.remove(session.id)?;
        }
        Ok(())
    }

    pub fn signup(&self, form: &Signup) -> Result<()> {
        form.is_valid()?;
        let email = &form.email;
        let password = &form.password;
        self.create_user(email, password, false)?;
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
            raise("Contraseña incorrecta.")
        }
    }

    /******* ACCESSING *******/
    pub fn get_by_id(&self, user_id: u32) -> Result<User> {
        self.conn.get_user_by_id(user_id)
    }

    pub fn get_by_email(&self, email: &str) -> Result<User> {
        self.conn.get_user_by_email(email)
    }


    
    fn is_auth(&self, session: &Session) -> bool {
        let option = self.sess.get(session.id);
        if let Some(auth_key) = option {
            auth_key == session.auth_key
        } else {
            false
        }
    }

    /********* MANAGE USERS *********/

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

    /// Deletes a user from de database. Note that this method won't delete the session. To do that use `Auth::delete`.
    /// 


    pub fn delete(&self, id: u32) -> Result<()> {
        self.conn.delete_user_by_id(id)?;
        Ok(())
    }

    pub fn modify(&self, user: User) -> Result<()> {
        self.conn.update_user(user)?;
        Ok(())
    }

    /******* HELPERS ********/
    fn set_auth_key_for(&self, user_id: u32, time: Duration) -> Result<String> {
        let key = rand_string(10);
        self.sess.insert_for(user_id.into(), key.clone(), time)?;
        Ok(key)
    }

    fn set_auth_key(&self, user_id: u32) -> Result<String> {
        let key = rand_string(15);
        self.sess.insert(user_id.into(), key.clone())?;
        Ok(key)
    }
}
