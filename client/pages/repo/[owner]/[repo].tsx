import { useRouter } from "next/router";
import Head from "next/head";
import dynamic from "next/dynamic";
import { useMemo, useState } from "react";
import { useGalaxyData } from "@/hooks/useGalaxyData";
import { RepoInfo } from "@/components/RepoInfo";
import type { GalaxyPlanet } from "@/utils/api";

const GalaxyScene = dynamic(
  () =>
    import("@/components/GalaxyScene").then((mod) => mod.GalaxyScene),
  {
    ssr: false,
    loading: () => (
      <div className="flex h-full min-h-[360px] items-center justify-center rounded-2xl border border-border bg-surface/50">
        <span className="text-sm text-muted">Loading scene…</span>
      </div>
    ),
  }
);

export default function RepoGalaxyPage() {
  const router = useRouter();
  const { owner, repo } = router.query as { owner?: string; repo?: string };

  const valid = useMemo(
    () =>
      Boolean(
        owner &&
        repo &&
        typeof owner === "string" &&
        typeof repo === "string"
      ),
    [owner, repo]
  );

  const { galaxy, repo: repoMeta, isLoading, isError, isRefreshing, refresh } = useGalaxyData({
    owner: owner ?? "",
    repo: repo ?? "",
  });

  const [activeContributor, setActiveContributor] =
    useState<GalaxyPlanet | null>(null);

  const pageTitle = repoMeta
    ? `${repoMeta.full_name} · Galaxy`
    : owner && repo
      ? `${owner}/${repo} · Galaxy`
      : "Galaxy";

  if (!valid) {
    return null;
  }

  return (
    <>
      <Head>
        <title>{pageTitle}</title>
      </Head>
      <main className="relative flex min-h-screen flex-col">
        {/* Top bar */}
        <header className="sticky top-0 z-10 flex items-center justify-between gap-4 border-b border-border bg-background/80 px-4 py-3 backdrop-blur-md md:px-6">
          <button
            onClick={() => router.push("/")}
            className="text-sm text-muted transition-colors hover:text-zinc-100"
          >
            ← Search
          </button>
          <div className="flex items-center gap-3">
            <button
              type="button"
              onClick={() => refresh()}
              disabled={isRefreshing}
              className="rounded-lg border border-border bg-surface/80 px-3 py-1.5 text-xs font-medium text-muted transition-colors hover:bg-surface hover:text-zinc-100 disabled:opacity-60"
              title="Re-fetch from GitHub and update repo & galaxy"
            >
              {isRefreshing ? "Refreshing…" : "Refresh data"}
            </button>
            <p className="truncate font-mono text-xs text-muted md:text-sm">
              {owner}/{repo}
            </p>
          </div>
        </header>

        {/* Main: canvas + sidebar */}
        <section className="grid flex-1 gap-4 p-4 md:grid-cols-[1fr_320px] md:gap-6 md:p-6 lg:grid-cols-[minmax(0,1fr)_360px]">
          <div className="relative min-h-[360px] md:min-h-[480px] lg:min-h-[560px]">
            {isLoading && (
              <div className="flex h-full flex-col items-center justify-center gap-3 rounded-2xl border border-border bg-surface/50">
                <div className="h-8 w-8 animate-pulse rounded-full border-2 border-accent/50 border-t-accent" />
                <p className="text-sm text-muted">Building galaxy…</p>
              </div>
            )}
            {isError && (
              <div className="flex h-full flex-col items-center justify-center gap-3 rounded-2xl border border-danger/30 bg-danger/5 p-6 text-center">
                <p className="text-sm text-danger">
                  Couldn’t load this repository. Check that it exists and the backend is running.
                </p>
              </div>
            )}
            {galaxy && (
              <GalaxyScene
                galaxy={galaxy}
                onSelectContributor={(c) => setActiveContributor(c)}
              />
            )}
          </div>

          <aside className="flex min-h-[260px] flex-col md:min-h-0">
            {repoMeta && (
              <RepoInfo
                repo={repoMeta}
                galaxy={galaxy ?? null}
                activeContributor={activeContributor}
              />
            )}
          </aside>
        </section>
      </main>
    </>
  );
}
