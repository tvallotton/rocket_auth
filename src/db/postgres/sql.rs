










const CREATE_TABLE: &str =  "
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR (50) UNIQUE NOT NULL,
	password VARCHAR ( 85 ) NOT NULL,
    is_admin BOOL
);
";

