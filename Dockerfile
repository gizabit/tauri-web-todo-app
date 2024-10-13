#
# backend build stage
#
FROM rust:1.81 as backend-builder

# cache dependencies of the logic library
WORKDIR /usr/src/logic
COPY logic/Cargo.toml logic/Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    touch src/lib.rs && \
    cargo build --release

# cache dependencies of the backend
WORKDIR /usr/src/backend
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release

# build the backend 
RUN rm -rf /usr/src 
WORKDIR /usr/src/backend
COPY logic /usr/src/logic
COPY backend .
RUN cargo build --release

#
# frontend build stage
#
FROM node:22-alpine as frontend-builder

# cache dependencies of the frontend
WORKDIR /usr/src/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN npm install

# build the frontend
COPY frontend .
RUN npm run build

#
# final stage
#
FROM debian:bookworm-slim

ARG LOCAL_PORT=80
ARG STATIC_FILE_PATH=/var/www/frontend
ARG BACKEND_URL=http://localhost:80

ENV LOCAL_PORT=${LOCAL_PORT}
ENV STATIC_FILE_PATH=${STATIC_FILE_PATH}
ENV BACKEND_URL=${BACKEND_URL}

EXPOSE ${LOCAL_PORT}

WORKDIR /app
COPY --from=backend-builder /usr/src/backend/target/release/backend .
COPY --from=frontend-builder /usr/src/frontend/build ${STATIC_FILE_PATH}

# Create the start_backend.sh script that updates the remote.js file with the BACKEND_URL
RUN echo '#!/bin/sh' > start_backend.sh && \
    echo "echo \"window.APP_BACKEND_URL = '\$BACKEND_URL';\" > \$STATIC_FILE_PATH/backend.js" >> start_backend.sh && \
    echo "./backend" >> start_backend.sh && \
    chmod +x start_backend.sh

CMD ["./start_backend.sh"]
