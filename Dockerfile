FROM rust:1.67
WORKDIR /usr/src/dojo-discord-bot

COPY . .

RUN cargo install --path .

CMD ["dojo-discord-bot"]