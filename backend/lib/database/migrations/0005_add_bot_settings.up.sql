CREATE TABLE bot_settings (
    application_id TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    activity TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (application_id)
);
