import { Suspense, useCallback, useMemo, useState } from "react";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Stars as DreiStars } from "@react-three/drei";
import type { GalaxyPlanet, GalaxyResponse } from "@/utils/api";
import { Star } from "@/components/Star";
import { Planet } from "@/components/Planet";
import { AsteroidBelt } from "@/components/AsteroidBelt";
import { OrbitRing } from "@/components/OrbitRing";

const ORBIT_INCLINATION_SCALE = 0.07;

interface GalaxySceneProps {
  galaxy: GalaxyResponse;
  onSelectContributor: (contributor: GalaxyPlanet | null) => void;
}

export function GalaxyScene({ galaxy, onSelectContributor }: GalaxySceneProps) {
  const [hovered, setHovered] = useState<GalaxyPlanet | null>(null);

  const topPlanets = useMemo(
    () => galaxy.planets.slice(0, 50),
    [galaxy.planets]
  );
  const totalContributors =
    galaxy.planets.length + (galaxy.asteroid_belt?.count ?? 0);

  const handleHover = useCallback(
    (planet: GalaxyPlanet | null) => {
      setHovered(planet);
      if (planet) {
        onSelectContributor(planet);
      }
    },
    [onSelectContributor]
  );

  const handleClick = useCallback(
    (planet: GalaxyPlanet) => {
      onSelectContributor(planet);
    },
    [onSelectContributor]
  );

  return (
    <div className="relative h-full w-full overflow-hidden rounded-2xl border border-border bg-background shadow-card">
      <div className="absolute top-3 right-3 z-10 rounded-lg border border-border/80 bg-background/90 px-3 py-2 shadow-lg backdrop-blur-sm">
        <p className="text-xs font-medium text-muted">Total contributors</p>
        <p className="text-xl font-semibold tabular-nums text-zinc-100">
          {totalContributors.toLocaleString()}
        </p>
        <p className="mt-0.5 text-[10px] text-muted">
          {galaxy.planets.length} planets · {(galaxy.asteroid_belt?.count ?? 0).toLocaleString()} in belt
        </p>
      </div>
      <Canvas
        camera={{ position: [0, 8, 28], fov: 50 }}
        dpr={[1, 2]}
        gl={{ antialias: true, alpha: true }}
      >
        <color attach="background" args={["#030712"]} />
        <fog attach="fog" args={["#030712", 25, 75]} />

        <ambientLight intensity={0.2} />
        <pointLight position={[0, 0, 0]} intensity={2} distance={45} color="#fef3c7" decay={2} />
        <directionalLight intensity={0.9} color="#fde68a" position={[12, 14, 10]} />
        <directionalLight intensity={0.25} color="#7dd3fc" position={[-15, -8, -12]} />
        <pointLight intensity={0.4} distance={70} position={[20, 10, 20]} color="#a78bfa" />

        <Suspense fallback={null}>
          <DreiStars
            radius={120}
            depth={80}
            count={8000}
            factor={4}
            saturation={0}
            fade
          />

          <Star star={galaxy.star} position={[0, 0, 0]} />

          {topPlanets.map((planet) => (
            <OrbitRing
              key={`ring-${planet.id}`}
              radius={planet.orbitRadius}
              inclination={(planet.rank - 1) * ORBIT_INCLINATION_SCALE}
              opacity={0.18 + (planet.rank % 3) * 0.04}
            />
          ))}
          {topPlanets.map((planet) => (
            <Planet
              key={planet.id}
              planet={planet}
              isActive={hovered?.id === planet.id}
              onHover={(p) => handleHover(p)}
              onClick={handleClick}
            />
          ))}

          {galaxy.asteroid_belt.count > 0 && (
            <AsteroidBelt
              belt={galaxy.asteroid_belt}
              position={[0, 0, 0]}
              rotation={[0.12, 0, 0.05]}
            />
          )}

          <OrbitControls
            enablePan={false}
            enableDamping
            dampingFactor={0.08}
            rotateSpeed={0.7}
            minDistance={12}
            maxDistance={70}
            maxPolarAngle={Math.PI * 0.9}
          />
        </Suspense>
      </Canvas>
    </div>
  );
}

