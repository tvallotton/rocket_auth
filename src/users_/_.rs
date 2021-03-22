use super::forms::{CookieUser, LoginForm, SigninForm};
use crate::prelude::*;

pub use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};

const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    email TEXT UNIQUE,
    password INTEGER NOT NULL,
    is_admin BOOL DEFAULT 0,
    auth_key INTEGER
);";

struct User {
    id: u32,
    email: String,
    password: u32,
    is_admin: bool,
    auth_key: Option<u32>,
}


pub struct Users {
    conn: DBConn,
}




impl TryFrom<DBConn> for Users {
    type Error = Error;
    fn try_from(conn: DBConn) -> Result<Users> {
        let out = Users { conn };
        out.init()?;
        Ok(out)
    }
}


// impl TryFrom<Connection> for Users {
//     type Error = Error;
//     fn try_from(conn: Connection) -> Result<Users> {
//         let out = Users { conn };
//         out.init()?;
//         Ok(out)
//     }
// }

impl Users {
    fn init(&self) -> Result<()> {
        self.execute(CREATE_TABLE, params![])?;
        Ok(())
    }

    fn execute<P>(&self, sql: &str, params: P) -> Result<usize>
    where
        P: IntoIterator,
        P::Item: rusqlite::ToSql,
    {
        
        let out = self.conn.execute(sql, params)?;
        Ok(out)
    }
    pub fn is_auth(&self, cookies: &CookieUser) -> Result<bool> {
        let user = self.get_by_id(cookies.id)?;
        Ok(user.auth_key == Some(cookies.auth_key))
    }

    pub fn signin(&self, form: SigninForm) -> Result<()> {
        todo!()
    }

    pub fn login(&self, form: LoginForm) -> Result<u32> {
        let user = self.get_by_email(form.email)?;
        if user.password == hash(form.password) {
            let key = self.set_auth_key(user.id)?;
            Ok(key)
        } else {
            raise("Contraseña incorrecta.")
        }
    }
    pub fn logout(&self, cookies: &CookieUser) -> Result<()> {
        if self.is_auth(cookies)? {
            self.execute(
                "UPDATE users SET auth_key = ?1 WHERE id = ?2",
                params![None::<i32>, cookies.id],
            );
            Ok(())
        } else {
            raise("Usuario no autentificado.")
        }
    }
    fn set_auth_key(&self, user_id: u32) -> Result<u32> {
        let auth_key: u32 = rand::random();
        self.execute(
            "UPDATE users SET auth_key = ?1 WHERE id = ?2;",
            params![auth_key, user_id],
        )?;
        Ok(auth_key)
    }

    pub fn get_by_id(&self, user_id: u32) -> Result<User> {
        let user = self
            .conn
            .query_row(
                "SELECT * FROM users WHERE id = ?1;",
                params![user_id],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        email: row.get(1)?,
                        password: row.get(2)?,
                        is_admin: row.get(3)?,
                        auth_key: row.get(4)?,
                    })
                },
            )
            .msg("No se encontró al usuario.")?;
        Ok(user)
    }

    fn get_by_email(&self, email: String) -> Result<User> {
        let user = self
            .conn
            .query_row(
                "SELECT * FROM users WHERE email = ?1;",
                params![email],
                |row| {
                    Ok(User {
                        id: row.get(0)?,
                        email: row.get(1)?,
                        password: row.get(2)?,
                        is_admin: row.get(3)?,
                        auth_key: row.get(4)?,
                    })
                },
            )
            .msg("No se encontró al usuario.")?;
        Ok(user)
    }
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash<T: Hash>(obj: T) -> u32 {
    let mut s = DefaultHasher::new();
    obj.hash(&mut s);
    s.finish() as u32
}

use std::time::{SystemTime, UNIX_EPOCH};

fn now() -> Result<u128> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .msg("500")?
        .as_millis())
}
