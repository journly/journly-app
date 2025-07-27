import { createContext, useContext } from 'react';
import { useSubscribe } from 'replicache-react';
import { listAllTripsSortedByUpdatedAt, Trip, TripCreate } from '@/models/trip';
import { useEventSourcePoke } from '@/utils/poke';
import { useAuth } from './AuthProvider';
import { useReplicache } from './ReplicacheProvider';
import { useUser } from './UserProvider';

interface AllTripsContextType {
  trips: Trip[];
  createTrip: (trip: TripCreate) => Promise<void>;
  deleteAllTrips: () => Promise<void>;
}

const AllTripsContext = createContext<AllTripsContextType | undefined>(undefined);

export const AllTripsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { userId } = useAuth();
  const { rep } = useReplicache();
  const { user } = useUser();

  // Always call the hook
  const trips: Trip[] = useSubscribe(rep, listAllTripsSortedByUpdatedAt, { default: [] });

  // Always call the hook
  useEventSourcePoke(
    rep ? `${import.meta.env.VITE_REPLICACHE_POKE_URL}?channel=user/${userId}` : '',
    rep
  );

  const createTrip = async (trip: TripCreate) => {
    if (!userId || !rep || !user) {
      throw new Error('User ID and Replicache and user are required');
    }
    await rep.mutate.createTrip({
      trip,
      user: {
        id: user.id,
        username: user.username,
        avatar: user.avatar || undefined,
      },
    });
  };

  const deleteAllTrips = async () => {
    if (!rep) return;
    console.log('trips list', trips);
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
