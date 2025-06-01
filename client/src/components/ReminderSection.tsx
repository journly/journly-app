import React from 'react';
import { BellIcon, PlusIcon } from 'lucide-react';
export function ReminderSection() {
  return <div className="bg-white border border-gray-200 rounded-lg p-5">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium">Reminders</h2>
        <button className="text-blue-600 hover:text-blue-700">
          <PlusIcon className="h-4 w-4" />
        </button>
      </div>
      <div className="space-y-3 mb-3">
        <ReminderItem title="Apply for visa" date="Oct 15, 2023" priority="high" />
        <ReminderItem title="Purchase travel insurance" date="Oct 20, 2023" priority="medium" />
        <ReminderItem title="Exchange currency" date="Nov 10, 2023" priority="low" />
        <ReminderItem title="Check-in for flight" date="Nov 14, 2023" priority="medium" />
      </div>
      <button className="w-full text-sm text-blue-600 hover:text-blue-700 font-medium">
        View all reminders
      </button>
    </div>;
}
function ReminderItem({
  title,
  date,
  priority
}) {
  const priorityColors = {
    high: 'bg-red-100 text-red-700',
    medium: 'bg-yellow-100 text-yellow-700',
    low: 'bg-green-100 text-green-700'
  };
  return <div className="flex items-center justify-between p-2 hover:bg-gray-50 rounded">
      <div className="flex items-center gap-2">
        <div className="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
          <BellIcon className="h-4 w-4 text-blue-600" />
        </div>
        <div>
          <p className="text-sm font-medium">{title}</p>
          <p className="text-xs text-gray-500">{date}</p>
        </div>
      </div>
      <div className={`px-2 py-0.5 rounded-full text-xs font-medium ${priorityColors[priority]}`}>
        {priority}
      </div>
    </div>;
}