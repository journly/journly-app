import { useEffect } from 'react';
import { Replicache } from 'replicache';
import { Mutators } from '@/mutators';

export function useEventSourcePoke(url: string, rep: Replicache<Mutators> | null) {
  useEffect(() => {
    if (!rep || !url) return;

    const eventSource = new EventSource(url);
    eventSource.onmessage = () => {
      void rep.pull();
    };
    return () => eventSource.close();
  }, [url, rep]);
}
