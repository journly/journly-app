import { Navigate, Outlet } from 'react-router-dom';
import { useAuth } from '../providers/AuthProvider';
import { useEffect, useState } from 'react';

export const ProtectedRoute = () => {
  const { checkAuthenticated } = useAuth();
  const [isAllowed, setIsAllowed] = useState<boolean | null>(null);

  useEffect(() => {
    let cancelled = false;

    const verify = async () => {
      try {
        const ok = await checkAuthenticated();
        if (!cancelled) setIsAllowed(ok);
      } catch (err) {
        if (!cancelled) setIsAllowed(false);
      }
    };

    verify();

    return () => {
      cancelled = true;
    };
  }, [checkAuthenticated]);

  if (isAllowed === null) {
    return <div className="text-center mt-10 text-gray-600">Checking authentication...</div>;
  }

  if (!isAllowed) {
    return <Navigate to="/login" replace />;
  }

  return <Outlet />;
};

