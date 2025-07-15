import { Navigate, Outlet } from 'react-router-dom';
import { AuthStatus, useAuth } from '../providers/AuthProvider';
import { useEffect, useState } from 'react';

export const ProtectedRoute = () => {
  const { checkAuthenticated } = useAuth();
  const [authStatus, setAuthStatus] = useState<AuthStatus | null>(null);

  useEffect(() => {
    let cancelled = false;

    const verify = async () => {
      try {
        const res = await checkAuthenticated();

        console.log(res)
        console.log(cancelled)

        if (!cancelled) setAuthStatus(res);
      } catch (err) {
        if (!cancelled) setAuthStatus(AuthStatus.Unauthenticated);
      }
    };

    verify();

    return () => {
      cancelled = true;
    };
  }, [checkAuthenticated]);

  if (authStatus === null) {
    return <div className="text-center mt-10 text-gray-600">Checking authentication...</div>;
  }

  if (authStatus == AuthStatus.Unauthenticated) {
    console.log("here")
    return <Navigate to="login" />;
  }

  if (authStatus == AuthStatus.Unverified) {
    return <Navigate to="verify" />;
  }


  return <Outlet />;
};

