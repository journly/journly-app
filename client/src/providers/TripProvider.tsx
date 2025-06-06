import { createContext, ReactNode, useContext, useState } from "react";
import { Trip,} from "./TripsProvider";
import { DateTime } from "luxon";

interface TripContextType {
    trip: Trip;
    updateTrips: () => void;
}

const TripContext = createContext<TripContextType | undefined>(undefined);

export const TripProvider = ({ children }: { children: ReactNode }) => {
    const tripId = window.location.pathname.split("/").pop() ?? "Japan"
    const [trip, setTrip] = useState<Trip>({
        id: tripId,
        title: "Japan Adventure",
        travelDates: {
            startDate: DateTime.fromFormat("15/11/2023", "dd/MM/yyyy"),
            endDate: DateTime.fromFormat("28/11/2023", "dd/MM/yyyy"),
        },
        locations: ["Tokyo", "Kyoto", "Osaka"],
        users: [
            {
                username: "john_doe",
                email: "",
                fullName: "John Doe",
                initials: "JD"
            }
        ]
    });
    
    const fetchTrip = () => {
        fetch("/api/trip/" + tripId)
        .then((res) => res.json())
        .then((data) => setTrip(data))
        .catch((err) => console.error("Failed to fetch trip:", err));
    };

    // useEffect(() => {
    //     fetchTrips();
    // }, []);

    return (
        <TripContext.Provider value={{ trip, updateTrips: fetchTrip }}>
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

export const useTrip = () => {
    const context = useContext(TripContext);
    if (!context) {
        throw new Error('useTrips must be used within a TripProvider');
    }
    return context;
    }