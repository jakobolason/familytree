FROM node:24-bookworm AS build
WORKDIR /app

# Enable pnpm
RUN corepack enable

COPY package.json pnpm-lock.yaml .npmrc ./
RUN pnpm i --frozen-lockfile

COPY . ./
RUN pnpm run build

FROM node:24-slim
WORKDIR /app

ENV NODE_ENV=production

# Copy only the output from the build stage
COPY --from=build /app/.output/ ./

# Port settings
ENV PORT=3000
ENV HOST=0.0.0.0

EXPOSE 3000

CMD ["node", "server/index.mjs"]
