import { generate, ReadTransaction, Update } from '@rocicorp/rails';
import { z } from 'zod';

export const TaskSchema = z.object({
  id: z.string(),
  tripId: z.string(),
  title: z.string(),
  description: z.string(),
  completed: z.boolean(),
  position: z.string(),
  urgency: z.enum(['low', 'medium', 'high']),
});

export type Task = z.infer<typeof TaskSchema>;
export type TaskUpdate = Update<Task>;
export type TaskCreate = Omit<Task, 'id' | 'position'>;

export const {
  init: createTask,
  list: listTasks,
  get: getTask,
  delete: deleteTask,
  update: updateTask,
} = generate('task', TaskSchema.parse);

export async function getTasksByTrip(tx: ReadTransaction, tripId: string) {
  const tasks = await listTasks(tx);
  return tasks
    .filter((task) => task.tripId === tripId)
    .sort((a, b) => a.position.localeCompare(b.position));
}
