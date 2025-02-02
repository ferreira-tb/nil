import { z } from 'zod';

export const worldConfig = z.object({
  size: z.number().int().positive().safe().min(10).max(255),
});

export const playerConfig = z.object({
  name: z.string().trim().min(3).max(20),
});
