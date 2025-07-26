import { generate, ReadTransaction } from '@rocicorp/rails';
import { z } from 'zod';

export const collaboratorSchema = z.object({
  id: z.string(),
  tripId: z.string(),
  userId: z.string(),
  username: z.string(),
  avatarUrl: z.string().optional(),
});

export type Collaborator = z.infer<typeof collaboratorSchema>;

export const {
  init: createCollaborator,
  list: listCollaborators,
  get: getCollaborator,
  delete: deleteCollaborator,
} = generate('collaborator', collaboratorSchema.parse);

// returns a list of collaborators for a given trip
export async function getCollaboratorsByTrip(tx: ReadTransaction, tripId: string) {
  const allCollaborators = await listCollaborators(tx);

  return allCollaborators.filter((collaborator) => collaborator.tripId === tripId);
}
