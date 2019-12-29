FROM sagansoftware/eosio.cdt:1.7.0-rc1

# Arguments
ARG fork=EOSIO
ARG branch=v1.9.0-rc3

# Build
RUN git clone \
    --recursive \
    --branch $branch \
    --single-branch \
    https://github.com/$fork/eosio.contracts.git \
    /eosio.contracts
WORKDIR /eosio.contracts
RUN ./build.sh -y -t -e ${EOSIO_ROOT} -c ${EOSIO_CDT_ROOT}
RUN ./build/tests/unit_test --show_progress=yes
WORKDIR /
