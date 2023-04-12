FROM rust:1.65 as builder

WORKDIR /project/user-app

# Copy config files
COPY Cargo.toml Cargo.toml
COPY Rocket.toml Rocket.toml

# Copy code
COPY src ./src

ENV DATABASE_URL=mysql://root@host.docker.internal/test
EXPOSE 3306

RUN cargo build 



FROM debian:bullseye-slim

COPY --from=builder /project/user-app/Cargo.toml /project/user-app/Rocket.toml /project/user-app/target/debug/ ./

EXPOSE 800

CMD [ "./personal_infomatics_software_backend" ]



