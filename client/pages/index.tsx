import { useState } from "react";
import Head from "next/head";
import { useRouter } from "next/router";
import { searchRepos, type SearchResult } from "@/utils/api";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

export default function HomePage() {
  const router = useRouter();
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<SearchResult[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSearch(e: React.FormEvent) {
    e.preventDefault();
    if (!query.trim()) return;
    try {
      setIsSearching(true);
      setError(null);
      const data = await searchRepos(query.trim());
      setResults(data.status === "ready" ? data.items : []);
    } catch (err) {
      setError((err as Error).message);
    } finally {
      setIsSearching(false);
    }
  }

  return (
    <>
      <Head>
        <title>GitHub Galaxy · Oxide</title>
        <meta name="description" content="Explore GitHub repositories as 3D galaxies. Stars are repos, planets are contributors." />
      </Head>
      <main className="relative flex min-h-screen flex-col">
        <div className="mx-auto flex w-full max-w-3xl flex-1 flex-col px-4 pt-16 pb-20 md:px-6 lg:px-8">
          {/* Hero */}
          <header className="mb-12 text-center animate-fade-in-up">
            <div className="mb-4 inline-flex items-center gap-2 rounded-full border border-border bg-surface/60 px-4 py-1.5 text-xs font-medium uppercase tracking-widest text-muted">
              <span className="h-1.5 w-1.5 rounded-full bg-accent shadow-glow-sm" />
              Oxide
            </div>
            <h1 className="text-balance text-4xl font-bold tracking-tight text-zinc-50 md:text-5xl md:leading-[1.15]">
              Repositories as{" "}
              <span className="text-accent">galaxies</span>
            </h1>
            <p className="mx-auto mt-4 max-w-xl text-base leading-relaxed text-zinc-400 md:text-lg">
              Search any GitHub repo. We turn it into a 3D galaxy: the repo is the star, top contributors are planets, the rest form the asteroid belt.
            </p>
          </header>

          {/* Search */}
          <section className="mb-14 animate-fade-in-up" style={{ animationDelay: "0.1s" }}>
            <form
              onSubmit={handleSearch}
              className="rounded-2xl border border-border bg-surface/80 p-5 shadow-card backdrop-blur-sm md:p-6"
            >
              <div className="flex flex-col gap-3 sm:flex-row sm:items-center">
                <Input
                  value={query}
                  onChange={(e) => setQuery(e.target.value)}
                  placeholder="owner/repo or search terms…"
                  className="flex-1 text-base"
                  autoFocus
                />
                <Button
                  type="submit"
                  className="sm:w-auto sm:min-w-[120px]"
                  disabled={isSearching}
                >
                  {isSearching ? "Searching…" : "Explore"}
                </Button>
              </div>
            </form>
          </section>

          {/* Results / empty / error */}
          <section className="flex-1 space-y-4">
            {error && (
              <div className="rounded-xl border border-danger/30 bg-danger/5 px-4 py-3 text-sm text-danger animate-fade-in">
                {error}
              </div>
            )}

            {results.length > 0 && (
              <div className="grid gap-4 sm:grid-cols-2 animate-fade-in">
                {results.map((repo, i) => (
                  <Card
                    key={repo.full_name}
                    className="cursor-pointer animate-fade-in-up"
                    style={{ animationDelay: `${Math.min(i * 0.03, 0.3)}s` }}
                    onClick={() => {
                      const [owner, name] = repo.full_name.split("/");
                      void router.push(`/repo/${owner}/${name}`);
                    }}
                  >
                    <CardHeader className="flex flex-row items-start justify-between gap-3">
                      <CardTitle className="font-mono text-sm font-semibold text-zinc-200 truncate">
                        {repo.full_name}
                      </CardTitle>
                      <span className="shrink-0 text-xs text-muted">
                        ★ {(repo.stars ?? 0).toLocaleString()} · {repo.forks ?? 0} forks
                      </span>
                    </CardHeader>
                    <CardContent className="pt-0">
                      <p className="line-clamp-2 text-sm leading-relaxed text-zinc-400">
                        {repo.description ?? "No description."}
                      </p>
                      {repo.language && (
                        <span className="mt-3 inline-block rounded-md bg-background/80 px-2 py-0.5 font-mono text-xs text-muted">
                          {repo.language}
                        </span>
                      )}
                    </CardContent>
                  </Card>
                ))}
              </div>
            )}

            {!results.length && !error && (
              <div className="flex flex-col items-center justify-center py-20 text-center">
                <div className="mb-4 flex h-16 w-16 items-center justify-center rounded-2xl border border-border bg-surface/80 text-3xl">
                  ✦
                </div>
                <p className="text-sm text-muted">
                  Search for a repository to generate its galaxy.
                </p>
              </div>
            )}
          </section>
        </div>
      </main>
    </>
  );
}
