import { HomeIcon, MapIcon, BookIcon, CompassIcon, SettingsIcon, PlusIcon, ChevronRight, ChevronLeft, SearchIcon, BellIcon, ReceiptIcon, ChartNoAxesColumn, ChartBar, ChartColumnIncreasing } from 'lucide-react';
import { TripItem } from './TripItem';
import { MenuItem } from './MenuItem';
import { Typography, Box, Avatar, List, Drawer, IconButton, Menu } from '@mui/material';
import { useUser } from '../../providers/UserProvider';
import { SearchBar } from '../generic/Search';
import { useTrips } from '../../providers/TripsProvider';
import { Link } from 'react-router-dom';
import { useState } from 'react';
import NewTripDialog from '../NewTripDialog';
import { NotificationPanel } from './NotificationPanel';
export function MenuBar() {
  const { user } = useUser();
  const { trips, updateTrips } = useTrips();
  const [collapsed, setCollapsed] = useState(false);
  const [openNew, setOpenNew] = useState(false);
  const [openNotif, setOpenNotif] = useState(false);

  const toggleCollapse = () => {
    setCollapsed((prev) => !prev);
    updateTrips(); // Refresh trips when collapsing/expanding
    console.log(trips)
  };

  return (
    <Box
      className={`transition-all duration-300 ease-in-out ${
        collapsed ? 'w-14' : 'w-64'
      } h-screen bg-white border-r border-gray-200`}
    >
      {collapsed ? (
        <Box className="flex flex-col items-center py-4 h-full">
          <Link to="/" className="mb-4">
            <img src="/favicon.png" alt="Journly Icon" className="w-10 h-10" />
          </Link>
          <List dense>
            <MenuItem icon={<HomeIcon />} link="/" />
            <MenuItem icon={<MapIcon />} link="/map" />
            <MenuItem icon={<BookIcon />} link="/journal" />
            <MenuItem icon={<CompassIcon />} link="/explore" />
            <MenuItem icon={<ChartColumnIncreasing />} link="/stats" />
          </List>
          <Box className="flex-grow" />
          <MenuItem icon={<BellIcon />} onClick={() => setOpenNotif(true)} />
          <MenuItem icon={<SettingsIcon />} link="/settings" />
          <IconButton onClick={toggleCollapse}>
            <ChevronRight />
          </IconButton>
        </Box>
      ) : (
        <nav className="flex flex-col h-full">
          <Box className="p-4 border-b border-gray-200">
            <Link to="/" className="flex items-center gap-2">
              <img src="/favicon.png" alt="Journly Icon" className="w-10 h-10" />
              <Typography variant="h6" color="secondary">
                Journly
              </Typography>
            </Link>
          </Box>
          <SearchBar placeholder="Search" />
          <Box className="flex-1 overflow-y-auto">
            <Box className="px-2 pt-1 border-t border-gray-200">
              <List dense>
                <MenuItem icon={<HomeIcon />} label="Dashboard" link="/" />
                <MenuItem icon={<MapIcon />} label="Map" link="/map" />
                <MenuItem icon={<BookIcon />} label="Journal" link="/journal" />
                <MenuItem icon={<CompassIcon />} label="Explore" link="/explore" />
                <MenuItem icon={<ChartColumnIncreasing />} label="Travel Stats" link="/stats" />
              </List>
            </Box>
            <Box className="px-3 py-2 border-t border-gray-200">
              <Typography
                className="text-xs font-medium tracking-wider px-3 mb-1"
                variant="body2"
                color="secondary"
              >
                Recent Trips
              </Typography>
              {trips.map((trip) => (
                <TripItem
                  key={trip.id}
                  name={trip.title}
                  date={`${trip.travelDates.startDate} - ${trip.travelDates.endDate}`}
                />
              ))}
              {/* placeholder for trips to display */}
              <TripItem name="Japan 2023" date="Nov 15–28" />
              <TripItem name="Paris Weekend" date="Dec 10–12" />
              <TripItem name="Bali Getaway" date="Jan 5–15" />
              <MenuItem
                icon={<PlusIcon />}
                label="New Trip"
                textColor="primary"
                iconColor="text-blue-600"
                onClick={() => setOpenNew(true)}
              />
              <NewTripDialog open={openNew} onClose={() => setOpenNew(false)}/>
            </Box>
          </Box>
          <Box className="p-3 border-t border-gray-200">
            <MenuItem icon={<BellIcon />} label="Notifications" onClick={() => setOpenNotif(true)}/>
            <MenuItem icon={<SettingsIcon />} label="Settings" link="/settings" />
            <Box className="flex items-center gap-2 px-3 py-2 mt-2">
              <Avatar className="w-8 h-8 rounded-full bg-blue-600 flex items-center justify-center font-medium">
                {user?.username.charAt(0).toUpperCase() || 'JD'}
              </Avatar>
              <Box className="flex-1">
                <p className="text-sm font-medium">{user?.username || 'John Doe'}</p>
                <p className="text-xs text-gray-500">{user?.email || 'john@example.com'}</p>
              </Box>
            </Box>
            <Box className="flex justify-end p-1">
              <IconButton onClick={toggleCollapse}>
                <ChevronLeft />
              </IconButton>
            </Box>
          </Box>
        </nav>
      )}
      <NotificationPanel open={openNotif} onClose={() => setOpenNotif(false)} />
    </Box>
  );
}


