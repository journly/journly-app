import { createContext, useContext, useEffect, useState } from "react";
import { Configuration, EncodableUser, LoginCredentials, PasswordUpdateRequest, UpdateInformationBody, UsersApi } from "../api-client";
import { useAuth } from "./AuthProvider";

interface UserContextType {
  user: EncodableUser | null;
  fetchUser: () => Promise<void>;
  updateUsername: (newUsername: string) => Promise<boolean>;
  updateEmail: (newEmail: string) => Promise<boolean>;
  updatePassword: (currentPassword: string, newPassword: string) => Promise<boolean>;
  deleteUser: () => Promise<void>;
  validateUserPassword: (password: string) => Promise<boolean>;
}

const UserContext = createContext<UserContextType | undefined>(undefined);

export const UserProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const { accessToken, getAuthApi, logout } = useAuth();
  const [user, setUser] = useState<EncodableUser | null>(null);

  const getUsersApi = () =>
    new UsersApi(
      new Configuration({
        basePath: import.meta.env.VITE_API_BASE_URL,
        accessToken: () => accessToken ?? '',
      })
    )


  const fetchUser = async () => {
    let res = await getAuthApi().getMe();
    setUser(res.data.user);
  }

  useEffect(() => {
    if (!user && accessToken) {
      fetchUser();
    }
  }, [accessToken])

  const updateUser = async (data: UpdateInformationBody) => {
    if (!user) return false;

    try {
      await getUsersApi().updateUser(user.id, data);

      fetchUser()
      return true
    } catch {
      return false
    }
  }

  const updateUsername = async (newUsername: string) => {
    const updateBody: UpdateInformationBody = {
      username: newUsername
    }

    return await updateUser(updateBody);
  }

  const updateEmail = async (newEmail: string) => {
    const updateBody: UpdateInformationBody = {
      email: newEmail
    }

    return await updateUser(updateBody);
  }

  const updatePassword = async (currentPassword: string, newPassword: string) => {
    if (!user) return false;

    const updateBody: PasswordUpdateRequest = {
      current_password: currentPassword,
      new_password: newPassword
    }

    try {
      await getUsersApi().updateUserPassword(user.id, updateBody);

      return true
    } catch {
      return false
    }
  }

  const deleteUser = async () => {
    if (!user) return;

    try {
      await getUsersApi().deleteUser(user.id);

      await logout();
    } catch {
      console.log("Failed to delete user.")
    }
  }

  const validateUserPassword = async (password: string) => {
    if (!user) return false;

    const credentials: LoginCredentials = {
      email: user.email,
      password
    }

    try {
      await getAuthApi().login(credentials)
      return true
    } catch {
      return false
    }
  }

  return (
    <UserContext.Provider
      value={{
        user,
        fetchUser,
        updateUsername,
        updateEmail,
        updatePassword,
        deleteUser,
        validateUserPassword
      }}
    >
      {children}
    </UserContext.Provider>
  )
}

export const useUser = () => {
  const ctx = useContext(UserContext);
  if (!ctx) throw new Error('useUser must be used within an UserProvider');
  return ctx;
};
