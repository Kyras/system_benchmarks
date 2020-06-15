FROM kyras/tezedge_base:latest as builder
WORKDIR /home/appuser/
COPY . .
RUN cargo install --bins --path ./ --root ./

FROM ubuntu:latest
WORKDIR /home/appuser
RUN apt-get update
RUN apt-get install iperf3 -y
COPY --from=builder /home/appuser/bin/* ./