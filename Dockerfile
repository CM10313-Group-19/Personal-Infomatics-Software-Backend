FROM rust:1.65 as builder

WORKDIR /project/user-app

# Copy config files
COPY Cargo.toml Cargo.toml
COPY Rocket_docker.toml Rocket.toml
COPY sqlx-data.json sqlx-data.json
COPY migrations migrations

# Copy code
COPY src ./src

ENV DATABASE_URL=mysql://root@host.docker.internal/informatics
EXPOSE 3306

RUN cargo build 



FROM debian:bullseye-slim

RUN apt-get update && rm -rf /var/lib/apt/lists/*

COPY --from=builder /project/user-app/Cargo.toml /project/user-app/Rocket.toml /project/user-app/sqlx-data.json /project/user-app/target/debug/personal_infomatics_software_backend ./

EXPOSE 8000


CMD [ "./personal_infomatics_software_backend" ]



