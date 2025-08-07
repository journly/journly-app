import { Route, Routes } from 'react-router-dom';
import { CheckServerConnection } from './components/CheckServerConnection/CheckServerConnection';
import { Toolbar } from './components/DataTest/Toolbar';
import { DefaultLayout } from './components/DefaultLayout';
import { ProtectedRoute } from './components/ProtectedRoute/ProtectedRoute';
import HomePage from './pages/Home/Home.page';
import LoginPage from './pages/Login/Login.page';
import NotFoundPage from './pages/NotFound/NotFound.page';
import GoogleOAuthPage from './pages/OAuth/GoogleOAuth.page';
import SignUpPage from './pages/SignUp/SignUp.page';
import TripPage from './pages/Trips/Trip.page';

export function Router() {
  return (
    <Routes>
      <Route element={<CheckServerConnection />}>
        <Route path="/login" element={<LoginPage />} />
        <Route path="/signup" element={<SignUpPage />} />
        <Route path="/oauth/google" element={<GoogleOAuthPage />} />
        <Route element={<ProtectedRoute />}>
          <Route element={<Toolbar />}>
            <Route element={<DefaultLayout />}>
              <Route path="/" element={<HomePage />} />
              <Route path="/trip/:tripId" element={<TripPage />} />
            </Route>
          </Route>
        </Route>
      </Route>
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  );
}
