# Stage 1: Build the frontend assets using Bun
FROM oven/bun:1-alpine AS bun-builder

# Set the working directory
WORKDIR /app/web

# Copy the frontend source code
COPY ./web ./

# Install dependencies and build the frontend
RUN bun install --frozen-lockfile && bun run build

# Build binaries
FROM rust:slim

RUN apt update && \
  apt install -y libsqlite3-dev
COPY --from=bun-builder /app/web/dist /app/web/dist

WORKDIR /app

COPY . .

RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

EXPOSE 8000

CMD [ "cargo", "run", "--release" ]
