CREATE TABLE refresh_tokens (
    token           TEXT PRIMARY KEY,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id       TEXT NOT NULL REFERENCES clients(id) ON DELETE CASCADE,
    scope           TEXT NOT NULL,
    expires_at      TIMESTAMPTZ NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_refresh_tokens_user ON refresh_tokens(user_id);

CREATE TABLE authorization_codes (
    code            TEXT PRIMARY KEY,
    user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    client_id       TEXT NOT NULL,
    redirect_uri    TEXT NOT NULL,
    scope           TEXT NOT NULL,
    code_challenge  TEXT,
    expires_at      TIMESTAMPTZ NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
