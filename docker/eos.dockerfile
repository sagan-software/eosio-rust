FROM ubuntu:18.04

# Arguments
ARG fork=EOSIO
ARG branch=v2.0.3

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

# Environment variables
ENV EOSIO_ROOT /root/eosio/2.0
ENV BOOST_ROOT "${EOSIO_ROOT}/src/boost_1_71_0"
ENV PATH "${EOSIO_ROOT}/bin:${PATH}"

ENTRYPOINT [ "nodeos" ]
