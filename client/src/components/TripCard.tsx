import React from 'react';
import { CalendarIcon, UsersIcon, MoreHorizontalIcon } from 'lucide-react';

interface TripCardProps {
  title: string;
  image: string;
  progress: string;
  date: string;
}

export function TripCard({
  title,
  date,
  image,
  progress,
  collaborators
}) {
  return <div className="bg-white border border-gray-200 rounded-lg overflow-hidden hover:shadow-md transition-shadow">
    <div className="h-40 overflow-hidden">
      <img src={image} alt={title} className="w-full h-full object-cover" />
    </div>
    <div className="p-4">
      <div className="flex justify-between items-start mb-2">
        <h3 className="font-medium text-gray-900">{title}</h3>
        <button className="text-gray-500 hover:text-gray-700">
          <MoreHorizontalIcon className="h-4 w-4" />
        </button>
      </div>
      <div className="flex items-center text-sm text-gray-500 mb-3">
        <CalendarIcon className="h-3.5 w-3.5 mr-1" />
        <span>{date}</span>
      </div>
      <div className="mb-3">
        <div className="flex justify-between text-xs mb-1">
          <span className="text-gray-600">Planning progress</span>
          <span className="font-medium">{progress}%</span>
        </div>
        <div className="w-full bg-gray-100 rounded-full h-1.5">
          <div className="bg-blue-600 h-1.5 rounded-full" style={{
            width: `${progress}%`
          }}></div>
        </div>
      </div>
      <div className="flex justify-between items-center">
        <div className="flex items-center">
          <UsersIcon className="h-3.5 w-3.5 text-gray-500 mr-1" />
          <span className="text-xs text-gray-500">
            {collaborators} collaborators
          </span>
        </div>
        <button className="text-xs text-blue-600 font-medium hover:text-blue-700">
          View Details
        </button>
      </div>
    </div>
  </div>;
}
