FROM rust:1-bookworm AS builder
WORKDIR /src
COPY . .
RUN cargo build --release -p sdkwork-llm-api-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /src/target/release/sdkwork-llm-api-server /usr/local/bin/sdkwork-llm-api-server
COPY --from=builder /src/database /app/database
ENV SDKWORK_LLM_APP_ROOT=/app
ENV SDKWORK_LLM_ENVIRONMENT=production
ENV SDKWORK_LLM_CONFIG_PROFILE=production
ENV SDKWORK_LLM_APPLICATION_PUBLIC_INGRESS_BIND=0.0.0.0:8080
EXPOSE 8080
CMD ["sdkwork-llm-api-server"]
