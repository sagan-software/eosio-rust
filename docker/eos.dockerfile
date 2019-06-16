FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y git sudo

# Build eos
ARG eos_fork=EOSIO
ARG eos_branch=v1.6.3
RUN git clone \
    --recursive \
    --branch $eos_branch \
    --single-branch \
    https://github.com/$eos_fork/eos.git \
    /eos
WORKDIR /eos
RUN git submodule update --init --recursive
RUN echo 1 | ./eosio_build.sh
RUN ./eosio_install.sh

WORKDIR /

# Build eosio.cdt
ARG cdt_fork=EOSIO
ARG cdt_branch=v1.5.0
RUN git clone \
    --recursive \
    --branch $cdt_branch \
    --single-branch \
    https://github.com/$cdt_fork/eosio.cdt \
    /eosio.cdt
WORKDIR /eosio.cdt
RUN echo 1 | ./build.sh
RUN ./install.sh

WORKDIR /

# RUN apt-get install -y wget
# RUN wget https://github.com/eosio/eosio.cdt/releases/download/v1.5.0/eosio.cdt_1.5.0-1_amd64.deb
# RUN apt install -y ./eosio.cdt_1.5.0-1_amd64.deb

# RUN apt-get install -y cmake
RUN PATH=/usr/local/eosio/bin:/usr/local/eosio.cdt/bin:$PATH
ENV EOSIO_ROOT=/usr/local/eosio
ENV EOSIO_CDT_ROOT=/usr/local/eosio.cdt
ENV LD_LIBRARY_PATH=/usr/local/lib