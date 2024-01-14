FROM alpine:3

VOLUME /data
VOLUME /log
RUN apk add curl nginx && rm -rf /var/cache/apk/*
RUN apk add --update apk-cron && rm -rf /var/cache/apk/*
COPY crontab.txt /var/spool/cron/crontabs/root
COPY script.sh /script.sh
RUN chmod +x /script.sh

CMD ["sh", "-c", "/usr/sbin/crond && nginx -g 'daemon off;'"]