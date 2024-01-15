FROM rust:1.71
WORKDIR /
COPY . /
RUN cargo build --release

FROM debian:stable-slim
COPY --from=0 /target/release/iboodrss /usr/local/bin/
RUN chmod +x /usr/local/bin/iboodrss
VOLUME /data
VOLUME /log
RUN apt update && apt install --no-install-recommends -y curl cron lighttpd ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/\* /var/tmp/*
COPY script.sh /script.sh
COPY lighttpd.conf /etc/lighttpd/lighttpd.conf
COPY crontab.txt /etc/cron.d/crontab.txt
COPY web/index.html /web/index.html
COPY lighttpd.conf /etc/lighttpd/lighttpd.conf
RUN chmod +x /script.sh
RUN crontab /etc/cron.d/crontab.txt

CMD ["sh", "-c", "/usr/sbin/cron && lighttpd -D -f /etc/lighttpd/lighttpd.conf"]