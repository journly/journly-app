import { createContext, ReactNode, useContext, useMemo } from 'react';
import { Replicache } from 'replicache';
import { Mutators, mutators } from '../mutators';
import { useAuth } from './AuthProvider';

const createReplicacheClient = (userId?: string | null) => {
  if (typeof window === 'undefined' || !userId) {
    return null;
  }

  const pushURL = import.meta.env.VITE_REPLICACHE_PUSH_URL;
  const pullURL = import.meta.env.VITE_REPLICACHE_PULL_URL;

  return new Replicache({
    name: userId,
    pushURL,
    pullURL,
    mutators,
  });
};

interface ReplicacheContextType {
  rep: Replicache<Mutators> | null;
}

export const ReplicacheContext = createContext<ReplicacheContextType | undefined>(undefined);

export const ReplicacheProvider = ({ children }: { children: ReactNode }) => {
  const { userId } = useAuth();
  const replicache = useMemo(() => createReplicacheClient(userId), [userId]);

  return (
    <ReplicacheContext.Provider
      value={{
        rep: replicache,
      }}
    >
      {children}
    </ReplicacheContext.Provider>
  );
};

export const useReplicache = () => {
  const ctx = useContext(ReplicacheContext);
  if (!ctx) throw new Error('useReplicache must be used within an ReplicacheProvider');
  return ctx;
};
