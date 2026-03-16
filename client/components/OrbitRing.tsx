import { memo } from "react";

interface OrbitRingProps {
  radius: number;
  inclination: number;
  opacity?: number;
}

function OrbitRingImpl({ radius, inclination, opacity = 0.22 }: OrbitRingProps) {
  return (
    <group rotation={[inclination, 0, 0]}>
      <mesh rotation={[Math.PI / 2, 0, 0]}>
        <torusGeometry args={[radius, 0.018, 8, 72]} />
        <meshBasicMaterial
          color="#94a3b8"
          transparent
          opacity={opacity}
          depthWrite={false}
        />
      </mesh>
    </group>
  );
}

export const OrbitRing = memo(OrbitRingImpl);
