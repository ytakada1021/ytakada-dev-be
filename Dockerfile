FROM amazonlinux:2

SHELL ["/bin/bash", "-oeux", "pipefail", "-c"]

WORKDIR /tmp

# Install utilities etc
RUN touch ~/.bashrc ~/.bash_profile ~/.profile && \
    yum update -y && \
    yum install -y systemd tar zip unzip xz gzip openssl-devel && \
    yum groupinstall -y "Development Tools"

# Install Node.js
ENV NODE_VERSION 16.13.0
ENV NVM_DIR /usr/local/nvm
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH
RUN mkdir -p $NVM_DIR && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash && \
    . $NVM_DIR/nvm.sh && \
    nvm install $NODE_VERSION

# Install Rust tools
ENV RUST_VERSION 1.64.0
ENV PATH ~/.cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $RUST_VERSION

# Install AWS CLI
RUN curl -o awscli.zip -L https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip && \
    unzip awscli.zip && \
    ./aws/install && \
    rm awscli.zip

# Install jq
RUN mkdir -p /usr/local/jq && \
    curl -L -o /usr/local/jq/jq-linux64 https://github.com/stedolan/jq/releases/download/jq-1.6/jq-linux64 && \
    chmod u+x /usr/local/jq/jq-linux64 && \
    ln -s /usr/local/jq/jq-linux64 /usr/local/bin/jq

# Install cdk
RUN npm install -g aws-cdk aws-cdk-local

# # Install sqlx cli
# RUN cargo install sqlx-cli --no-default-features --features native-tls,mysql

WORKDIR /work

CMD ["/sbin/init"]
