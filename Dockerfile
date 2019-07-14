FROM rustlang/rust:nightly

WORKDIR /imageloader

COPY . /imageloader

RUN cargo build --release

EXPOSE 8000

CMD ["cargo", "run --release"]