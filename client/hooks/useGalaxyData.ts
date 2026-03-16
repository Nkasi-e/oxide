import { useState, useCallback } from "react";
import useSWR from "swr";
import {
  fetchGalaxy,
  fetchRepoMetadata,
  toGalaxyResponse,
  type GalaxyResponse,
  type RepoMetadata
} from "@/utils/api";

interface UseGalaxyDataParams {
  owner: string;
  repo: string;
}

interface UseGalaxyDataResult {
  galaxy: GalaxyResponse | undefined;
  repo: RepoMetadata | undefined;
  isLoading: boolean;
  isError: boolean;
  isRefreshing: boolean;
  /** Trigger re-ingestion from GitHub, then refetch. Call when you want up-to-date repo/contributors. */
  refresh: () => Promise<void>;
}

export function useGalaxyData({ owner, repo }: UseGalaxyDataParams): UseGalaxyDataResult {
  const {
    data: galaxyPayload,
    error: galaxyError,
    isLoading: galaxyLoading,
    mutate: mutateGalaxy
  } = useSWR(
    owner && repo ? ["galaxy", owner, repo] : null,
    () => fetchGalaxy(owner, repo),
    { revalidateOnFocus: false }
  );

  const {
    data: repoData,
    error: repoError,
    isLoading: repoLoading,
    mutate: mutateRepo
  } = useSWR(
    owner && repo ? ["repo", owner, repo] : null,
    () => fetchRepoMetadata(owner, repo),
    { revalidateOnFocus: false }
  );

  const galaxy: GalaxyResponse | undefined =
    galaxyPayload?.status === "ready"
      ? toGalaxyResponse(galaxyPayload.galaxy)
      : undefined;

  const isLoading =
    galaxyLoading ||
    repoLoading ||
    (galaxyPayload != null && galaxyPayload.status === "loading");
  const isError = Boolean(galaxyError || repoError);

  const [isRefreshing, setIsRefreshing] = useState(false);
  const refresh = useCallback(async () => {
    if (!owner || !repo) return;
    setIsRefreshing(true);
    try {
      await fetchGalaxy(owner, repo, { refresh: true });
      // Poll until ingestion finishes (backend returns "ready") or give up after ~10s
      for (let i = 0; i < 6; i++) {
        await new Promise((r) => setTimeout(r, 2000));
        const payload = await fetchGalaxy(owner, repo);
        if (payload?.status === "ready") break;
      }
      await Promise.all([mutateGalaxy(), mutateRepo()]);
    } finally {
      setIsRefreshing(false);
    }
  }, [owner, repo, mutateGalaxy, mutateRepo]);

  return {
    galaxy,
    repo: repoData,
    isLoading,
    isError,
    isRefreshing,
    refresh
  };
}

