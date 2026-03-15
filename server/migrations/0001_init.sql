CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    github_id BIGINT NOT NULL UNIQUE,
    owner TEXT NOT NULL,
    name TEXT NOT NULL,
    full_name TEXT NOT NULL UNIQUE,
    description TEXT,
    stars INT NOT NULL DEFAULT 0,
    forks INT NOT NULL DEFAULT 0,
    language TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_repositories_owner_name ON repositories(owner, name);
CREATE INDEX IF NOT EXISTS idx_repositories_stars ON repositories(stars DESC);

CREATE TABLE IF NOT EXISTS contributors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    github_id BIGINT NOT NULL UNIQUE,
    username TEXT NOT NULL,
    avatar_url TEXT,
    profile_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_contributors_username ON contributors(username);

CREATE TABLE IF NOT EXISTS repo_contributors (
    repo_id UUID NOT NULL,
    contributor_id UUID NOT NULL,
    commits INT NOT NULL DEFAULT 0,
    additions INT NOT NULL DEFAULT 0,
    deletions INT NOT NULL DEFAULT 0,
    first_commit_at TIMESTAMPTZ,
    last_commit_at TIMESTAMPTZ,
    PRIMARY KEY (repo_id, contributor_id),
    CONSTRAINT fk_repo_contributors_repo
        FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE,
    CONSTRAINT fk_repo_contributors_contributor
        FOREIGN KEY (contributor_id) REFERENCES contributors(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_repo_contributors_contributor_id ON repo_contributors(contributor_id);
CREATE INDEX IF NOT EXISTS idx_repo_contributors_commits ON repo_contributors(repo_id, commits DESC);

CREATE TABLE IF NOT EXISTS repo_languages (
    repo_id UUID NOT NULL,
    language TEXT NOT NULL,
    bytes BIGINT NOT NULL DEFAULT 0,
    PRIMARY KEY (repo_id, language),
    CONSTRAINT fk_repo_languages_repo
        FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS galaxies (
    repo_id UUID PRIMARY KEY,
    galaxy_json JSONB NOT NULL,
    version INT NOT NULL,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_galaxies_repo
        FOREIGN KEY (repo_id) REFERENCES repositories(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_galaxies_generated_at ON galaxies(generated_at DESC);
