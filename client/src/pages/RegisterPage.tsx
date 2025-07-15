import { Navigate, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import { LoginCredentials, RegisterUserBody } from "../api-client";
import { AuthStatus, useAuth } from "../providers/AuthProvider";

export default function RegisterPage() {
  const navigate = useNavigate();

  const { getAuthApi, checkAuthenticated, login } = useAuth();
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
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

  if (authStatus == AuthStatus.Authenticated) {
    return <Navigate to="/" replace />;
  }

  if (authStatus == AuthStatus.Unverified) {
    return <Navigate to="/verify" replace />;
  }


  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await getAuthApi().registerUser({ username, email, password } as RegisterUserBody);

      await login({ email, password } as LoginCredentials);

      navigate('/');
    } catch (err) {
      console.error('Register failed', err);
    }
  };

  return (
    <div>
      <div className="min-h-screen flex flex-col justify-center items-center px-4">
        <form onSubmit={handleSubmit} className="w-full max-w-sm space-y-4">
          <h2 className="text-center text-gray-600 text-2xl font-medium">Sign Up</h2>

          <input
            type="text"
            placeholder="Username"
            className="w-full border border-gray-300 rounded-xl px-4 py-3 text-sm"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
            required
          />

          <input
            type="email"
            placeholder="Email"
            className="w-full border border-gray-300 rounded-xl px-4 py-3 text-sm"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
            required
          />

          <input
            type="password"
            placeholder="Password"
            className="w-full border border-gray-300 rounded-xl px-4 py-3 text-sm"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            required
          />

          <button
            type="submit"
            className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 rounded-xl"
          >
            Sign up
          </button>

          <div className="text-center text-sm text-gray-500 pt-2">
            Already have an account?{' '}
            <a onClick={() => navigate('/login')} className="font-semibold text-black hover:underline cursor-pointer">
              Log in here
            </a>
          </div>
        </form>
      </div>
    </div>
  )
}
