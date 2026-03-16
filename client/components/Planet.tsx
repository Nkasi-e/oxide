import { memo, useRef } from "react";
import { useFrame } from "@react-three/fiber";
import { Html } from "@react-three/drei";
import type { GalaxyPlanet } from "@/utils/api";
import type { Group } from "three";

interface PlanetProps {
  planet: GalaxyPlanet;
  isActive: boolean;
  onHover: (planet: GalaxyPlanet | null, event: unknown) => void;
  onClick: (planet: GalaxyPlanet) => void;
}

const ORBIT_SPEED = 0.06;
const INCLINATION_SCALE = 0.07;

function PlanetImpl({ planet, isActive, onHover, onClick }: PlanetProps) {
  const groupRef = useRef<Group>(null);
  const inclination = (planet.rank - 1) * INCLINATION_SCALE;

  useFrame(({ clock }) => {
    const group = groupRef.current;
    if (!group) return;
    const t = clock.getElapsedTime();
    const angle = planet.angle + t * ORBIT_SPEED;
    const r = planet.orbitRadius;
    const cosA = Math.cos(angle);
    const sinA = Math.sin(angle);
    const x = r * cosA;
    const y = -r * sinA * Math.sin(inclination);
    const z = r * sinA * Math.cos(inclination);
    group.position.set(x, y, z);
    group.rotation.y += 0.008;
  });

  return (
    <group ref={groupRef}>
      <mesh
        onPointerOver={(e) => {
          e.stopPropagation();
          onHover(planet, e);
        }}
        onPointerOut={(e) => {
          e.stopPropagation();
          onHover(null, e);
        }}
        onClick={(e) => {
          e.stopPropagation();
          onClick(planet);
        }}
      >
        <sphereGeometry args={[planet.radius, 36, 36]} />
        <meshStandardMaterial
          color={isActive ? "#fb923c" : "#38bdf8"}
          emissive={isActive ? "#ea580c" : "#0ea5e9"}
          emissiveIntensity={isActive ? 1.4 : 0.85}
          roughness={0.35}
          metalness={0.4}
        />
      </mesh>
      <mesh scale={1.15}>
        <sphereGeometry args={[planet.radius, 20, 20]} />
        <meshBasicMaterial
          color={isActive ? "#f97316" : "#22d3ee"}
          transparent
          opacity={0.12}
          depthWrite={false}
        />
      </mesh>
      <Html
        position={[0, planet.radius + 0.25, 0]}
        center
        distanceFactor={8}
        style={{
          pointerEvents: "none",
          userSelect: "none",
          whiteSpace: "nowrap",
          fontSize: "10px",
          fontWeight: 600,
          color: isActive ? "#f97316" : "#94a3b8",
          textShadow: "0 0 8px rgba(0,0,0,0.9)",
          fontFamily: "var(--font-outfit), system-ui, sans-serif"
        }}
      >
        @{planet.username}
      </Html>
    </group>
  );
}

export const Planet = memo(PlanetImpl);

