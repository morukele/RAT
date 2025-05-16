FROM rust:1.87.0-bullseye AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev

WORKDIR /rat

COPY . .

RUN cargo build -p server --release

RUN ls -l /rat/target/release/  # Debug: check if binary exists

FROM debian:bullseye-slim

ENV USER=user
ENV UID=10001

RUN addgroup --system "${USER}" && \
    adduser --system --ingroup "${USER}" --uid "${UID}" "${USER}"

WORKDIR /rat

# Replace 'server' with the actual binary name if different
COPY --from=builder /rat/target/release/server ./

USER ${USER}

CMD ["./server"]
