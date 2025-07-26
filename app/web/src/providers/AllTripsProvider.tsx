import { createContext, useContext } from 'react';
import { nanoid } from 'nanoid';
import { useSubscribe } from 'replicache-react';
import { listAllTripsSortedByUpdatedAt, Trip } from '@/models/trip';
import { useEventSourcePoke } from '@/utils/poke';
import { useAuth } from './AuthProvider';
import { useReplicache } from './ReplicacheProvider';

interface AllTripsContextType {
  trips: Trip[];
  createTrip: (trip: Partial<Trip>) => Promise<void>;
  deleteAllTrips: () => Promise<void>;
}

const AllTripsContext = createContext<AllTripsContextType | undefined>(undefined);

export const AllTripsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { userId } = useAuth();
  const { rep } = useReplicache();

  // Always call the hook
  const trips: Trip[] = useSubscribe(rep, listAllTripsSortedByUpdatedAt, { default: [] });

  // Always call the hook
  useEventSourcePoke(
    rep ? `${import.meta.env.VITE_REPLICACHE_POKE_URL}?channel=user/${userId}` : '',
    rep
  );

  const createTrip = async (trip: Partial<Trip>) => {
    if (!userId || !rep) {
      throw new Error('User ID and Replicache are required');
    }
    await rep.mutate.createTrip({
      ...trip,
      id: `trip/${nanoid()}`,
      ownerId: userId,
      name: trip.name || 'Untitled Trip',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });
  };

  const deleteAllTrips = async () => {
    if (!rep) return;
    for (const trip of trips) {
      await rep.mutate.deleteTrip(trip.id);
    }
  };

  return (
    <AllTripsContext.Provider value={{ trips, createTrip, deleteAllTrips }}>
      {children}
    </AllTripsContext.Provider>
  );
};

export const useAllTrips = () => {
  const context = useContext(AllTripsContext);
  if (!context) {
    throw new Error('useAllTrips must be used within an AllTripsProvider');
  }
  return context;
};
