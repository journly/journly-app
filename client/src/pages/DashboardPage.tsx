import { PlusIcon, SparklesIcon, MapPinIcon, UsersIcon, BookIcon, ArrowRightIcon, PlaneTakeoff, Calendar, Lightbulb, Map } from 'lucide-react';
import NewTripDialog from '../components/NewTripDialog';
import { useEffect, useState } from 'react';
import { Box, Button, Typography, Stack } from '@mui/material';
import Section from '../components/generic/Section';
import PageWrapper from '../components/generic/Page';
import { useSubscribe } from "replicache-react";
import { useReplicache } from '../providers/ReplicacheProvider';
import { TripData } from '../data';

export default function DashboardPage() {
  const replicache = useReplicache();
  const [dialogOpen, setDialogOpen] = useState(false);
  const [recommendations, setRecommendations] = useState<{ icon: any, label: string }[]>([]);

  /*
  const trips = useSubscribe(replicache, async (tx) => {
    const result = await tx.scan<TripData>({ prefix: 'trip/' })
      .values()
      .toArray();

    return result
      .filter((trip) => new Date(trip.createdAt).getTime() < Date.now() - 100)
      .sort((a, b) => a.updatedAt.localeCompare(b.updatedAt) * -1);
  }, {
    default: []
  })
*/
  const handleOpen = () => setDialogOpen(true);

  const fetchRecommendations = async () => {
    try {
      // const response = await fetch('/api/recommendations');
      // if (!response.ok) {
      //   throw new Error('Network response was not ok');
      // }
      // const data = await response.json();
      // max 4 recommendations
      // setRecommendations(data.slice(0, 4));
      setRecommendations([
        { icon: <PlaneTakeoff />, label: 'Popular destinations' },
        { icon: <Calendar />, label: 'Best times to travel' },
        { icon: <Lightbulb />, label: 'Travel planning tips' },
        { icon: <Map />, label: 'Explore new places' },
      ]);

    } catch (error) {
      console.error('Error fetching recommendations:', error);
    }
  }

  useEffect(() => {
    // Fetch recommendations when the component mounts
    fetchRecommendations();
  }, []);

  //TODO: render trips if they exist and do the same for budgets, tasks and documents.

  return (
    <PageWrapper>
      <header className="my-8">
        <Typography variant="h5" style={{ fontWeight: 600 }}>
          Welcome to Journly!
        </Typography>
        <p className="text-gray-600 mt-1">
          Start planning your next adventure today.
        </p>
      </header>
      <Box className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <Box className="lg:col-span-2">
          <Section>
            <Box className="p-12 flex flex-col items-center justify-center text-center">
              <div className="w-16 h-16 bg-blue-50 rounded-full flex items-center justify-center mb-6">
                <MapPinIcon className="h-8 w-8 text-blue-600" />
              </div>
              <Typography variant="h6" fontWeight="500" marginBottom={1} className="text-md font-semibold text-gray-900 mb-2">
                Plan Your First Trip
              </Typography>
              <p className="text-gray-600 mb-6 max-w-md">
                Start by creating a new trip. Add destinations, plan your
                itinerary, and invite travel companions to join your adventure.
              </p>
              <Button
                onClick={handleOpen}
                variant="contained"
                sx={{ '&:hover': { bgcolor: 'primary.dark' } }}
                style={{
                  color: 'white',
                  fontWeight: 400,
                  padding: '0.5rem 1rem',
                  borderRadius: '0.375rem',
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: '0.5rem'
                }}
              >
                <PlusIcon className="h-5 w-5" />
                Create New Trip
              </Button>
            </Box>
          </Section>
        </Box>
        <Box>
          <Section>
            <Typography variant="h6" fontWeight="500" fontSize="large" marginBottom={2}>
              AI Travel Assistant
            </Typography>

            <Box className="bg-blue-50 border border-blue-100 rounded-lg p-4 flex gap-3 mb-4">
              <Box className="mt-1">
                <SparklesIcon className="h-5 w-5 text-blue-500" />
              </Box>
              <Box className="flex-1 align-self-start">
                <p className="text-sm text-gray-700">
                  Need help planning your first trip?
                </p>
                <Button
                  variant="text"
                  style={{
                    color: '#2563EB',
                    fontWeight: 500,
                    textTransform: 'none',
                    padding: '0',
                    margin: '0',
                    borderRadius: '0.375rem'
                  }}
                >
                  Get personalized suggestions
                </Button>
              </Box>
            </Box>
            <Stack spacing={1.5} >
              {recommendations.map(({ icon, label }) => (
                <Button
                  key={label}
                  fullWidth
                  variant="outlined"
                  disableRipple
                  style={{
                    textTransform: 'none',
                    borderColor: '#E5E7EB',
                    color: '#374151',
                    justifyContent: 'space-between',
                    padding: '0.5rem 1rem',
                    borderRadius: '0.375rem',
                    display: 'flex',
                    alignItems: 'center'
                  }}
                >
                  <Box className="flex items-center gap-2">
                    {icon}
                    <p className='font-normal'>{label}</p>
                  </Box>
                  <ArrowRightIcon className="h-4 w-4 text-gray-400" />
                </Button>
              ))}
            </Stack>
          </Section>
        </Box>
      </Box>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <Section>
          <Box className="text-center">
            <Box className="w-12 h-12 bg-blue-50 rounded-full flex items-center justify-center mx-auto mb-4">
              <SparklesIcon className="h-6 w-6 text-blue-600" />
            </Box>
            <Typography variant="body1" fontWeight="500" marginBottom={1}>
              Get Started
            </Typography>
            <p className="text-sm text-gray-600 mb-4">
              Create your first trip to start tracking expenses, storing documents, and setting reminders.
            </p>
            <Button
              variant="text"
              disableRipple
              sx={{ '&:hover': { bgcolor: 'transparent' } }}
            >
              Learn how it works
            </Button>
          </Box>
        </Section>
        <Section>
          <Box className="text-center">
            <Box className="w-12 h-12 bg-green-50 rounded-full flex items-center justify-center mx-auto mb-4">
              <UsersIcon className="h-6 w-6 text-green-600" />
            </Box>
            <Typography variant="body1" fontWeight="500" marginBottom={1}>
              Travel Together
            </Typography>
            <p className="text-sm text-gray-600 mb-4">
              Invite friends and family to plan trips together and share the
              excitement.
            </p>
            <Button
              variant="text"
              disableRipple
              sx={{ '&:hover': { bgcolor: 'transparent' } }}
            >
              Invite travelers
            </Button>
          </Box>
        </Section>
        <Section>
          <Box className="text-center">
            <Box className="w-12 h-12 bg-purple-50 rounded-full flex items-center justify-center mx-auto mb-4">
              <BookIcon className="h-6 w-6 text-purple-600" />
            </Box>
            <Typography variant="body1" fontWeight="500" marginBottom={1}>
              Travel Journal
            </Typography>
            <p className="text-sm text-gray-600 mb-4">
              Document your adventures and create lasting memories of your
              travels.
            </p>
            <Button
              variant="text"
              sx={{ '&:hover': { bgcolor: 'transparent' } }}
            >
              Start writing
            </Button>
          </Box>
        </Section>

      </div>
      <NewTripDialog open={dialogOpen} onClose={() => setDialogOpen(false)} />
      {/*
      <div className="p-8 max-w-7xl mx-auto">
        <header className="mb-8">
          <h1 className="text-2xl font-semibold text-gray-900">
            Welcome back, John!
          </h1>
          <p className="text-gray-600 mt-1">
            Plan your next adventure or check your upcoming trips.
          </p>
        </header>
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
          <div className="lg:col-span-2">
            <UpcomingTrip />
          </div>
          <div>
            <div className="bg-white border border-gray-200 rounded-lg p-5 h-full">
              <h2 className="text-lg font-medium mb-4">AI Travel Assistant</h2>
              <div className="bg-blue-50 border border-blue-100 rounded-lg p-4 flex gap-3 mb-4">
                <div className="mt-1">
                  <SparklesIcon className="h-5 w-5 text-blue-500" />
                </div>
                <div>
                  <p className="text-sm text-gray-700">
                    Need travel recommendations for your Japan trip?
                  </p>
                  <button className="mt-2 text-sm font-medium text-blue-600 hover:text-blue-700">
                    Get recommendations
                  </button>
                </div>
              </div>
              <div className="space-y-3">
                <button className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm hover:bg-gray-50 flex items-center gap-2">
                  <span>🍽️</span> Restaurant recommendations
                </button>
                <button className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm hover:bg-gray-50 flex items-center gap-2">
                  <span>🏨</span> Hotel suggestions
                </button>
                <button className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm hover:bg-gray-50 flex items-center gap-2">
                  <span>🎭</span> Local activities
                </button>
              </div>
            </div>
          </div>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
          <BudgetSection />
          <DocumentSection />
          <ReminderSection />
        </div>
        <div className="mb-8">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-medium text-gray-900">Your Trips</h2>
            <button className="flex items-center gap-1 px-3 py-1.5 bg-blue-600 text-white rounded-md text-sm hover:bg-blue-700">
              <PlusIcon className="h-4 w-4" />
              New Trip
            </button>
          </div>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
            <TripCard
              title="Japan Adventure"
              date="Nov 15-28, 2023"
              image="https://images.unsplash.com/photo-1526481280693-3bfa7568e0f3?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1471&q=80"
              progress={75}
              collaborators={3}
            />
            <TripCard
              title="Paris Weekend"
              date="Dec 10-12, 2023"
              image="https://images.unsplash.com/photo-1502602898657-3e91760cbb34?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1473&q=80"
              progress={40}
              collaborators={2}
            />
            <TripCard
              title="Bali Getaway"
              date="Jan 5-15, 2024"
              image="https://images.unsplash.com/photo-1573790387438-4da905039392?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1471&q=80"
              progress={20}
              collaborators={4}
            />
          </div>
        </div>
      </div>
            */}
    </PageWrapper>
  );
}
