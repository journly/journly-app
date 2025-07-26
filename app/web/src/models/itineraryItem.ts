import { generate, Update } from '@rocicorp/rails';
import { ReadTransaction } from 'replicache';
import { z } from 'zod';

export const itineraryItemSchema = z.object({
  id: z.string(),
  tripId: z.string(),
  name: z.string(),
  description: z.string().optional(),
  startDateTime: z.string(),
  endDateTime: z.string(),
  location: z.string().optional(),
  notes: z.string().optional(),
  expenseId: z.string().optional(),
});

export type ItineraryItem = z.infer<typeof itineraryItemSchema>;
export type ItineraryItemUpdate = Update<ItineraryItem>;

export const {
  init: createItineraryItem,
  update: updateItineraryItem,
  delete: deleteItineraryItem,
  list: listItineraryItems,
  get: getItineraryItem,
} = generate('itineraryItem', itineraryItemSchema.parse);

export async function itineraryItemsByTrip(tx: ReadTransaction, tripId: string) {
  const allItineraryItems = await listItineraryItems(tx);
  return allItineraryItems.filter((item) => item.tripId === tripId);
}
