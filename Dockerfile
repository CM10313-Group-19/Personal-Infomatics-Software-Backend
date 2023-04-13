FROM rust:1.65 as builder

WORKDIR /project/user-app

# Copy config files
COPY Cargo.toml Cargo.toml
COPY Rocket_docker.toml Rocket.toml

# Copy code
COPY src ./src

ENV DATABASE_URL=mysql://root@host.docker.internal/test
EXPOSE 3306

RUN cargo build --release



FROM debian:bullseye-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /project/user-app/Cargo.toml /project/user-app/Rocket.toml /project/user-app/target/release/personal_infomatics_software_backend ./

EXPOSE 8000

CMD [ "./personal_infomatics_software_backend" ]



