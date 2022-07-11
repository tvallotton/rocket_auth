pub mod auth;
mod user_impl;
mod users;
use crate::prelude::*;
use argon2::verify_encoded as verify;
use rand::{distributions::Alphanumeric, Rng};
#[cfg(feature = "rusqlite")]
use rusqlite::Error::SqliteFailure;
#[cfg(feature = "tokio-postgres")]
use tokio_postgres::error::SqlState;

pub fn rand_string(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

impl Users {
    async fn is_auth(&self, session: &Session) -> bool {
        self.user_id(session).await.is_some()
    }
    async fn user_id(&self, session: &Session) -> Option<i32> {
        let auth = session.auth()?;
        self.sess.get(&auth.session_id).await?
    }

    #[throws(Error)]
    async fn login(&self, form: &Login) -> String {
        let form_pwd = &form.password.as_bytes();
        let user = self
            .conn
            .get_user_by_email(&form.email.to_lowercase())
            .await
            .map_err(|_| ValidationError::UserNotFound(form.email.clone()))?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            self.set_auth_key(user.id).await?
        } else {
            throw!(Error::Unauthorized)
        }
    }
    #[throws(Error)]
    async fn logout(&self, session: &Session) {
        if self.is_auth(session).await {
            self.sess.destroy(session.session_id()).await;
        }
    }

    #[throws(Error)]
    async fn set_auth_key_for(&self, user_id: i32, time: Duration) -> String {
        let session_id = rand_string(32);
        self.sess.create_auth(&session_id, user_id, time).await?;
        session_id
    }

    #[throws(Error)]
    async fn set_auth_key(&self, user_id: i32) -> String {
        let duration = 7 * 24 * 60 * 60;
        let duration = Duration::from_secs(duration);
        self.set_auth_key_for(user_id, duration).await?
    }

    #[throws(Error)]
    async fn signup(&self, form: &Signup) {
        form.validate()?;
        let email = &form.email.to_lowercase();
        let password = &form.password;
        let result = self.create_user(email, password, false).await;
        match result {
            Ok(_) => (),
            #[cfg(feature = "sqlx")]
            Err(Server(SQLx(sqlx::Error::Database(error))))
                if Some("23000") == error.code().as_deref() =>
            {
                throw!(ValidationError::EmailAlreadyExists(form.email.clone()))
            }
            #[cfg(feature = "tokio-postgres")]
            Err(Server(TokioPostgres(error)))
                if Some(&SqlState::UNIQUE_VIOLATION) == error.code() =>
            {
                throw!(EmailAlreadyExists(form.email.clone()))
            }
            #[cfg(feature = "rusqlite")]
            Err(Server(Rusqlite(SqliteFailure(error, _)))) // .
                if error.extended_code == 2067 => 
            {
                throw!(EmailAlreadyExists(form.email.clone()))
            }
            Err(error) => {
                throw!(error)
            }
        }
    }

    #[throws(Error)]
    async fn login_for(&self, form: &Login, time: Duration) -> String {
        let form_pwd = &form.password.as_bytes();
        let user = self
            .conn
            .get_user_by_email(&form.email.to_lowercase())
            .await?;
        let user_pwd = &user.password;
        if verify(user_pwd, form_pwd)? {
            self.set_auth_key_for(user.id, time).await?
        } else {
            throw!(Error::Unauthorized)
        }
    }
}
