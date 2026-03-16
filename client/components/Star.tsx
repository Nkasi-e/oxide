import { memo } from "react";
import type { GalaxyStar } from "@/utils/api";

interface StarProps {
  star: GalaxyStar;
  position?: [number, number, number];
}

function StarImpl({ star, position = [0, 0, 0] }: StarProps) {
  const glowScale = 1.85;
  return (
    <group position={position}>
      <mesh>
        <sphereGeometry args={[star.radius * glowScale, 32, 32]} />
        <meshBasicMaterial
          color="#fef3c7"
          transparent
          opacity={0.2}
          depthWrite={false}
        />
      </mesh>
      <mesh>
        <sphereGeometry args={[star.radius * 1.25, 48, 48]} />
        <meshBasicMaterial
          color="#fde68a"
          transparent
          opacity={0.35}
          depthWrite={false}
        />
      </mesh>
      <mesh>
        <sphereGeometry args={[star.radius, 64, 64]} />
        <meshStandardMaterial
          emissive="#fbbf24"
          emissiveIntensity={2}
          color="#f97316"
          roughness={0.2}
          metalness={0.1}
        />
      </mesh>
    </group>
  );
}

export const Star = memo(StarImpl);

