import { createContext, ReactNode, useContext, useEffect, useState } from "react";
import { useUser } from "./UserProvider";

interface travelDates {
    startDate: string;
    endDate: string;
}

interface TripContext {
    id: string;
    title: string;
    travelDates: travelDates;
}

interface TripContextType {
    trips: TripContext[];
    updateTrips: () => void;
}


const TripContext = createContext<TripContextType | undefined>(undefined);

export const TripProvider = ({ children }: { children: ReactNode }) => {
    const [trips, setTrips] = useState([]);
    const { user } = useUser();

    const fetchTrips = () => {
        fetch("/api/trips")
        .then((res) => res.json())
        .then((data) => setTrips(data))
        .catch((err) => console.error("Failed to fetch trips:", err));
    };

    // useEffect(() => {
    //     fetchTrips();
    // }, []);

    return (
        <TripContext.Provider value={{ trips, updateTrips: fetchTrips }}>
        {children}
        </TripContext.Provider>
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
    const context = useContext(TripContext);
    if (!context) {
        throw new Error('useTrips must be used within a TripProvider');
    }
    return context;
    }