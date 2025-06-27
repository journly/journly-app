import { jwtDecode } from 'jwt-decode';
import { AuthenticationApi, Configuration, LoginCredentials } from '../api-client';
import React, { createContext, useContext, useEffect, useRef, useState } from 'react';

interface JwtPayload {
  exp: number;
}

interface Tokens {
  access_token: string,
  refresh_token: string
}

interface AuthContextType {
  accessToken: string | null;
  refreshToken: string | null;
  checkAuthenticated: () => Promise<boolean>;
  login: (creds: LoginCredentials) => Promise<void>;
  oAuthLogin: (access_token: string, refresh_token: string) => void;
  logout: () => Promise<void>;
  getAuthApi: () => AuthenticationApi;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [refreshToken, setRefreshToken] = useState<string | null>(() =>
    localStorage.getItem('refresh_token') ?? null
  );
  const refreshTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);

  const buildAuthApi = (token: string | null) =>
    new AuthenticationApi(
      new Configuration({
        basePath: import.meta.env.VITE_API_BASE_URL,
        accessToken: () => token ?? '',
      })
    )

  const getAuthApi = () => buildAuthApi(accessToken)

  const oAuthLogin = (access_token: string, refresh_token: string) => {
    setAccessToken(access_token);
    setRefreshToken(refresh_token);
    localStorage.setItem('refresh_token', refresh_token);
  }

  const login = async (creds: LoginCredentials) => {
    const response = await getAuthApi().login(creds);
    const { access_token, refresh_token }: Tokens = response.data;

    setAccessToken(access_token);
    setRefreshToken(refresh_token);
    localStorage.setItem('refresh_token', refresh_token);
  };


  const logout = async () => {
    if (refreshToken) {
      await getAuthApi().logout({ refresh_token: refreshToken });

      setAccessToken(null);
      setRefreshToken(null);
      localStorage.removeItem('refresh_token');
    }
  };


  const scheduleTokenRefresh = (access_token: string, refresh_token: string) => {
    const { exp } = jwtDecode<JwtPayload>(access_token);
    const expiresInMs = exp * 1000 - Date.now();

    const refreshIn = Math.max(expiresInMs - 60000, 10000);

    if (refreshTimeout.current) {
      clearTimeout(refreshTimeout.current);
      refreshTimeout.current = null;
    }

    refreshTimeout.current = setTimeout(() => {
      refreshAccessToken(refresh_token);
    }, refreshIn)
  }

  const refreshAccessToken = async (refresh: string | null = null): Promise<boolean> => {
    if (!refreshToken) return false;


    try {
      const response = await getAuthApi().refresh({ refresh_token: refresh ?? refreshToken });
      const { access_token, refresh_token }: Tokens = response.data;

      setAccessToken(access_token);
      setRefreshToken(refresh_token);

      localStorage.setItem('refresh_token', refresh_token);

      return true;
    } catch {
      return false;
    }
  };

  const checkAuthenticated = async (): Promise<boolean> => {
    if (!refreshToken) return false;

    if (!accessToken && refreshToken) {
      const refreshed = await refreshAccessToken();
      if (!refreshed) {
        await logout();
        return false;
      }

      return false;
    }

    try {
      await getAuthApi().getMe();

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
    if (accessToken && refreshToken) {
      scheduleTokenRefresh(accessToken, refreshToken);
    }

  }, [accessToken, refreshToken])


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

