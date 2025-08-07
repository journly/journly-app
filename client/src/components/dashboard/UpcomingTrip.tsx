import { CalendarIcon, ClockIcon, UsersIcon, CheckIcon, MapPinIcon } from 'lucide-react';

export function UpcomingTrip() {
  return <div className="bg-white border border-gray-200 rounded-lg overflow-hidden h-full">
    <div className="h-40 relative">
      <img src="https://images.unsplash.com/photo-1526481280693-3bfa7568e0f3?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=1471&q=80" alt="Japan" className="w-full h-full object-cover" />
      <div className="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent flex items-end">
        <div className="p-4 text-white">
          <h2 className="text-xl font-semibold">Japan Adventure</h2>
          <div className="flex items-center mt-1">
            <CalendarIcon className="h-4 w-4 mr-1" />
            <span className="text-sm">Nov 15-28, 2023</span>
          </div>
        </div>
      </div>
    </div>
    <div className="p-4">
      <div className="flex flex-wrap gap-3 mb-4">
        <div className="flex items-center bg-blue-50 text-blue-700 text-xs font-medium px-2.5 py-1 rounded">
          <ClockIcon className="h-3 w-3 mr-1" />
          13 days left
        </div>
        <div className="flex items-center bg-gray-100 text-gray-700 text-xs font-medium px-2.5 py-1 rounded">
          <UsersIcon className="h-3 w-3 mr-1" />3 collaborators
        </div>
        <div className="flex items-center bg-green-50 text-green-700 text-xs font-medium px-2.5 py-1 rounded">
          <CheckIcon className="h-3 w-3 mr-1" />
          75% completed
        </div>
      </div>
      <div className="mb-4">
        <h3 className="text-sm font-medium text-gray-900 mb-2">
          Next steps:
        </h3>
        <ul className="space-y-2">
          <li className="flex items-start gap-2">
            <div className="w-4 h-4 rounded border border-gray-300 mt-0.5"></div>
            <span className="text-sm text-gray-700">
              Book airport transfer in Tokyo
            </span>
          </li>
          <li className="flex items-start gap-2">
            <div className="w-4 h-4 rounded border border-gray-300 mt-0.5"></div>
            <span className="text-sm text-gray-700">
              Confirm hotel reservation in Kyoto
            </span>
          </li>
          <li className="flex items-start gap-2">
            <div className="w-4 h-4 rounded border border-gray-300 mt-0.5"></div>
            <span className="text-sm text-gray-700">
              Purchase Japan Rail Pass
            </span>
          </li>
        </ul>
      </div>
      <div className="flex items-center text-sm text-gray-500 mb-3">
        <MapPinIcon className="h-4 w-4 mr-1 text-gray-400" />
        <span>Tokyo → Kyoto → Osaka</span>
      </div>
      <div className="flex space-x-2">
        <button className="flex-1 bg-blue-600 text-white text-sm font-medium py-2 px-3 rounded-md hover:bg-blue-700">
          Continue Planning
        </button>
        <button className="bg-gray-100 text-gray-700 text-sm font-medium py-2 px-3 rounded-md hover:bg-gray-200">
          Invite
        </button>
      </div>
    </div>
  </div>;
}
