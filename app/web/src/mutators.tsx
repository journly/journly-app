import { nanoid } from 'nanoid';
import { WriteTransaction } from 'replicache';
import {
  createCollaborator,
  deleteCollaborator,
  getCollaboratorsByTrip,
  updateCollaborator,
} from './models/collaborators';
import {
  createExpensePayer,
  deleteExpensePayer,
  getExpensePayersByExpense,
} from './models/expensePayer';
import { createExpense, deleteExpense, getExpensesByTrip, updateExpense } from './models/expenses';
import {
  createItineraryItem,
  deleteItineraryItem,
  getItineraryItemsByTrip,
  updateItineraryItem,
} from './models/itineraryItem';
import { createTrip, deleteTrip, TripCreate, updateTrip } from './models/trip';

export type Mutators = typeof mutators;

export const mutators = {
  updateTrip,
  createCollaborator,
  deleteCollaborator,
  updateCollaborator,
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
      email: user.email,
      avatarUrl: user.avatar,
      role: 'owner',
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
  deleteTrip: async (tx: WriteTransaction, tripId: string) => {
    const collaborators = await getCollaboratorsByTrip(tx, tripId);

    await Promise.all(collaborators.map((collaborator) => deleteCollaborator(tx, collaborator.id)));

    const expenses = await getExpensesByTrip(tx, tripId);

    await Promise.all(
      expenses.map(async (expense) => {
        const expensePayers = await getExpensePayersByExpense(tx, expense.id);
        await Promise.all(
          expensePayers.map((expensePayer) =>
            expensePayer ? deleteExpensePayer(tx, expensePayer.id) : null
          )
        );
        await deleteExpense(tx, expense.id);
      })
    );

    const itineraryItems = await getItineraryItemsByTrip(tx, tripId);

    await Promise.all(
      itineraryItems.map((itineraryItem) => deleteItineraryItem(tx, itineraryItem.id))
    );

    await deleteTrip(tx, tripId);
  },
};
