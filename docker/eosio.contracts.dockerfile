FROM sagansoftware/eosio.contracts:1.8.3 AS old_contracts

FROM sagansoftware/eosio.cdt:1.7.0

# Arguments
ARG fork=EOSIO
ARG branch=v1.9.1

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

ENV EOSIO_CONTRACTS_DIRECTORY /eosio.contracts/build/contracts
ENV EOSIO_OLD_CONTRACTS_DIRECTORY /eosio.contracts/build/old_contracts

COPY --from=old_contracts ${EOSIO_CONTRACTS_DIRECTORY} ${EOSIO_OLD_CONTRACTS_DIRECTORY}
