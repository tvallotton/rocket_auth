pub mod auth;
mod user;
mod users;
use crate::prelude::*;
use argon2::verify_encoded as verify;

use rand::random;
pub fn rand_string(size: usize) -> String {
    (0..)
        .map(|_| random::<char>())
        .filter(|c| c.is_ascii())
        .map(char::from)
        .take(size)
        .collect()
}

impl Users {
    fn is_auth(&self, session: &Session) -> bool {
        let option = self.sess.get(session.id);
        if let Some(auth_key) = option {
            auth_key == session.auth_key
        } else {
            false
        }
    }

    async fn login(&self, form: &Login) -> Result<String> {
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email).await?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key(user.id)?;
            Ok(key)
        } else {
            Err(Error::UnauthorizedError)
        }
    }
    fn logout(&self, session: &Session) -> Result<()> {
        if self.is_auth(session) {
            self.sess.remove(session.id)?;
        }
        Ok(())
    }
    fn set_auth_key_for(&self, user_id: i32, time: Duration) -> Result<String> {
        let key = rand_string(10);
        self.sess.insert_for(user_id, key.clone(), time)?;
        Ok(key)
    }

    fn set_auth_key(&self, user_id: i32) -> Result<String> {
        let key = rand_string(15);
        self.sess.insert(user_id, key.clone())?;
        Ok(key)
    }
    async fn signup(&self, form: &Signup) -> Result<()> {
        form.is_valid()?;
        let email = &form.email;
        let password = &form.password;
        self.create_user(email, password, false).await?;
        Ok(())
    }

    async fn login_for(&self, form: &Login, time: Duration) -> Result<String> {
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email).await?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key_for(user.id, time)?;
            Ok(key)
        } else {
            Err(Error::InvalidCredentialsError)
        }
    }
}
