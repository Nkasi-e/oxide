import Image from "next/image";
import type { GalaxyPlanet, GalaxyResponse, RepoMetadata } from "@/utils/api";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

interface RepoInfoProps {
  repo: RepoMetadata;
  galaxy?: GalaxyResponse | null;
  activeContributor?: GalaxyPlanet | null;
}

function formatRelativeTime(iso: string): string {
  try {
    const d = new Date(iso);
    const now = new Date();
    const sec = Math.floor((now.getTime() - d.getTime()) / 1000);
    if (sec < 60) return "just now";
    if (sec < 3600) return `${Math.floor(sec / 60)}m ago`;
    if (sec < 86400) return `${Math.floor(sec / 3600)}h ago`;
    if (sec < 2592000) return `${Math.floor(sec / 86400)}d ago`;
    return d.toLocaleDateString();
  } catch {
    return "";
  }
}

export function RepoInfo({ repo, galaxy, activeContributor }: RepoInfoProps) {
  const stars = (repo.stargazers_count ?? 0).toLocaleString();
  const forks = (repo.forks_count ?? 0).toLocaleString();
  const issues = (repo.open_issues_count ?? 0).toLocaleString();
  const totalContributors = galaxy
    ? galaxy.planets.length + (galaxy.asteroid_belt?.count ?? 0)
    : null;

  return (
    <div className="flex h-full flex-col gap-4">
      {/* Repo card */}
      <Card className="flex-1 min-h-0">
        <CardHeader className="flex flex-row items-center gap-4 pb-4">
          {repo.owner?.avatar_url && (
            <Image
              src={repo.owner.avatar_url}
              alt={repo.owner.login ?? "Owner"}
              width={48}
              height={48}
              className="rounded-xl border border-border"
            />
          )}
          <div className="min-w-0 flex-1">
            <CardTitle className="truncate font-mono text-sm font-semibold text-zinc-100">
              {repo.full_name}
            </CardTitle>
            <p className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-0 text-xs text-muted">
              <span>★ {stars}</span>
              <span>·</span>
              <span>{forks} forks</span>
              <span>·</span>
              <span>{issues} issues</span>
            </p>
          </div>
        </CardHeader>
        <CardContent className="space-y-4 pt-0">
          {repo.description && (
            <p className="text-sm leading-relaxed text-zinc-400 line-clamp-3">
              {repo.description}
            </p>
          )}
          <div className="flex flex-wrap items-center gap-2">
            {repo.language && (
              <span className="rounded-lg bg-background/80 px-2.5 py-1 font-mono text-xs text-muted">
                {repo.language}
              </span>
            )}
            {repo.last_synced_at && (
              <span className="text-[10px] text-muted">
                Synced {formatRelativeTime(repo.last_synced_at)}
              </span>
            )}
            <a
              href={repo.html_url}
              target="_blank"
              rel="noreferrer"
              className="inline-flex items-center rounded-lg border border-border bg-surface/80 px-3 py-1.5 text-xs font-medium text-link transition-colors hover:border-link/50 hover:bg-surface"
            >
              Open on GitHub →
            </a>
          </div>
        </CardContent>
      </Card>

      {totalContributors !== null && galaxy && (
        <Card className="shrink-0">
          <CardHeader className="pb-2">
            <CardTitle className="text-xs font-medium uppercase tracking-wider text-muted">
              Galaxy
            </CardTitle>
          </CardHeader>
          <CardContent className="pt-0">
            <div className="grid grid-cols-2 gap-2 text-sm">
              <div>
                <p className="text-[10px] uppercase text-muted">Contributors</p>
                <p className="font-semibold tabular-nums text-zinc-100">
                  {totalContributors.toLocaleString()}
                </p>
              </div>
              <div>
                <p className="text-[10px] uppercase text-muted">Planets (top)</p>
                <p className="font-semibold tabular-nums text-zinc-100">
                  {galaxy.planets.length}
                </p>
              </div>
              <div>
                <p className="text-[10px] uppercase text-muted">In belt</p>
                <p className="font-semibold tabular-nums text-zinc-100">
                  {(galaxy.asteroid_belt?.count ?? 0).toLocaleString()}
                </p>
              </div>
              <div>
                <p className="text-[10px] uppercase text-muted">Star</p>
                <p className="truncate font-mono text-xs text-zinc-300">
                  {galaxy.star.name}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      )}

      {/* Contributor focus */}
      <Card className="shrink-0">
        <CardHeader className="pb-2">
          <CardTitle className="text-xs font-medium uppercase tracking-wider text-muted">
            Contributor
          </CardTitle>
        </CardHeader>
        <CardContent className="pt-0">
          {activeContributor ? (
            <div className="space-y-2">
              <p className="font-mono text-sm font-semibold text-accent">
                @{activeContributor.username}
              </p>
              <p className="text-sm text-zinc-400">
                <span className="text-zinc-300">{(activeContributor.commits ?? 0).toLocaleString()}</span> commits
                <span className="text-muted"> · rank #{activeContributor.rank}</span>
              </p>
            </div>
          ) : (
            <p className="text-sm text-muted">
              Hover or click a planet to see contributor details.
            </p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
