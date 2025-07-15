import { createContext, ReactNode, useContext, useEffect, useState } from "react";
import { useUser, User } from "./UserProvider";
import { DateTime } from 'luxon';
import { TripsApi } from "../api-client";

export interface travelDates {
    startDate: DateTime;
    endDate: DateTime;
}

export interface Trip {
    id: string;
    title: string;
    travelDates: travelDates;
    locations?: string[];
    users?: User[];
}

interface TripsContextType {
    trips: Trip[];
    updateTrips: () => void;
}

const tripsApi = new TripsApi();

const TripsContext = createContext<TripsContextType | undefined>(undefined);

export const TripsProvider = ({ children }: { children: ReactNode }) => {
    const [trips, setTrips] = useState<Trip[]>([]);
    const mounted = false

    const fetchTrips = async () => {
        await tripsApi.getTrips(
            
        ).then((response) => {
            if (response && response.data) {
                const fetchedTrips = response.data.trips.map((trip: any) => ({
                    id: trip.id,
                    title: trip.title,
                    travelDates: {
                        startDate: DateTime.fromISO(trip.travelDates.startDate),
                        endDate: DateTime.fromISO(trip.travelDates.endDate)
                    },
                    locations: trip.locations || [],
                    users: trip.users || []
                }));
                console.log("trips", response.data.trips)
                setTrips(fetchedTrips);
            }
        })
    }


    useEffect(() => {
        if (mounted) {
            fetchTrips();
        }
    }, [mounted]);

    return (
        <TripsContext.Provider value={{ trips, updateTrips: fetchTrips }}>
        {children}
        </TripsContext.Provider>
    );
};

// export const useTrip = () => {
//   const context = useContext(TripContext);
//   if (!context) {
//     throw new Error('useTrip must be used within a TripProvider');
//   }
//   return context;
// };

export const useTrips = () => {
    const context = useContext(TripsContext);
    if (!context) {
        throw new Error('useTrips must be used within a TripProvider');
    }
    return context;
    }