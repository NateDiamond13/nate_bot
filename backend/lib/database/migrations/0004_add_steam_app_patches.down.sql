ALTER TABLE patch_game_info
    DROP COLUMN IF EXISTS created_at,
    DROP COLUMN IF EXISTS steam_app_id;
