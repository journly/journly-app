import { jwtDecode } from 'jwt-decode';
import { AuthenticationApi, Configuration, ErrorResponse, LoginCredentials, ResendVerificationBody, VerificationBody } from '../api-client';
import React, { createContext, useContext, useEffect, useRef, useState } from 'react';

interface JwtPayload {
  sub: string;
  exp: number;
}

interface Tokens {
  access_token: string,
}

export enum AuthStatus {
  Authenticated = 'Authenticated',
  Unauthenticated = 'Unauthenticated',
  Unverified = 'Unverified',
  NoConnection = 'NoConnection'
}

interface AuthContextType {
  accessToken: string | null;
  userId: string | null;
  checkAuthenticated: () => Promise<AuthStatus>;
  login: (creds: LoginCredentials) => Promise<void>;
  oAuthLogin: (access_token: string, refresh_token: string) => void;
  logout: () => Promise<void>;
  resendVerificationCode: () => Promise<void>;
  verifyEmail: (code: number) => Promise<boolean>;
  getAuthApi: () => AuthenticationApi;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [accessToken, setAccessToken] = useState<string | null>(null);
  const [userId, setUserId] = useState<string | null>(null);
  const [userEmail, setUserEmail] = useState<string | null>(null);
  const refreshTimeout = useRef<ReturnType<typeof setTimeout> | null>(null);

  const buildAuthApi = (token: string | null) =>
    new AuthenticationApi(
      new Configuration({
        basePath: import.meta.env.VITE_API_BASE_URL,
        accessToken: () => token ?? '',
      })
    )

  const getAuthApi = () => buildAuthApi(accessToken)

  const oAuthLogin = (access_token: string) => {
    const { sub } = jwtDecode<JwtPayload>(access_token);

    setUserId(sub);
    setAccessToken(access_token);
  }

  const login = async (creds: LoginCredentials) => {
    try {
      const response = await getAuthApi().login(creds);

      const { access_token }: Tokens = response.data;
      const { sub } = jwtDecode<JwtPayload>(access_token);

      setUserId(sub);
      setUserEmail(creds.email);
      setAccessToken(access_token);
    } catch (e) {
      console.log(e)
    }
  };

  const logout = async () => {
    try {
      await getAuthApi().logout({ withCredentials: true });

      setUserId(null);
      setAccessToken(null);
    } catch {
      console.log("Could not logout")
    }
  }


  const scheduleTokenRefresh = (access_token: string) => {
    const { exp } = jwtDecode<JwtPayload>(access_token);
    const expiresInMs = exp * 1000 - Date.now();

    const refreshIn = 2000 //Math.max(expiresInMs - 60000, 10000);

    if (refreshTimeout.current) {
      clearTimeout(refreshTimeout.current);
      refreshTimeout.current = null;
    }

    refreshTimeout.current = setTimeout(() => {
      refreshAccessToken();
    }, refreshIn)
  }

  const refreshAccessToken = async (): Promise<boolean> => {
    try {
      const response = await getAuthApi().refresh({ withCredentials: true });
      const { access_token }: Tokens = response.data;
      const { sub } = jwtDecode<JwtPayload>(access_token);

      setAccessToken(access_token);
      setUserId(sub);

      return true;
    } catch {
      console.log("Could not refresh")
      return false;
    }
  };

  const checkAuthenticated = async (): Promise<AuthStatus> => {
    if (!accessToken) {
      const refreshed = await refreshAccessToken();
      if (!refreshed) {
        await logout();
        return AuthStatus.Unauthenticated;
      }

      return AuthStatus.Unauthenticated;
    }

    try {
      await getAuthApi().getMe();

      return AuthStatus.Authenticated;
    } catch (err: any) {
      if (err.response?.status === 401) {
        const refreshed = await refreshAccessToken();
        if (refreshed) {
          try {
            await getAuthApi().getMe();

            return AuthStatus.Authenticated;
          } catch (e: any) {
            if (err.response?.status === 403 && (err.response?.data as ErrorResponse).error == 'unverified_user') {
              return AuthStatus.Unverified;
            }

            await logout();
            return AuthStatus.Unauthenticated;
          }
        } else {
          await logout();
          return AuthStatus.Unauthenticated;
        }
      } else if (err.response?.status === 403 && (err.response?.data as ErrorResponse).error == 'unverified_user') {
        return AuthStatus.Unverified
      } else if (err.code === 'ERR_NETWORK') {
        return AuthStatus.NoConnection
      } else {
        await logout();
        return AuthStatus.Unauthenticated;
      }
    }
  };

  const resendVerificationCode = async (): Promise<void> => {
    try {
      const body: ResendVerificationBody = {
        email: userEmail ?? ''
      }

      await getAuthApi().resendVerificationCode(body)
    } catch (err: any) {
      console.log("failed to resend verification code")
    }
  }

  const verifyEmail = async (code: number): Promise<boolean> => {
    try {
      const body: VerificationBody = {
        email: userEmail ?? '',
        verification_code: code
      }

      console.log(userEmail);

      await getAuthApi().verifyUserEmail(body)

      return true
    } catch (err: any) {
      return false
    }
  }


  return (
    <AuthContext.Provider
      value={{
        accessToken,
        checkAuthenticated,
        oAuthLogin,
        userId,
        login,
        logout,
        resendVerificationCode,
        verifyEmail,
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

