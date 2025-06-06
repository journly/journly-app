import { Route, Routes } from 'react-router-dom';
import DashboardPage from "../pages/DashboardPage";
import ExplorePage from '../pages/ExplorePage';
import TripPage from '../pages/TripPage';
import MapPage from '../pages/MapPage';
import JournalPage from '../pages/JournalPage';
import NotFoundPage from '../pages/NotFound';
import DefaultLayout from '../components/DefaultLayout';
import SettingsPage from '../pages/SettingsPage';
// import NewTripDialog from '../components/NewTripDialog';
import StatsPage from '../pages/StatsPage';
import { TripProvider } from '../providers/TripProvider';


export default function Router() {
    /**
     * Router component that defines the application's routes.
     * It uses React Router to manage navigation between different pages.
     * 
     * @returns {JSX.Element} The rendered routes.
     */
    return ( 
        <Routes>
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
            {/* <Route element={<SettingLayout />}> */}
                <Route path="/settings" element={<SettingsPage />} />
            {/* </Route> */}
            
            <Route path="*" element={<NotFoundPage />} />
        </Routes>
    )

}
