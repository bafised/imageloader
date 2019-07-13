FROM rustlang/rust:nightly

RUN mkdir -p /imageloader

WORKDIR /imageloader

COPY . /imageloader

EXPOSE 8000

RUN cargo build

CMD ["cargo", "run"]