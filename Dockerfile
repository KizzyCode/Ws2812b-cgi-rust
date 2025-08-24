FROM debian:stable-slim AS buildenv

ENV APT_PACKAGES build-essential ca-certificates curl
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get upgrade --yes \
    && apt-get install --yes --no-install-recommends ${APT_PACKAGES}

RUN useradd --system --uid=10000 rust
USER rust
WORKDIR /home/rust/

RUN curl --tlsv1.3 --output rustup.sh https://sh.rustup.rs \
    && sh rustup.sh -y --profile minimal
COPY --chown=rust:rust ./ ws2812b.cgi/
RUN .cargo/bin/cargo install --path=ws2812b.cgi/


FROM debian:stable-slim

ENV APT_PACKAGES fcgiwrap nginx supervisor
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get upgrade --yes \
    && apt-get install --yes --no-install-recommends ${APT_PACKAGES} \
    && apt-get autoremove --yes \
    && apt-get clean

RUN usermod -u 10000 www-data
RUN rm -rf /etc/nginx/*
COPY ./docker/mime.types /etc/nginx/mime.types
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
COPY ./docker/fastcgi.conf /etc/nginx/fastcgi.conf
COPY ./docker/supervisord.conf /etc/supervisord.conf

COPY --from=buildenv --chown=root:root /home/rust/.cargo/bin/ws2812b /var/www-data/ws2812b.cgi

ENTRYPOINT ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
