import { PlusIcon, SparklesIcon, MapPinIcon, ArrowRightIcon, UsersIcon, BookIcon } from 'lucide-react';
import NewTripDialog from '../components/NewTripDialog';
import { useState } from 'react';
export default function DashboardPage() {

  const [dialogOpen, setDialogOpen] = useState(false);

  const handleOpen = () => setDialogOpen(true);

  return (
    <>
      <header className="mb-8">
        <h1 className="text-2xl font-semibold text-gray-900">
          Welcome to Journly!
        </h1>
        <p className="text-gray-600 mt-1">
          Start planning your next adventure today.
        </p>
      </header>
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6 mb-8">
        <div className="lg:col-span-2">
          <div className="bg-white border border-gray-200 rounded-lg overflow-hidden h-full">
            <div className="p-12 flex flex-col items-center justify-center text-center">
              <div className="w-16 h-16 bg-blue-50 rounded-full flex items-center justify-center mb-6">
                <MapPinIcon className="h-8 w-8 text-blue-600" />
              </div>
              <h2 className="text-xl font-semibold text-gray-900 mb-2">
                Plan Your First Trip
              </h2>
              <p className="text-gray-600 mb-6 max-w-md">
                Start by creating a new trip. Add destinations, plan your
                itinerary, and invite travel companions to join your adventure.
              </p>
              <button className="inline-flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors" onClick={handleOpen}>
                <PlusIcon className="h-5 w-5" />
                Create New Trip
              </button>
            </div>
          </div>
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
                  Need help planning your first trip?
                </p>
                <button className="mt-2 text-sm font-medium text-blue-600 hover:text-blue-700">
                  Get personalized suggestions
                </button>
              </div>
            </div>
            <div className="space-y-3">
              <button className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm hover:bg-gray-50 flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span>✈️</span> Popular destinations
                </div>
                <ArrowRightIcon className="h-4 w-4 text-gray-400" />
              </button>
              <button className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm hover:bg-gray-50 flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span>📅</span> Best times to travel
                </div>
                <ArrowRightIcon className="h-4 w-4 text-gray-400" />
              </button>
              <button className="w-full text-left px-3 py-2 border border-gray-200 rounded-md text-sm hover:bg-gray-50 flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <span>💡</span> Travel planning tips
                </div>
                <ArrowRightIcon className="h-4 w-4 text-gray-400" />
              </button>
            </div>
          </div>
        </div>
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <div className="bg-white border border-gray-200 rounded-lg p-8 text-center">
          <div className="w-12 h-12 bg-blue-50 rounded-full flex items-center justify-center mx-auto mb-4">
            <SparklesIcon className="h-6 w-6 text-blue-600" />
          </div>
          <h3 className="text-gray-900 font-medium mb-2">Get Started</h3>
          <p className="text-sm text-gray-600 mb-4">
            Create your first trip to start tracking expenses, storing
            documents, and setting reminders.
          </p>
          <button className="text-sm text-blue-600 hover:text-blue-700 font-medium">
            Learn how it works
          </button>
        </div>
        <div className="bg-white border border-gray-200 rounded-lg p-8 text-center">
          <div className="w-12 h-12 bg-green-50 rounded-full flex items-center justify-center mx-auto mb-4">
            <UsersIcon className="h-6 w-6 text-green-600" />
          </div>
          <h3 className="text-gray-900 font-medium mb-2">Travel Together</h3>
          <p className="text-sm text-gray-600 mb-4">
            Invite friends and family to plan trips together and share the
            excitement.
          </p>
          <button className="text-sm text-blue-600 hover:text-blue-700 font-medium">
            Invite travelers
          </button>
        </div>
        <div className="bg-white border border-gray-200 rounded-lg p-8 text-center">
          <div className="w-12 h-12 bg-purple-50 rounded-full flex items-center justify-center mx-auto mb-4">
            <BookIcon className="h-6 w-6 text-purple-600" />
          </div>
          <h3 className="text-gray-900 font-medium mb-2">Travel Journal</h3>
          <p className="text-sm text-gray-600 mb-4">
            Document your adventures and create lasting memories of your
            travels.
          </p>
          <button className="text-sm text-blue-600 hover:text-blue-700 font-medium">
            Start writing
          </button>
        </div>
      </div>
      <NewTripDialog open={dialogOpen} onClose={() => setDialogOpen(false)} />
      </>
);
}