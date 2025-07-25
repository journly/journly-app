import { WriteTransaction } from 'replicache';
import { ExpenseData, ItineraryItemData, TripData } from './data';

export const mutators = {
  createTrip: async (tx: WriteTransaction, args: TripData) => {
    await tx.set(`trip/${args.id}`, args);
  },
  updateTrip: async (tx: WriteTransaction, args: { id: string } & Partial<TripData>) => {
    const trip = await tx.get<TripData>(`trip/${args.id}`);
    await tx.set(`trip/${args.id}`, { ...trip, ...args });
  },
  deleteTrip: async (tx: WriteTransaction, id: string) => {
    await tx.del(`trip/${id}`);
  },
  createItineraryItem: async (tx: WriteTransaction, args: ItineraryItemData) => {
    await tx.set(`trip/${args.id}`, args);
  },
  updateItineraryItem: async (
    tx: WriteTransaction,
    args: { id: string } & Partial<ItineraryItemData>
  ) => {
    const item = await tx.get<ItineraryItemData>(`itineraryItem/${args.id}`);
    await tx.set(`itineraryItem/${args.id}`, { ...item, ...args });
  },
  deleteItineraryItem: async (tx: WriteTransaction, id: string) => {
    await tx.del(`itineraryItem/${id}`);
  },
  createExpense: async (tx: WriteTransaction, args: ExpenseData) => {
    await tx.set(`expense/${args.id}`, args);
  },
  updateExpense: async (tx: WriteTransaction, args: { id: string } & Partial<ExpenseData>) => {
    const expense = await tx.get<ExpenseData>(`expense/${args.id}`);
    await tx.set(`expense/${args.id}`, { ...expense, ...args });
  },
  deleteExpense: async (tx: WriteTransaction, id: string) => {
    await tx.del(`expense/${id}`);
  },
};
