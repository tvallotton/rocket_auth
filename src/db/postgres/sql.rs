










const CREATE_TABLE: &str =  "
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR (50) UNIQUE NOT NULL,
	password VARCHAR ( 85 ) NOT NULL,
    is_admin BOOL
);
";


// pub const INSERT_USER: &str = "
// INSERT INTO users (email, password, is_admin) VALUES (?1, ?2, ?3);
// ";

// pub const UPDATE_USER: &str = "
// UPDATE table SET 
//     email = ?2,
//     password = ?3,
//     is_admin = ?4,
// WHERE
//     id = ?1
// ";

// pub const SELECT_BY_ID: &str = "
// SELECT * FROM users WHERE id = ?1;
// ";

// pub const SELECT_BY_EMAIL: &str = "
// SELECT * FROM users WHERE email = ?1;
// ";

// pub const REMOVE_BY_ID: &str = "
// REMOVE FROM table WHERE id =?1;
// ";
// pub const REMOVE_BY_EMAIL: &str = "
// REMOVE FROM table WHERE email =?1;
// ";