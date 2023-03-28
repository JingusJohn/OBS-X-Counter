import { z } from "zod";

export type FormatMode = "equation" | "colon";

export const counterSchema = z.object({
  title: z.string(),
  count: z.number(),
  formatMode: z.enum(["equation", "colon"]),
  allowNegative: z.boolean().default(false),
});

export type CounterType = z.TypeOf<typeof counterSchema>;

