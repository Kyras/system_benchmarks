#!/bin/bash
trap clean EXIT

function clean() {
  iptables -D INPUT -p tcp --dport 5201 -j NFQUEUE --queue-num 0
}

nohup iperf3 -s >/dev/null 2>&1 &
sleep 1
IP_PID=$!

echo "Running control (without debugger)"
iperf3 -c 127.0.0.1

nohup ./tokio_raw_socket &
PID=$!
sleep 1
echo "Running benchmark for raw socket (pid=$PID)"
iperf3 -c 127.0.0.1
kill $PID

nohup ./tokio_nfqueue &
PID=$!
sleep 1
echo "Running benchmark for nfqueue (pid=$PID)"
iptables -A INPUT -p tcp --dport 5201 -j NFQUEUE --queue-num 0
iperf3 -c 127.0.0.1
kill $PID

kill $IP_PID
