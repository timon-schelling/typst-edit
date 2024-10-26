FROM rust:alpine AS builder

RUN apk add --no-cache curl musl-dev

RUN cd tmp && \ 
    curl -L https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz -o typst.tar.xz && \
    tar -xvf typst.tar.xz && \
    mv ./typst*/typst /usr/local/bin/typst && \
    rm -r /tmp/*

RUN cargo install --git https://github.com/ItsEthra/typst-live
RUN ls usr/local/bin && cp /usr/local/cargo/bin/typst-live /usr/local/bin/typst-live

COPY . /src
WORKDIR /src
RUN cargo install --path .
RUN cp /usr/local/cargo/bin/typst-edit /usr/local/bin/typst-edit

FROM busybox

COPY --from=builder /usr/local/bin/typst /usr/local/bin/typst
COPY --from=builder /usr/local/bin/typst-live /usr/local/bin/typst-live
COPY --from=builder /usr/local/bin/typst-edit /usr/local/bin/typst-edit

ENV PATH="/usr/local/bin:${PATH}"

WORKDIR /workspace

ENTRYPOINT [ "typst-edit" ]
