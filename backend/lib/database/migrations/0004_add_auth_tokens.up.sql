CREATE TABLE auth_tokens (
    source_site TEXT NOT NULL,
    access_token TEXT NOT NULL,
    token_type TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    refresh_token TEXT NOT NULL,
    scope TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    PRIMARY KEY (source_site)
);
