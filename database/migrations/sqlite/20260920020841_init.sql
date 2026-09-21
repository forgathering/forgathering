CREATE TABLE users (
    id TEXT PRIMARY KEY,
    created_at INTEGER NOT NULL,

    username TEXT UNIQUE NOT NULL,
    nickname TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,

    avatar BLOB DEFAULT NULL,
    bio TEXT,
    links TEXT[],
    following TEXT[],

    landing_page TEXT DEFAULT '/',
    show_nsfw BOOLEAN DEFAULT false
);

CREATE TABLE external_users (
	id TEXT PRIMARY KEY,

	server TEXT NOT NULL
);

CREATE TABLE sessions (
	session_id TEXT PRIMARY KEY,
	user_id TEXT NOT NULL,

	expires_at INTEGER NOT NULL
);
