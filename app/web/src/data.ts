export type UserData = {
  id: string;
  avatar: string;
  username: string;
};

export type TripData = {
  id: string;
  title: string;
  owner: string;
  collaborators: UserData[];
  createdAt: string;
  updatedAt: string;
};

export type ItineraryItemData = {
  id: string;
  title: string;
  type: string;
  startTime: string;
  endTime?: string;
  location?: string;
  cost?: number;
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
  tripId: string;
};

export type ExpenseData = {
  id: string;
  title: string;
  type: string;
  cost: number;
  currency: string;
  payers: UserData[];
  spaceId: string;
};
