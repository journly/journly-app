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
import {
  createExpense,
  deleteExpense,
  ExpenseUpdate,
  getExpensesByTrip,
  updateExpense,
} from './models/expenses';
import {
  createItineraryItem,
  deleteItineraryItem,
  getItineraryItemsByTrip,
  ItineraryItemUpdate,
  updateItineraryItem,
} from './models/itineraryItem';
import { createTask, deleteTask, getTasksByTrip, TaskCreate, updateTask } from './models/tasks';
import { createTrip, deleteTrip, TripCreate, updateTrip } from './models/trip';
import { getNextPosition } from './utils/positioning';

export type Mutators = typeof mutators;

export const mutators = {
  updateTrip,
  createCollaborator,
  deleteCollaborator,
  updateCollaborator,
  createItineraryItem,
  deleteItineraryItem,
  updateItineraryItem: async (
    tx: WriteTransaction,
    args: { itineraryItem: ItineraryItemUpdate; tripId: string }
  ) => {
    const { itineraryItem, tripId } = args;
    await updateItineraryItem(tx, itineraryItem);

    await updateTrip(tx, {
      id: tripId,
      updatedAt: new Date().toISOString(),
    });
  },
  createExpense,
  deleteExpense,
  updateExpense: async (tx: WriteTransaction, args: { expense: ExpenseUpdate; tripId: string }) => {
    const { expense, tripId } = args;
    await updateExpense(tx, expense);

    await updateTrip(tx, {
      id: tripId,
      updatedAt: new Date().toISOString(),
    });
  },
  createExpensePayer,
  deleteExpensePayer,
  createTask: async (tx: WriteTransaction, task: TaskCreate) => {
    const tripId = task.tripId;
    const tasks = await getTasksByTrip(tx, tripId);

    // Use string-based fractional indexing for position
    const position = tasks.length > 0 ? getNextPosition(tasks[tasks.length - 1].position) : 'a';
    await createTask(tx, { ...task, position: position.toString(), id: nanoid() });

    await updateTrip(tx, {
      id: tripId,
      updatedAt: new Date().toISOString(),
    });
  },
  updateTask,
  deleteTask,
  createTrip: async (tx: WriteTransaction, args: { trip: TripCreate; user: User }) => {
    const { trip, user } = args;
    let collaboratorId = nanoid();
    let tripId = nanoid();

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

    const tasks = await getTasksByTrip(tx, tripId);

    await Promise.all(tasks.map((task) => deleteTask(tx, task.id)));

    await deleteTrip(tx, tripId);
  },
};
