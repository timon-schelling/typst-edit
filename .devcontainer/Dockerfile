FROM rust:alpine AS builder

RUN apk add --no-cache curl musl-dev

RUN cd tmp && \ 
    curl -L https://github.com/typst/typst/releases/latest/download/typst-x86_64-unknown-linux-musl.tar.xz -o typst.tar.xz && \
    tar -xvf typst.tar.xz && \
    mv ./typst*/typst /usr/local/bin/typst && \
    rm -r /tmp/*

RUN cargo install --git https://github.com/ItsEthra/typst-live --rev 550a69bea07d3948bbe9e2dfe0c45447b8f1bc83
RUN ls usr/local/bin && cp /usr/local/cargo/bin/typst-live /usr/local/bin/typst-live

FROM rust

COPY --from=builder /usr/local/bin/typst /usr/local/bin/typst
COPY --from=builder /usr/local/bin/typst-live /usr/local/bin/typst-live
