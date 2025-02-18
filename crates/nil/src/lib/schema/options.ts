import { z } from 'zod';

export const world = z.object({
  name: z.string().trim().min(3).max(30),
  size: z.number().int().positive().safe().min(10).max(255),
});

export const player = z.object({
  id: z.string().trim().min(3).max(20),
});
