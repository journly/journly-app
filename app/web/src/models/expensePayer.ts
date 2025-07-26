import { generate, ReadTransaction } from '@rocicorp/rails';
import { z } from 'zod';
import { getCollaborator, listCollaborators } from './collaborators';

export const expensePayerSchema = z.object({
  id: z.string(),
  expenseId: z.string(),
  collaboratorId: z.string(),
});

export type ExpensePayer = z.infer<typeof expensePayerSchema>;

export const {
  init: createExpensePayer,
  list: listExpensePayers,
  get: getExpensePayer,
  delete: deleteExpensePayer,
} = generate('expensePayer', expensePayerSchema.parse);

// returns a list of users that paid for a given expense
export async function getExpensePayersByExpense(tx: ReadTransaction, expenseId: string) {
  const allExpensePayers = await listExpensePayers(tx);

  const payers = allExpensePayers.filter((expensePayer) => expensePayer.expenseId === expenseId);

  const collaborators = await Promise.all(
    payers.map((payer) => getCollaborator(tx, payer.collaboratorId))
  );

  return collaborators;
}
