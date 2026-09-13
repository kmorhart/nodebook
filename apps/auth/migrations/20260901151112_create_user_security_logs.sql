CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';


CREATE TABLE IF NOT EXISTS user_security_logs (
    user_uuid UUID PRIMARY KEY REFERENCES users(uuid) ON DELETE CASCADE,
    
    -- Real-time security state
    recent_failed_login_attempts INT NOT NULL DEFAULT 0,
    total_successful_login_attempts INT NOT NULL DEFAULT 0,
    total_failed_login_attempts INT NOT NULL DEFAULT 0,
    lockout_until TIMESTAMP WITH TIME ZONE,

    -- Timestamps
    last_successful_login_at TIMESTAMP WITH TIME ZONE,
    last_failed_login_at TIMESTAMP WITH TIME ZONE,
    last_password_change_at TIMESTAMP WITH TIME ZONE,
    email_verified_at TIMESTAMP WITH TIME ZONE,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),

    last_successful_login_ip INET,
    last_failed_login_ip INET,
    last_successful_login_user_agent TEXT,
    last_failed_login_user_agent TEXT
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON user_security_logs
FOR EACH ROW
EXECUTE FUNCTION update_updated_at();

CREATE INDEX idx_user_security_logs_last_failed_login_ip ON user_security_logs(last_failed_login_ip);
CREATE INDEX idx_user_security_logs_last_successful_login_ip ON user_security_logs(last_successful_login_ip);