# --- Stage 1: Build (Use full image for build tools) ---
# Using LTS version (24) instead of 25 for stability
FROM node:24-bookworm AS build
WORKDIR /app

# Enable pnpm
RUN corepack enable

COPY package.json pnpm-lock.yaml .npmrc ./
RUN pnpm i --frozen-lockfile

COPY . ./
RUN pnpm run build

# --- Stage 2: Production (Use slim image for small size) ---
FROM node:24-slim
WORKDIR /app

# Set production environment (Nuxt uses this to optimize caching/logging)
ENV NODE_ENV=production

# Copy only the output from the build stage
COPY --from=build /app/.output/ ./

# Port settings
ENV PORT=3000
ENV HOST=0.0.0.0

# Optional: Add the memory fix explicitly here if you want to be safe
# ENV NODE_OPTIONS="--max-old-space-size=900"

EXPOSE 3000

CMD ["node", "server/index.mjs"]
