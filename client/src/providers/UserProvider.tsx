import { createContext, useContext, useState, ReactNode, useEffect } from 'react';
import { EncodableUser, UsersApi } from '../api-client';

export interface User {
    username: string;
    email: string;
    fullName: string;
    initials: string;
}

// Context value type
interface UserContextType {
  user: EncodableUser | null;
  setUser: (user: EncodableUser | null) => void;
}

const userApi = new UsersApi();

// Create context with default
const UserContext = createContext<UserContextType | undefined>(undefined);

// Provider component
export const UserProvider = ({ children }: { children: ReactNode }) => {
    const [user, setUser] = useState<EncodableUser | null>(null);

//TODO: Fetch user data from API and set it in state
    useEffect(() => {
       const fetchUserData = async () => {
            try {
                const response = await userApi.getUser({
                    userId: 'current', // Replace with actual user ID or logic to get current user ID
                });
                if (response) {
                    setUser(response.user as EncodableUser);
                }
            } catch (error) {
                console.error('Error fetching user data:', error);
            }
        }
    }, []);
  

  return (
    <UserContext.Provider value={{ user, setUser }}>
      {children}
    </UserContext.Provider>
  );
};

// Custom hook for using the user context
export const useUser = () => {
  const context = useContext(UserContext);
  if (!context) {
    throw new Error('useUser must be used within a UserProvider');
  }
  return context;
};