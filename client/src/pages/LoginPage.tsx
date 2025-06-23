import { useNavigate } from "react-router-dom";
import { useAuth } from "../providers/AuthProvider";
import { useState } from "react";
import { LoginCredentials } from "../api-client";

export default function LoginPage() {
  const { login } = useAuth();
  const navigate = useNavigate();

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      await login({ email, password } as LoginCredentials);
      navigate('/');
    } catch (err) {
      console.error('Login failed', err);
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

          <div className="text-center text-sm text-gray-500 pt-2">
            Don’t have an account?{' '}
            <a href="/register" className="font-semibold text-black hover:underline">
              Create a one here
            </a>
          </div>
        </form>
      </div>
    </div>
  )
}
