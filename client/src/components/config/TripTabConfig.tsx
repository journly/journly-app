import { CalendarDaysIcon, DollarSign, FileTextIcon, House, MapIcon } from 'lucide-react';

export enum TripTabs {
    OVERVIEW = 'overview',
    ITINERARY = 'itinerary',
    BOOKING = 'booking',
    BUDGET = 'budget',
    DOCUMENTS = 'documents'
}


export type Tab = {
  label: string;
  value: TripTabs;
  icon: any;
};

export const TripTabConfig: Tab[] = [
  {
    label: 'Overview',
    value: TripTabs.OVERVIEW,
    icon: <House className="h-6 w-6" />,
  },
  {
    label: 'Itinerary',
    value: TripTabs.ITINERARY,
    icon: <CalendarDaysIcon className="h-6 w-6" />,
  },
  {
    label: 'Booking',
    value: TripTabs.BOOKING,
    icon: <MapIcon className="h-6 w-6" />,
  },
  {
    label: 'Budget',
    value: TripTabs.BUDGET,
    icon: <DollarSign className="h-6 w-6" />,
  },
  {
    label: 'Documents',
    value: TripTabs.DOCUMENTS,
    icon: <FileTextIcon className="h-6 w-6" />,
  },
];