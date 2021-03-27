pub mod auth;
mod user;
mod users;
use crate::prelude::*;
use argon2::verify_encoded as verify;

use rand::random;
pub fn rand_string(size: usize) -> String {
    // let dissallowed = ['\\', '"', '{', '}', '(', ')', '`', '\''];
    (0..)
        .map(|_| random::<u8>())
        .filter(|n| 31 < *n && *n < 126)
        .map(|n| char::from(n))
        // .filter(|c| !dissallowed.contains(c))
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

    fn login(&self, form: &Login) -> Result<String> {
        let form_pwd = &form.password.as_bytes();
        let user = self.conn.get_user_by_email(&form.email)?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            let key = self.set_auth_key(user.id)?;
            Ok(key)
        } else {
            raise(ErrorKind::Unauthorized, "Incorrect password.")
        }
    }
    fn logout(&self, session: &Session) -> Result<()> {
        if self.is_auth(session) {
            self.sess.remove(session.id)?;
        }
        Ok(())
    }
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
     fn signup(&self, form: &Signup) -> Result<()> {
        form.is_valid()?;
        let email = &form.email;
        let password = &form.password;
        self.create_user(email, password, false)?;
        Ok(())
    }

    fn login_for(&self, form: &Login, time: Duration) -> Result<String> {
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
}
