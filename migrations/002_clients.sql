CREATE TABLE clients (
    id              TEXT PRIMARY KEY,
    secret_hash     TEXT NOT NULL,
    name            TEXT NOT NULL,
    redirect_uris   TEXT[] NOT NULL,
    allowed_scopes  TEXT[] NOT NULL,
    grant_types     TEXT[] NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
