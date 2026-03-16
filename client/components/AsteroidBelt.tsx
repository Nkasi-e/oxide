import { memo, useEffect, useRef } from "react";
import { useFrame } from "@react-three/fiber";
import * as THREE from "three";
import type { AsteroidBelt as AsteroidBeltData } from "@/utils/api";

interface AsteroidBeltProps {
  belt: AsteroidBeltData;
  position?: [number, number, number];
  rotation?: [number, number, number];
}

function AsteroidBeltImpl({
  belt,
  position = [0, 0, 0],
  rotation = [0, 0, 0]
}: AsteroidBeltProps) {
  const meshRef = useRef<any>(null);

  useEffect(() => {
    const mesh = meshRef.current;
    if (!mesh) return;
    const temp = new THREE.Object3D();
    const { count, innerRadius, outerRadius } = belt;
    const instances = Math.min(count, 2000);
    for (let i = 0; i < instances; i += 1) {
      const angle = (i / instances) * Math.PI * 2;
      const radius =
        innerRadius + (outerRadius - innerRadius) * Math.random() * 0.95;
      const x = Math.cos(angle) * radius;
      const z = Math.sin(angle) * radius;
      temp.position.set(x, (Math.random() - 0.5) * 1.4, z);
      const s = 0.04 + Math.random() * 0.08;
      temp.scale.set(s, s, s);
      temp.updateMatrix();
      mesh.setMatrixAt(i, temp.matrix);
    }
    mesh.instanceMatrix.needsUpdate = true;
  }, [belt]);

  useFrame((_, delta) => {
    if (!meshRef.current) return;
    meshRef.current.rotation.y += delta * 0.04;
  });

  const instances = Math.min(belt.count, 2000);

  return (
    <instancedMesh ref={meshRef} args={[undefined, undefined, instances]} position={position} rotation={rotation}>
      <sphereGeometry args={[1, 6, 6]} />
      <meshStandardMaterial
        color="#6b7280"
        emissive="#111827"
        roughness={0.9}
        metalness={0.1}
      />
    </instancedMesh>
  );
}

export const AsteroidBelt = memo(AsteroidBeltImpl);

