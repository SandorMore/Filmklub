CREATE TABLE USERS (
    id PRIMARY KEY NOT NULL
    username TEXT NOT NULL
    password_hash TEXT NOT NULL
    created_at TEXT NOT NULL DEFAULT (datetime("now"))
)