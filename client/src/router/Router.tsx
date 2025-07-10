import { Navigate, Route, Routes } from 'react-router-dom';
import DashboardPage from "../pages/DashboardPage";
import ExplorePage from '../pages/ExplorePage';
import TripPage from '../pages/TripPage';
import MapPage from '../pages/MapPage';
import JournalPage from '../pages/JournalPage';
import NotFoundPage from '../pages/NotFound';
import DefaultLayout from '../components/DefaultLayout';
import StatsPage from '../pages/StatsPage';
import { TripProvider } from '../providers/TripProvider';
import { ProtectedRoute } from '../components/ProtectedRoute';
import LoginPage from '../pages/LoginPage';
import RegisterPage from '../pages/RegisterPage';
import OAuthCallbackPage from '../pages/oauth/CallbackPage';
import SettingsLayout from '../components/settings/SettingsLayout';
import MyAccountPage from '../pages/settings/MyAccountPage';
import NotificationSettingsPage from '../pages/settings/NotificationSettingsPage';
import PreferencesPage from '../pages/settings/PreferencesPage';
import { VerifyPage } from '../pages/VerifyPage';


export default function Router() {
  /**
   * Router component that defines the application's routes.
   * It uses React Router to manage navigation between different pages.
   *
   * @returns {JSX.Element} The rendered routes.
   */
  return (
    <Routes>
      <Route path="login" element={<LoginPage />} />
      <Route path="register" element={<RegisterPage />} />
      <Route path="/oauth/callback" element={<OAuthCallbackPage />} />
      <Route path="verify" element={<VerifyPage />} />

      <Route element={<ProtectedRoute />}>
        <Route element={<DefaultLayout />}>
          <Route path="/" element={<DashboardPage />} />
          <Route path="/map" element={<MapPage />} />
          <Route path="/journal" element={<JournalPage />} />
          <Route path="/explore" element={<ExplorePage />} />
          <Route path="/stats" element={<StatsPage />} />
          <Route path="/trip/:id" element={
            <TripProvider>
              <TripPage />
            </TripProvider>
          } />

          {/* <Route path="/trip/new" element={<NewTripDialog open={open} onClose={onClose}/>} /> */}
        </Route>
        <Route path="settings" element={<SettingsLayout />}>
          <Route path="account" element={<MyAccountPage />} />
          <Route path="preferences" element={<PreferencesPage />} />
          <Route path="notifications" element={<NotificationSettingsPage />} />
        </ Route>
        {/* </Route> */}
      </Route>
      <Route path="*" element={<Navigate to="/" />} />
      <Route path="*" element={<NotFoundPage />} />
    </Routes >
  )

}
