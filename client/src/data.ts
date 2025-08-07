export type UserData = {
  id: string;
  avatar: string;
  username: string;
};

export type TripData = {
  id: string;
  title: string;
  owner: string;
  collaborators: string[]; // list of user uuid's
  createdAt: string;
  updatedAt: string;
  deleted: boolean;
  lastModifiedVersion: number;
};

export type ItineraryItemData = {
  id: string;
  title: string;
  type: string;
  startTime: string;
  endTime?: string;
  location?: string;
  cost?: number;
  deleted: boolean;
  lastModifiedVersion: number;
  tripId: string;
};

export type BudgetData = {
  id: string;
  currency: string;
  totalBudget: number;
  accommodationBudget: number;
  transportationBudget: number;
  foodDiningBudget: number;
  activitiesBudget: number;
  shoppingBudget: number;
  deleted: boolean;
  lastModifiedVersion: number;
  tripId: string;
};

export type ExpenseData = {
  id: string;
  title: string;
  type: string;
  cost: number;
  currency: string;
  payers: string[]; // list of user uuid's
  deleted: boolean;
  lastModifiedVersion: number;
  spaceId: string;
}


