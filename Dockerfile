FROM debian:latest AS buildenv

ENV APT_PACKAGES build-essential ca-certificates curl
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get upgrade --yes \
    && apt-get install --yes --no-install-recommends ${APT_PACKAGES}

RUN adduser --disabled-password --uid=1000 rust
USER rust
WORKDIR /home/rust/

ADD --chown=rust:rust https://sh.rustup.rs rustup.sh
RUN sh rustup.sh -y
COPY --chown=rust:rust ./ ws2812b.cgi/
RUN .cargo/bin/cargo install --path=ws2812b.cgi/


FROM debian:latest

ENV APT_PACKAGES fcgiwrap nginx supervisor
ENV DEBIAN_FRONTEND noninteractive
RUN apt-get update \
    && apt-get upgrade --yes \
    && apt-get install --yes --no-install-recommends ${APT_PACKAGES} \
    && apt-get autoremove --yes \
    && apt-get clean

RUN rm -rf /etc/nginx/*
COPY ./docker/mime.types /etc/nginx/mime.types
COPY ./docker/nginx.conf /etc/nginx/nginx.conf
COPY ./docker/fastcgi.conf /etc/nginx/fastcgi.conf
COPY ./docker/supervisord.conf /etc/supervisord.conf

COPY --from=buildenv --chown=root:root /home/rust/.cargo/bin/ws2812b /var/www-data/ws2812b.cgi

CMD ["/usr/bin/supervisord", "-c", "/etc/supervisord.conf"]
