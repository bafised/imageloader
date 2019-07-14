FROM rustlang/rust:nightly

WORKDIR /imageloader

COPY . /imageloader

RUN cargo build

EXPOSE 8000

CMD ["cargo", "run"]