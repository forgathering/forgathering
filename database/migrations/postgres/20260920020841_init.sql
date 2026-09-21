CREATE TABLE users (
    id UUID PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,

    username VARCHAR(255) UNIQUE NOT NULL,
    nickname VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,

    avatar BYTEA DEFAULT NULL,
    bio VARCHAR(4096),
    links VARCHAR(255)[],
    following UUID[],

    landing_page VARCHAR(1024) DEFAULT '/',
    show_nsfw BOOLEAN DEFAULT false
);

CREATE TABLE external_users (
	id UUID PRIMARY KEY,

	server VARCHAR(4096) NOT NULL
);

CREATE TABLE sessions (
	session_id UUID PRIMARY KEY,
	user_id UUID NOT NULL,

	expires_at TIMESTAMPTZ NOT NULL
);
