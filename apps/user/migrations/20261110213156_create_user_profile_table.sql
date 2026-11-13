CREATE TABLE IF NOT EXISTS user_profile (
    ref_uuid UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_uuid UUID NOT NULL,

    username VARCHAR(32) NOT NULL UNIQUE,
    firstname VARCHAR(32) DEFAULT 'Nodebook',
    lastname VARCHAR(32) DEFAULT 'User',
    bio VARCHAR(500) DEFAULT 'User has not set a bio yet.'
);

CREATE INDEX idx_user_profile_ref_uuid ON user_profile(ref_uuid);
CREATE INDEX idx_user_profile_username ON user_profile(username);