import { AuthenticationApi, Configuration, EncodableUser, LoginCredentials } from '../api-client';
import React, { createContext, useContext, useEffect, useRef, useState } from 'react';

interface AuthContextType {
  accessToken: string | null;
  refreshToken: string | null;
  checkAuthenticated: () => Promise<boolean>;
  login: (creds: LoginCredentials) => Promise<void>;
  oAuthLogin: (access_token: string, refresh_token: string) => void;
  logout: () => Promise<void>;
  getAuthApi: () => AuthenticationApi;
  getUser: () => EncodableUser | null;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [refreshToken, setRefreshToken] = useState<string | null>(() =>
    localStorage.getItem('refresh_token') ?? null
  );
  const userRef = useRef<EncodableUser | null>(null);

  const getUser = () => userRef.current;

  const getAuthApi = () =>
    new AuthenticationApi(
      new Configuration({
        basePath: import.meta.env.VITE_API_BASE_URL,
        accessToken: () => accessToken ?? '',
      })
    );

  const oAuthLogin = (access_token: string, refresh_token: string) => {
    setAccessToken(access_token);
    setRefreshToken(refresh_token);
    localStorage.setItem('refresh_token', refresh_token);
  }

  const login = async (creds: LoginCredentials) => {
    const response = await getAuthApi().login(creds);
    const { access_token, refresh_token } = response.data;

    setAccessToken(access_token);
    setRefreshToken(refresh_token);
    localStorage.setItem('refresh_token', refresh_token);
  };

  const logout = async () => {
    if (refreshToken) {
      await getAuthApi().logout({ refresh_token: refreshToken });
    }
    setAccessToken(null);
    setRefreshToken(null);
    localStorage.removeItem('refresh_token');
  };

  const refreshAccessToken = async (): Promise<boolean> => {
    if (!refreshToken) return false;

    try {
      const response = await getAuthApi().refresh({ refresh_token: refreshToken });
      const { access_token, refresh_token } = response.data;
      setAccessToken(access_token);
      setRefreshToken(refresh_token);

      localStorage.setItem('refresh_token', refresh_token);
      return true;
    } catch {
      return false;
    }
  };

  const checkAuthenticated = async (): Promise<boolean> => {
    if (!accessToken && refreshToken) {
      const refreshed = await refreshAccessToken();
      if (!refreshed) {
        await logout();
        return false;
      }
    }

    try {
      let user = await getAuthApi().getMe(); // uses current access token

      userRef.current = user.data.user;

      return true;
    } catch (err: any) {
      if (err.response?.status === 401 && refreshToken) {
        const refreshed = await refreshAccessToken();
        if (refreshed) {
          try {
            await getAuthApi().getMe();
            return true;
          } catch {
            await logout();
            return false;
          }
        } else {
          await logout();
          return false;
        }
      } else {
        await logout();
        return false;
      }
    }
  };

  useEffect(() => {
    if (refreshToken && !accessToken) {
      refreshAccessToken().catch(() => logout());
    }
  }, []);

  return (
    <AuthContext.Provider
      value={{
        accessToken,
        refreshToken,
        checkAuthenticated,
        oAuthLogin,
        login,
        logout,
        getAuthApi,
        getUser
      }}
    >
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error('useAuth must be used within an AuthProvider');
  return ctx;
};

