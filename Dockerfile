# Stage 1: Build
FROM rust:slim-bullseye AS builder

WORKDIR /jwt_in_rust

# Instalar dependências para build
RUN apt-get update &&  apt install -y libpq-dev

# Copiar o código-fonte para o container
COPY . .

# Compilar o projeto em modo release
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bullseye-slim

ENV DATABASE_URL=postgres://alex:alex@localhost/postgres

WORKDIR /jwt_in_rust

EXPOSE 8080

# Instalar dependências de runtime
RUN apt-get update &&  apt install -y libpq-dev
# Copiar apenas o binário compilado do estágio anterior
COPY --from=builder /jwt_in_rust/target/release/jwt_in_rust .

# Configurar o comando padrão
CMD ["./jwt_in_rust"]
