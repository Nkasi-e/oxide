// Base URL for the Rust backend, as seen from the browser.
// - In normal local dev, leave NEXT_PUBLIC_API_BASE_URL unset so that
//   requests go to `/api/*` on the same origin, and Next.js rewrites
//   proxy them to the Rust server (see `next.config.mjs`).
// - If you deploy the backend separately (different origin), set
//   NEXT_PUBLIC_API_BASE_URL to that full URL.
const API_BASE_URL =
  process.env.NEXT_PUBLIC_API_BASE_URL &&
  process.env.NEXT_PUBLIC_API_BASE_URL.length > 0
    ? process.env.NEXT_PUBLIC_API_BASE_URL
    : "";

export interface GalaxyContributor {
  id: string;
  username: string;
  avatar_url: string | null;
  commits: number;
  rank: number;
}

export interface GalaxyPlanet extends GalaxyContributor {
  radius: number;
  orbitRadius: number;
  angle: number;
}

export interface AsteroidBelt {
  count: number;
  innerRadius: number;
  outerRadius: number;
}

export interface GalaxyStar {
  name: string;
  popularityScore: number;
  radius: number;
}

/** Layout shape the 3D scene expects (after transform). */
export interface GalaxyResponse {
  star: GalaxyStar;
  planets: GalaxyPlanet[];
  asteroid_belt: AsteroidBelt;
}

/** Raw backend galaxy response (wrapped, ready vs loading). */
export type GalaxyApiResponse =
  | {
      status: "ready";
      version: number;
      generated_at: string;
      galaxy: BackendGalaxyLayout;
    }
  | { status: "loading"; message: string; owner: string; repo: string };

/** Backend layout (star/planets/asteroid_belt from Rust). */
export interface BackendGalaxyLayout {
  star: { name: string; size: number; brightness: number };
  planets: Array<{
    username: string;
    size: number;
    position: [number, number, number];
    commits: number;
  }>;
  asteroid_belt: { count: number };
}

/** Map backend layout to scene shape (radius, orbitRadius, angle, id). */
export function toGalaxyResponse(raw: BackendGalaxyLayout): GalaxyResponse {
  return {
    star: {
      name: raw.star.name,
      popularityScore: raw.star.brightness,
      radius: Math.min(3.5, Math.max(1.5, raw.star.size * 0.5))
    },
    planets: raw.planets.map((p, i) => {
      const [x, y] = p.position;
      const orbitRadius = Math.sqrt(x * x + y * y) || 1;
      const angle = Math.atan2(y, x);
      return {
        id: `planet-${i}-${p.username}`,
        username: p.username,
        avatar_url: null,
        commits: p.commits,
        rank: i + 1,
        radius: Math.max(0.15, Math.min(1.2, p.size * 0.18)),
        orbitRadius,
        angle
      };
    }),
    asteroid_belt: {
      count: raw.asteroid_belt.count,
      innerRadius: 20,
      outerRadius: 45
    }
  };
}

export interface RepoMetadata {
  full_name: string;
  description: string | null;
  stargazers_count: number;
  forks_count: number;
  open_issues_count: number;
  owner: {
    login: string;
    avatar_url: string | null;
  };
  html_url: string;
  language: string | null;
  last_synced_at?: string;
}

/** Backend repo response (Rust API). */
interface BackendRepoResponse {
  owner: string;
  name: string;
  full_name: string;
  description: string | null;
  stars: number;
  forks: number;
  open_issues?: number;
  language: string | null;
  last_synced_at?: string;
}

function toRepoMetadata(raw: BackendRepoResponse): RepoMetadata {
  return {
    full_name: raw.full_name,
    description: raw.description,
    stargazers_count: raw.stars,
    forks_count: raw.forks,
    open_issues_count: raw.open_issues ?? 0,
    owner: { login: raw.owner, avatar_url: null },
    html_url: `https://github.com/${raw.owner}/${raw.name}`,
    language: raw.language,
    last_synced_at: raw.last_synced_at
  };
}

/** One repo in search results (matches backend SearchItemResponse). */
export interface SearchResult {
  id: number;
  full_name: string;
  description: string | null;
  language: string | null;
  stars: number;
  forks: number;
}

/** Backend search response. */
export interface SearchApiResponse {
  status: "ready" | "loading";
  message: string | null;
  items: SearchResult[];
}

async function handleResponse<T>(res: Response): Promise<T> {
  if (!res.ok) {
    const text = await res.text();
    throw new Error(
      `API error ${res.status}: ${res.statusText} - ${text || "no body"}`
    );
  }
  return (await res.json()) as T;
}

export async function fetchGalaxy(
  owner: string,
  repo: string,
  options?: { refresh?: boolean }
): Promise<GalaxyApiResponse> {
  const url =
    `${API_BASE_URL}/api/galaxy/${owner}/${repo}` +
    (options?.refresh ? "?refresh=true" : "");
  const res = await fetch(url);
  return handleResponse<GalaxyApiResponse>(res);
}

export async function fetchRepoMetadata(
  owner: string,
  repo: string
): Promise<RepoMetadata> {
  const res = await fetch(`${API_BASE_URL}/api/repo/${owner}/${repo}`);
  const raw = await handleResponse<BackendRepoResponse>(res);
  return toRepoMetadata(raw);
}

export async function searchRepos(query: string): Promise<SearchApiResponse> {
  const res = await fetch(
    `${API_BASE_URL}/api/search?q=${encodeURIComponent(query)}`
  );
  return handleResponse<SearchApiResponse>(res);
}

