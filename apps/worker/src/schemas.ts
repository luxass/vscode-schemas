import { z } from '@hono/zod-openapi'

export const RELEASE_SCHEMA = z.object({
  tag: z.string(),
  url: z.string(),
}).openapi('Release')

export const LATEST_RELEASE_SCHEMA = z.object({
  tag: z.string(),
  url: z.string(),
  commit: z.string(),
}).openapi('Latest Release')

// export const CONTRIBUTES_SCHEMA = z.object({

// })
