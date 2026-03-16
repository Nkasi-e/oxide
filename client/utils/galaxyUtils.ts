import type { GalaxyPlanet, GalaxyStar } from "./api";

// Logarithmic scaling to keep sizes within a cinematic but readable range.
export function scalePlanetRadius(commits: number): number {
  if (commits <= 0) return 0.15;
  const base = Math.log10(commits + 1);
  return 0.25 + base * 0.15;
}

export function scaleStarRadius(popularityScore: number): number {
  if (popularityScore <= 0) return 1.5;
  const base = Math.log10(popularityScore + 1);
  return 1.5 + base * 0.3;
}

export function buildStar(
  name: string,
  stars: number,
  forks: number
): GalaxyStar {
  const popularityScore = stars * 2 + forks;
  return {
    name,
    popularityScore,
    radius: scaleStarRadius(popularityScore)
  };
}

// Deterministic angle based on contributor id.
export function contributorAngle(id: string): number {
  let hash = 0;
  for (let i = 0; i < id.length; i += 1) {
    hash = (hash * 31 + id.charCodeAt(i)) | 0;
  }
  const normalized = (hash >>> 0) / 2 ** 32;
  return normalized * Math.PI * 2;
}

export function orbitRadiusFromRank(rank: number): number {
  const baseRadius = 4;
  const spacing = 0.7;
  return baseRadius + (rank - 1) * spacing;
}

export function enhancePlanets<T extends { id: string; commits: number; rank: number }>(
  raw: T[]
): GalaxyPlanet[] {
  return raw.map((c) => ({
    ...c,
    radius: scalePlanetRadius(c.commits),
    orbitRadius: orbitRadiusFromRank(c.rank),
    angle: contributorAngle(c.id)
  }));
}

