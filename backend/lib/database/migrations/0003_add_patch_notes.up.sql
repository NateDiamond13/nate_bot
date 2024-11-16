CREATE TABLE patch_game_info (
    internal_name TEXT NOT NULL,
    game_title TEXT NOT NULL,
    thumbnail_url TEXT NULL,

    PRIMARY KEY (internal_name)
);

CREATE TABLE patch_notes (
    target_game TEXT NOT NULL,
    patch_id TEXT NOT NULL,
    link TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    posted_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    PRIMARY KEY (target_game, patch_id)
);

CREATE TABLE patch_notes_subscriptions (
    target_game TEXT NOT NULL,
    guild_id TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    webhook_id TEXT NOT NULL,
    webhook_token TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),

    PRIMARY KEY (target_game, guild_id, channel_id)
);
