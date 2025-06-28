import { Navigate, useNavigate } from "react-router-dom";
import { useAuth } from "../providers/AuthProvider";
import { useEffect, useState } from "react";
import { LoginCredentials } from "../api-client";
import GoogleIcon from "../assets/icons8-google.svg";
import { getGoogleUrl } from "../utils/getGoogleUrl";

export default function LoginPage() {
  const { login } = useAuth();
  const navigate = useNavigate();
  const { checkAuthenticated } = useAuth();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [isAuthenticated, setIsAuthenticated] = useState<boolean | null>(null);

  useEffect(() => {
    let cancelled = false;

    const verify = async () => {
      try {
        const ok = await checkAuthenticated();
        if (!cancelled) setIsAuthenticated(ok);
      } catch (err) {
        if (!cancelled) setIsAuthenticated(false);
      }
    };

    verify();

    return () => {
      cancelled = true;
    };
  }, [checkAuthenticated]);

  if (isAuthenticated) {
    return <Navigate to="/" replace />;
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await login({ email, password } as LoginCredentials);
      navigate('/');
    } catch (err) {
      return
    }
  };

  return (
    <div>
      <div className="min-h-screen flex flex-col justify-center items-center px-4">
        <form onSubmit={handleSubmit} className="w-full max-w-sm space-y-4">
          <h2 className="text-center text-gray-600 text-2xl font-medium">Login</h2>

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
            Login
          </button>
          <div className="flex items-center my-4 before:flex-1 before:border-t before:border-gray-300 before:mt-0.5 after:flex-1 after:border-t after:border-gray-300 after:mt-0.5 "
          >
            <p className="text-center font-semibold mx-4 mb-0">
              OR
            </p>
          </div>
          <a className="w-full h-12 bg-neutral-200 rounded-xl hover:bg-neutral-300 flex items-center justify-center gap-2 font-semibold"
            href={getGoogleUrl("/oauth/callback")}
            role="button"
            onClick={() => console.log("hello")}
          >
            <img src={GoogleIcon} width={40} /> CONTINUE WITH GOOGLE
          </a>

          <div className="text-center text-sm text-gray-500 pt-2">
            Don’t have an account?{' '}
            <a onClick={() => navigate('/register')} className="font-semibold text-black hover:underline">
              Create a one here
            </a>
          </div>
        </form>
      </div>
    </div>
  )
}
