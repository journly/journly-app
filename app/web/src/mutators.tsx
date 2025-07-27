import { nanoid } from 'nanoid';
import { WriteTransaction } from 'replicache';
import { createCollaborator, deleteCollaborator } from './models/collaborators';
import { createExpensePayer, deleteExpensePayer } from './models/expensePayer';
import { createExpense, deleteExpense, updateExpense } from './models/expenses';
import {
  createItineraryItem,
  deleteItineraryItem,
  updateItineraryItem,
} from './models/itineraryItem';
import { createTrip, deleteTrip, TripCreate, updateTrip } from './models/trip';

export type Mutators = typeof mutators;

export const mutators = {
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
  createTrip: async (tx: WriteTransaction, args: { trip: TripCreate; user: User }) => {
    const { trip, user } = args;
    let collaboratorId = `collaborator/${nanoid()}`;
    let tripId = `trip/${nanoid()}`;

    await createCollaborator(tx, {
      id: collaboratorId,
      userId: user.id,
      tripId,
      username: user.username,
      avatarUrl: user.avatar,
    });

    await createTrip(tx, {
      ...trip,
      id: tripId,
      ownerId: collaboratorId,
      name: trip.name || 'Untitled Trip',
      createdAt: new Date().toISOString(),
      updatedAt: new Date().toISOString(),
    });
  },
};
