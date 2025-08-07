import { generate, Update } from '@rocicorp/rails';
import { ReadTransaction } from 'replicache';
import { z } from 'zod';

export const tripSchema = z.object({
  id: z.string(),
  ownerId: z.string(),
  name: z.string(),
  description: z.string().optional(),
  startDate: z.string().optional(),
  endDate: z.string().optional(),
  coverImage: z.string().optional(),
  createdAt: z.string(),
  updatedAt: z.string(),
});

export type Trip = z.infer<typeof tripSchema>;
export type TripUpdate = Update<Trip>;
export type TripCreate = Omit<Trip, 'id' | 'createdAt' | 'updatedAt' | 'ownerId'>;

export const {
  init: createTrip,
  list: listTrips,
  get: getTrip,
  delete: deleteTrip,
  update: updateTrip,
} = generate('trip', tripSchema.parse);

export async function listAllTripsSortedByUpdatedAt(tx: ReadTransaction) {
  const trips = await listTrips(tx);
  return trips.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime());
}
