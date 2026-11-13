CREATE TABLE IF NOT EXISTS user_data (
    ref_uuid UUID PRIMARY KEY REFERENCES user_profile(ref_uuid),
    user_uuid UUID NOT NULL,

    settings JSONB NOT NULL DEFAULT '{"dark_mode": true}'::jsonb,
    workspaces JSONB NOT NULL DEFAULT '{"public": [], "private": []}'::jsonb
);

CREATE INDEX idx_user_data_ref_uuid ON user_profile(ref_uuid);
CREATE INDEX idx_user_data_username ON user_profile(username);