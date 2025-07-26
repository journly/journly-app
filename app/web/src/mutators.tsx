import { createCollaborator, deleteCollaborator } from './models/collaborators';
import { createExpensePayer, deleteExpensePayer } from './models/expensePayer';
import { createExpense, deleteExpense, updateExpense } from './models/expenses';
import {
  createItineraryItem,
  deleteItineraryItem,
  updateItineraryItem,
} from './models/itineraryItem';
import { createTrip, deleteTrip, updateTrip } from './models/trip';

export type Mutators = typeof mutators;

export const mutators = {
  createTrip,
  updateTrip,
  deleteTrip,
  createCollaborator,
  deleteCollaborator,
  createItineraryItem,
  deleteItineraryItem,
  updateItineraryItem,
  createExpense,
  deleteExpense,
  updateExpense,
  createExpensePayer,
  deleteExpensePayer,
};
