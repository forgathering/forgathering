CREATE TABLE users (
    id TEXT PRIMARY KEY,

    created_at INTEGER NOT NULL,

    username TEXT UNIQUE NOT NULL,
    nickname TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,

    avatar BLOB,
    bio TEXT NOT NULL,
    links TEXT[] NOT NULL,

    users_followed TEXT[] NOT NULL,

    collections_followed TEXT[] NOT NULL,

    landing_page TEXT NOT NULL,
    show_nsfw BOOLEAN NOT NULL
);

CREATE TABLE external_users (
	id TEXT PRIMARY KEY,

	server TEXT NOT NULL
);

CREATE TABLE sessions (
	id TEXT PRIMARY KEY,
	user_id TEXT NOT NULL,

	expires_at INTEGER NOT NULL
);

CREATE TABLE collections (
	id TEXT PRIMARY KEY,

	created_at INTEGER NOT NULL,
	modified_at INTEGER NOT NULL,

	owner_id TEXT NOT NULL,
	private BOOLEAN NOT NULL,
	collaborators TEXT[] NOT NULL,

	name TEXT NOT NULL,

	important_blocks TEXT[] NOT NULL,
	blocks TEXT[] NOT NULL
);

CREATE TABLE external_collections (
	id TEXT PRIMARY KEY,

	server TEXT NOT NULL
);

CREATE TABLE blocks (
	id UUID PRIMARY KEY,

	created_at INTEGER NOT NULL,
	modified_at INTEGER NOT NULL,

	markdown TEXT,
	link TEXT,
	image BLOB
);

CREATE TABLE external_blocks (
	id TEXT PRIMARY KEY,

	server TEXT NOT NULL
);
