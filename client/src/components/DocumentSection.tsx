import React from 'react';
import { FileTextIcon, FileImageIcon, FileIcon, PlusIcon } from 'lucide-react';
export function DocumentSection() {
  return <div className="bg-white border border-gray-200 rounded-lg p-5">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium">Travel Documents</h2>
        <button className="text-blue-600 hover:text-blue-700">
          <PlusIcon className="h-4 w-4" />
        </button>
      </div>
      <div className="space-y-3 mb-3">
        <DocumentItem icon={<FileTextIcon className="h-4 w-4 text-blue-600" />} name="Flight_Reservation.pdf" size="1.2 MB" />
        <DocumentItem icon={<FileImageIcon className="h-4 w-4 text-green-600" />} name="Passport_Scan.jpg" size="3.4 MB" />
        <DocumentItem icon={<FileIcon className="h-4 w-4 text-orange-600" />} name="Visa_Application.docx" size="845 KB" />
        <DocumentItem icon={<FileTextIcon className="h-4 w-4 text-blue-600" />} name="Hotel_Booking.pdf" size="1.8 MB" />
      </div>
      <button className="w-full text-sm text-blue-600 hover:text-blue-700 font-medium">
        View all documents
      </button>
    </div>;
}
function DocumentItem({
  icon,
  name,
  size
}) {
  return <div className="flex items-center justify-between p-2 hover:bg-gray-50 rounded">
      <div className="flex items-center gap-2">
        <div className="w-8 h-8 rounded-full bg-gray-100 flex items-center justify-center">
          {icon}
        </div>
        <div>
          <p className="text-sm font-medium">{name}</p>
          <p className="text-xs text-gray-500">{size}</p>
        </div>
      </div>
      <button className="text-gray-400 hover:text-gray-600">
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
        </svg>
      </button>
    </div>;
}