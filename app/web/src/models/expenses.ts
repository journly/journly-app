import { generate, ReadTransaction, Update } from '@rocicorp/rails';
import { z } from 'zod';

export const expenseSchema = z.object({
  id: z.string(),
  tripId: z.string(),
  name: z.string(),
  description: z.string().optional(),
  amount: z.number(),
  currency: z.string(),
  category: z.string().optional(),
});

export type Expense = z.infer<typeof expenseSchema>;
export type ExpenseUpdate = Update<Expense>;

export const {
  init: createExpense,
  list: listExpenses,
  get: getExpense,
  delete: deleteExpense,
  update: updateExpense,
} = generate('expense', expenseSchema.parse);

// returns a list of expenses for a given trip
export async function expensesByTrip(tx: ReadTransaction, tripId: string) {
  const allExpenses = await listExpenses(tx);
  return allExpenses.filter((expense) => expense.tripId === tripId);
}
