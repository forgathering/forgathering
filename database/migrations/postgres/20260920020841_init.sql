CREATE TABLE users (
    id UUID PRIMARY KEY,

    created_at TIMESTAMPTZ NOT NULL,

    username VARCHAR(255) UNIQUE NOT NULL,
    nickname VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,

    avatar BYTEA,
    bio VARCHAR(8192) NOT NULL,
    links VARCHAR(255)[] NOT NULL,

    users_followed UUID[] NOT NULL,

    collections_followed UUID[] NOT NULL,

    landing_page VARCHAR(1024) NOT NULL,
    show_nsfw BOOLEAN NOT NULL
);

CREATE TABLE external_users (
	id UUID PRIMARY KEY,

	server VARCHAR(8192) NOT NULL
);

CREATE TABLE sessions (
	id UUID PRIMARY KEY,
	user_id UUID NOT NULL,

	expires_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE collections (
	id UUID PRIMARY KEY,

	created_at TIMESTAMPTZ NOT NULL,
	modified_at TIMESTAMPTZ NOT NULL,

	owner_id UUID NOT NULL,
	private BOOLEAN NOT NULL,
	collaborators UUID[] NOT NULL,

	name VARCHAR(255) NOT NULL,

	important_blocks UUID[] NOT NULL,
	blocks UUID[] NOT NULL
);

CREATE TABLE external_collections (
	id UUID PRIMARY KEY,

	server VARCHAR(8192) NOT NULL
);

CREATE TABLE blocks (
	id UUID PRIMARY KEY,

	created_at TIMESTAMPTZ NOT NULL,
	modified_at TIMESTAMPTZ NOT NULL,

	markdown VARCHAR(8192),
	link VARCHAR(8192),
	image BYTEA
);

CREATE TABLE external_blocks (
	id UUID PRIMARY KEY,

	server VARCHAR(8192) NOT NULL
);
