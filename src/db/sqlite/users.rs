

use rusqlite::Connection;



pub use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Arc, Mutex,
};
const SALT_LENGTH: usize = 15;

const CREATE_USER: &str = "
INSERT INTO users (email, password) VALUES (?1, ?2);
";

const CREATE_TABLE: &str = "
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    email TEXT UNIQUE,
    password TEXT NOT NULL,
    is_admin BOOL DEFAULT 0,
    auth_key INTEGER
);";

const SELECT_BY_ID: &str = "
SELECT * FROM users WHERE id = ?1;
";

const SELECT_BY_EMAIL: &str = "
SELECT * FROM users WHERE email = ?1;
";
const UPDATE_AUTH_KEY = "
UPDATE users SET auth_key = ?1 WHERE id = ?2;
";


pub struct Users {
    pub conn: DBConn,
    pub sess: 
}


impl TryFrom<Connection> for Users {
    type Error = Error;
    fn try_from(conn: Connection) -> Result<Users> {
        let out = Users {
            conn: Mutex::new(conn),
        };
        out.init()?;
        Ok(out)
    }
}

use std::ops::Deref;
impl DBConnection for Users {
    fn init(&self) -> Result<()> {
        self.execute(CREATE_TABLE, params![])?;
        Ok(())
    }

    pub fn execute<P>(&self, sql: &str, params: P) -> Result<usize>
    where
        P: IntoIterator,
        P::Item: rusqlite::ToSql,
    {
        let db = self.conn.lock()?;
        let out = db.execute(sql, params)?;
        Ok(out)
    }
    pub fn is_auth(&self, cookies: &CookieUser) -> Result<bool> {
        let user = self.get_by_id(cookies.id)?;
        Ok(user.auth_key == Some(cookies.auth_key))
    }

    pub fn signin(&self, form: impl Deref<Target = Signup>) -> Result<()> {
        let email = &form.email;
        let password = &form.password;
        self.create_user(email, password, false)?;
        Ok(())
    }

    pub fn create_user(&self, email: &str, password: &str, is_admin: bool) -> Result<()> {
        let password = password.as_bytes();
        let salt = rand_salt();
        let config = argon2::Config::default();
        let hash = argon2::hash_encoded(password, &salt, &config).unwrap();
        self.execute(CREATE_USER, params![email, hash])?;
        Ok(())
    }

    pub fn login(&self, form: &impl Deref<Target=Login>) -> Result<u32> {
        let email = &form.email;
        let password = &form.password;
        let user = self.get_by_email(email)?;
        let is_verified = argon2::verify_encoded(password, &user.password.as_bytes())?;
        if is_verified {
            let key = self.set_auth_key(user.id)?;
            Ok(key)
        } else {
            raise("Contraseña incorrecta.")
        }
    }
    pub fn logout(&self, cookies: &CookieUser) -> Result<()> {
        if self.is_auth(cookies)? {
            self.execute(
                UPDATE_AUTH_KEY,
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
            ,
            params![auth_key, user_id],
        )?;
        Ok(auth_key)
    }

    pub fn get_by_id(&self, user_id: u32) -> Result<User> {
        let db = self.conn.lock()?;
        let user = db
            .query_row(
                SELECT_BY_ID,
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

    fn get_by_email(&self, email: &str) -> Result<User> {
        let db = self.conn.lock()?;
        let user = db
            .query_row(
                SELECT_BY_EMAIL,
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


fn rand_salt() -> [u8; SALT_LENGTH] {
    let out: [u8; SALT_LENGTH] = rand::random();
    out.into()
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
