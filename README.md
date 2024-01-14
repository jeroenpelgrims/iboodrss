# iBOOD RSS feed generator (unofficial)

This repo contains code to build an executable that can be run to generate an RSS feed for the daily iBOOD products on sale.

In addition a Dockerfile is provided that runs the script daily by using cron. The resulting RSS feed files are then served by nginx.

## Building the executable

Run `cargo build --release`.

## Building the docker image

Run `docker build -t ibood-rss .`
