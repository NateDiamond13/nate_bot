CREATE TABLE store (
    key TEXT NOT NULL,
    value TEXT NOT NULL,

    PRIMARY KEY (key)
);

CREATE TABLE pictures (
    name TEXT NOT NULL,
    url TEXT NOT NULL,
    added_by_user TEXT NOT NULL,
    is_nsfw BOOLEAN NOT NULL DEFAULT False,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    PRIMARY KEY (name)
);
