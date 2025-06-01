import { PlusIcon, SparklesIcon, MapPinIcon, UsersIcon, BookIcon, ArrowRightIcon, PlaneTakeoff, Calendar, Lightbulb, Map } from 'lucide-react';
import NewTripDialog from '../components/NewTripDialog';
import { useEffect, useState } from 'react';
import { Box, Button, Typography, Stack } from '@mui/material';
import Section from '../components/generic/Section';
export default function DashboardPage() {

  const [dialogOpen, setDialogOpen] = useState(false);
  const [recommendations, setRecommendations] = useState<{icon: any, label: string}[]>([]);

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
        { icon: <Calendar/>, label: 'Best times to travel' },
        { icon: <Lightbulb/>, label: 'Travel planning tips' },
        { icon: <Map/>, label: 'Explore new places' },
      ]);
    
    } catch (error) {
      console.error('Error fetching recommendations:', error);
    }
  }

  useEffect(() => {
    // Fetch recommendations when the component mounts
    fetchRecommendations();
  }, []);

  

  return (
    <>
      <header className="mb-8">
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
                sx={{'&:hover': {bgcolor: 'primary.dark'} }}
                style={{ 
                  color: 'white', 
                  fontWeight: 400, 
                  padding: '0.5rem 1rem', 
                  borderRadius: '0.375rem', 
                  display: 'inline-flex', 
                  alignItems: 'center', 
                  gap: '0.5rem' }}
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

              {/* Assistant Prompt Box */}
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
                    sx={{ '&:hover': { bgcolor: 'transparent' } }}
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
      </>
);
}