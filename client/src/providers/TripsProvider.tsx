import { createContext, ReactNode, useContext, useState } from "react";
import { DateTime } from 'luxon';
import { EncodableUser } from "../api-client";

export interface travelDates {
  startDate: DateTime;
  endDate: DateTime;
}

export interface Trip {
  id: string;
  title: string;
  travelDates: travelDates;
  locations?: string[];
  users?: EncodableUser[];
}

interface TripsContextType {
  trips: Trip[];
  updateTrips: () => void;
}


const TripsContext = createContext<TripsContextType | undefined>(undefined);

export const TripsProvider = ({ children }: { children: ReactNode }) => {
  const [trips, setTrips] = useState<Trip[]>([]);

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
