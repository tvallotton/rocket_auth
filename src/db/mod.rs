#[cfg(feature = "postgres-db")]
mod postgres;

#[cfg(feature = "sqlite-db")]
mod sqlite;

#[cfg(feature = "tokio-postgres-db")]
mod tokio_postgres;

use crate::prelude::*;

#[rocket::async_trait]
pub trait DBConnection: Send + Sync {
    async fn init(&self) -> Result<()>;
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error>;
    async fn update_user(&self, user: &User) -> Result<()>;
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()>;
    async fn delete_user_by_email(&self, email: &str) -> Result<()>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<User>;
    async fn get_user_by_email(&self, email: &str) -> Result<User>;
}

#[rocket::async_trait]
impl<T: DBConnection> DBConnection for std::sync::Arc<T> {
    async fn init(&self) -> Result<()> {
        T::init(self).await
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        T::create_user(self, email, hash, is_admin).await
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        T::update_user(self, user).await
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        T::delete_user_by_id(self, user_id).await
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        T::delete_user_by_email(self, email).await
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        T::get_user_by_id(self, user_id).await
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        T::get_user_by_email(self, email).await
    }
}

// #[rocket::async_trait]
// impl<T: DBConnection> DBConnection for tokio::sync::RwLock<T> {
//     async fn init(&self) -> Result<()> {
//         self.init().await
//     }
//     async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
//         self.create_user(email, hash, is_admin).await
//     }
//     async fn update_user(&self, user: &User) -> Result<()> {
//         self.update_user(user).await
//     }
//     async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
//         self.delete_user_by_id(user_id).await
//     }
//     async fn delete_user_by_email(&self, email: &str) -> Result<()> {
//         self.delete_user_by_email(email).await
//     }
//     async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
//         self.get_user_by_id(user_id).await
//     }
//     async fn get_user_by_email(&self, email: &str) -> Result<User> {
//         self.get_user_by_email(email).await
//     }
// }


#[rocket::async_trait]
impl<T: DBConnection> DBConnection for tokio::sync::Mutex<T> {
    async fn init(&self) -> Result<()> {
        self.init().await
    }
    async fn create_user(&self, email: &str, hash: &str, is_admin: bool) -> Result<(), Error> {
        self.lock().await.create_user(email, hash, is_admin).await
    }
    async fn update_user(&self, user: &User) -> Result<()> {
        self.lock().await.update_user(user).await
    }
    async fn delete_user_by_id(&self, user_id: i32) -> Result<()> {
        self.lock().await.delete_user_by_id(user_id).await
    }
    async fn delete_user_by_email(&self, email: &str) -> Result<()> {
        self.lock().await.delete_user_by_email(email).await
    }
    async fn get_user_by_id(&self, user_id: i32) -> Result<User> {
        self.lock().await.get_user_by_id(user_id).await
    }
    async fn get_user_by_email(&self, email: &str) -> Result<User> {
        self.lock().await.get_user_by_email(email).await
    }
}

