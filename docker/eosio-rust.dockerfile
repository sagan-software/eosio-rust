FROM sagansoftware/eosio.contracts:1.9.0

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install binaryen
RUN cd ~ && \
    git clone --recursive https://github.com/WebAssembly/binaryen.git && \
    cd binaryen && \
    cmake . && make && make install

# Install wabt
RUN cd ~ && \
    git clone --recursive https://github.com/WebAssembly/wabt && \
    cd wabt && \
    make && \
    make install
