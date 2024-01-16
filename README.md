# iBOOD RSS feed generator (unofficial)

This repo contains code to build an executable that can be run to generate an RSS feed for the daily iBOOD products on sale.

In addition a Dockerfile is provided that runs the script daily by using cron. The resulting RSS feed files are then served by nginx.

## Building the executable

Run `cargo build --release`.

## Building the docker image

Run `docker build -t iboodrss .`

## Using the prebuild image from docker hub

You can find the prebuild image [here](https://hub.docker.com/r/jeroenpelgrims/iboodrss).

## TODO at some point to make the image smaller

Build & run on alpine:
Tried this before, but got stuck with an error about invalid ssl certificates.
Eventually had the same issue in debian, where the fix was to install `ca-certificates`.
Check this for a solution for this in alpine: https://stackoverflow.com/questions/67231714/how-to-add-trusted-root-ca-to-docker-alpine

Other commands I used to build on alpine:

- sudo apt install musl musl-tools
- rustup target add x86_64-unknown-linux-musl
- cargo build --target=x86_64-unknown-linux-musl
