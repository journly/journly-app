import { createContext, useContext, useState, ReactNode, useEffect } from 'react';

export interface User {
    username: string;
    email: string;
    fullName: string;
    initials: string;
}

// Context value type
interface UserContextType {
  user: User | null;
  setUser: (user: User | null) => void;
}

// Create context with default
const UserContext = createContext<UserContextType | undefined>(undefined);

// Provider component
export const UserProvider = ({ children }: { children: ReactNode }) => {
    const [user, setUser] = useState<User | null>(null);

//TODO: Fetch user data from API and set it in state
    // useEffect(() => {
    //     const fetchUserData = async () => {
    //     const response = await fetch('/api/user');
    //     const data = await response.json();
    //     setUser(data);
    //     };
    //     fetchUserData();
    // }, []);
  

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