# iBOOD RSS feed generator (unofficial)

If you don't care about the code but just want to use the RSS feed, you can find it here: https://iboodrss.jeroenpelgrims.be

## What is this?

This repo contains code to build an executable that can be run to generate an RSS feed for the daily iBOOD products on sale.

In addition a Dockerfile is provided that runs the script daily by using cron. The resulting RSS feed files are then served by nginx.

### Building the executable

Run `cargo build --release`.

##### Building the docker image

Run `docker build -t iboodrss .`

##### Using the prebuild image from docker hub

You can find the prebuild image [here](https://hub.docker.com/r/jeroenpelgrims/iboodrss).

## TODO

At some point I should try to make the image smaller by using Alpine as a base image.  
I tried before, but I had issues with the SSL certificates.  
I eventually had the same issue in Debian, but I was able to fix it by installing `ca-certificates`.
The solution is probably similar in Alpine. Check this to solve this in Alpine: https://stackoverflow.com/questions/67231714/how-to-add-trusted-root-ca-to-docker-alpine

Some other commands I used when trying to build on Alpine:

- sudo apt install musl musl-tools
- rustup target add x86_64-unknown-linux-musl
- cargo build --target=x86_64-unknown-linux-musl
