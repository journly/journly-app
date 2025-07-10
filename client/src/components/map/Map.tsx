import { useEffect, useRef } from "react"
import mapboxgl from 'mapbox-gl'

mapboxgl.accessToken = import.meta.env.VITE_MAPBOX_ACCESS_TOKEN;

export const Map: React.FC = () => {
  const mapContainerRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (!mapContainerRef.current) return;

    const map = new mapboxgl.Map({
      container: mapContainerRef.current,
      style: 'mapbox://styles/mapbox/streets-v12',
    });

    return () => {
      map.remove();
    };
  }, []);

  return (
    <div className="w-full h-full overflow-hidden">
      <div ref={mapContainerRef} className="w-full h-full" />
    </div>
  );
};
