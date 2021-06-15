FROM liuchong/rustup

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=5000

COPY ./modulo/src /modulo/src
ADD modulo/Cargo.lock /modulo/Cargo.lock
ADD modulo/Cargo.toml /modulo/Cargo.toml

WORKDIR /modulo

RUN rustup default nightly

RUN rustup update
RUN cargo build

CMD ["cargo", "run"]