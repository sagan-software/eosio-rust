FROM sagansoftware/eos:2.0.3
# Arguments
ARG fork=EOSIO
ARG branch=v1.7.0

# Build
RUN git clone \
    --recursive \
    --branch $branch \
    --single-branch \
    https://github.com/$fork/eosio.cdt \
    /eosio.cdt
WORKDIR /eosio.cdt
RUN echo 1 | ./scripts/eosiocdt_build.sh
RUN ./scripts/eosiocdt_install.sh
WORKDIR /

# Environment variables
ENV EOSIO_CDT_ROOT /root/opt/eosio.cdt
ENV PATH "${EOSIO_CDT_ROOT}/bin:${PATH}"
