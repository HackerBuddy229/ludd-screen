FROM rust:1.58

COPY /. /.
COPY arch.png target/release/.

RUN ["cargo", "build", "--release"]

WORKDIR target/release/

CMD ["./luddScreen"]