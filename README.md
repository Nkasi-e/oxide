## Oxide

Oxide is a 3D visualization of GitHub repositories and their contributors, rendered as galaxies in space. Stars are repositories, planets are contributors, and an asteroid belt represents everyone beyond the top contributors.

This repo contains:

- `client/` – Three.js (or similar) frontend that renders galaxies.
- `server/` – Rust/Axum backend that talks to the GitHub API, stores data in Postgres, and generates deterministic galaxy layouts.

### How it works (short version)

- You ask the backend for a galaxy: `GET /api/galaxy/{owner}/{repo}`.
- The server:
  - Pulls repo + contributor + language data from the GitHub API.
  - Normalizes and stores it in Postgres.
  - Generates a deterministic layout:
    - Repo → central star.
    - Top 50 contributors → planets (log‑scaled sizes, rank‑based orbits, hashed positions).
    - Everyone else → `asteroid_belt.count` for the frontend to visualize densely without killing the GPU.
- The frontend turns that JSON into a 3D scene.

### Architecture

```mermaid
flowchart LR
  subgraph CLIENT["client (Next.js + R3F)"]
    U["Browser"]
    P["Repo page<br/>useGalaxyData · useSWR"]
    GS["GalaxyScene<br/>Star · Planet · AsteroidBelt"]
    SI["RepoInfo sidebar"]
    U --> P
    P --> GS
    P --> SI
  end

  subgraph SERVER["server (Rust / Axum)"]
    H["HTTP handlers<br/>/api/galaxy · /api/repo · /api/search"]
    GX["Galaxy service<br/>layout generator"]
    RS["Repo service"]
    CS["Contributor service"]
    Q["Ingestion queue"]
    W["Worker loop"]
    H --> GX
    H --> RS
    GX --> CS
    GX --> Q
    Q --> W
    W --> RS
    W --> CS
    W --> GX
  end

  subgraph DB["Postgres"]
    R[("repositories")]
    C[("contributors")]
    RC[("repo_contributors")]
    RL[("repo_languages")]
    GA[("galaxies")]
  end

  GH["GitHub REST API"]

  P -->|"GET /api/* proxy"| H
  H -->|"galaxy JSON"| P
  RS --- R
  RS --- RL
  CS --- C
  CS --- RC
  GX --- GA
  W -->|"fetch repo · contributors · languages"| GH
  GH -->|"raw data"| W
```

### Database ER diagram

```mermaid
erDiagram
  REPOSITORIES {
    uuid id PK
    bigint github_id UK
    text owner
    text name
    text full_name UK
    text description
    int stars
    int forks
    int open_issues
    text language
    timestamptz created_at
    timestamptz updated_at
    timestamptz last_synced_at
  }

  CONTRIBUTORS {
    uuid id PK
    bigint github_id UK
    text username
    text avatar_url
    text profile_url
    timestamptz created_at
  }

  REPO_CONTRIBUTORS {
    uuid repo_id FK
    uuid contributor_id FK
    int commits
    int additions
    int deletions
    timestamptz first_commit_at
    timestamptz last_commit_at
    string composite_pk
  }

  REPO_LANGUAGES {
    uuid repo_id FK
    text language
    bigint bytes
    string composite_pk
  }

  GALAXIES {
    uuid repo_id PK,FK
    jsonb galaxy_json
    int version
    timestamptz generated_at
  }

  REPOSITORIES ||--o{ REPO_CONTRIBUTORS : has
  CONTRIBUTORS ||--o{ REPO_CONTRIBUTORS : contributes_to
  REPOSITORIES ||--o{ REPO_LANGUAGES : uses
  REPOSITORIES ||--|| GALAXIES : renders_as
```

### Running the backend

From `server/`:

```bash
cp .env.example .env
# edit .env and set a real GitHub token + DATABASE_URL
cargo run
```

Key endpoints (default base URL: `http://localhost:8080`):

- `GET /api/galaxy/{owner}/{repo}` – galaxy layout JSON (`star`, `planets`, `asteroid_belt`).
- `GET /api/repo/{owner}/{repo}` – cached repository metadata.
- `GET /api/search?q=query` – search GitHub repos, cached into Postgres.

### Tech stack

- **Language**: Rust (async)
- **Web**: Axum + Tokio
- **DB**: Postgres + SQLx migrations
- **HTTP client**: Reqwest (GitHub API)
- **Config**: `.env` via `dotenvy`
- **Logging**: `tracing`

The backend is intentionally lean: one binary, one worker loop, one Postgres schema. No distributed queue yet; if that ever changes, it should still feel like this README.

