FROM ubuntu:18.04 AS builder

# Arguments
ARG fork=EOSIO
ARG branch=v2.0.0

# Build
RUN apt-get update && \
    apt-get install -y git sudo
RUN git clone \
    --recursive \
    --branch $branch \
    --single-branch \
    https://github.com/$fork/eos.git \
    /eos
WORKDIR /eos
RUN git submodule update --init --recursive
RUN yes | ./scripts/eosio_build.sh
RUN ./scripts/eosio_install.sh


FROM ubuntu:18.04

COPY --from=builder /root/eosio /root/eosio

RUN apt-get update && \
    apt-get install -y openssl && \
    apt-get autoremove -y && \
    apt-get clean

# Environment variables
ENV EOSIO_ROOT /root/eosio/2.0
ENV BOOST_ROOT "${EOSIO_ROOT}/src/boost_1_71_0"
ENV PATH "${EOSIO_ROOT}/bin:${PATH}"

ENTRYPOINT [ "nodeos" ]