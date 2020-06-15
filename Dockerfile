FROM kyras/tezedge_base:latest as builder
WORKDIR /home/appuser/
COPY . .
RUN cargo install --bins --path ./ --root ./

FROM ubuntu:latest
WORKDIR /home/appuser
RUN apt-get update
RUN apt-get install iperf3 -y
RUN apt-get install iptables -y
COPY ./benchmarks.sh ./
COPY --from=builder /home/appuser/bin ./
