FROM mcr.microsoft.com/devcontainers/base:debian-11

# Run as non-root.
USER vscode

# Install Rust & Go toolchains (latest & stable).
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile complete && \
    curl -sSf https://raw.githubusercontent.com/owenthereal/goup/master/install.sh | sh -s -- '--skip-prompt'

# Source the envs into bashrc, and establish the gobin path.
ENV PATH $PATH:/home/vscode/.go/current/bin:/home/vscode/.cargo/bin
ENV GOBIN /home/vscode/.go/bin

# Install protoscope.
RUN go install github.com/protocolbuffers/protoscope/cmd/protoscope...@latest

# Install protoc.
COPY --chown=vscode:vscode ./scripts/install_protoc.sh /home/vscode/install_protoc.sh
RUN chmod +x /home/vscode/install_protoc.sh && sudo /home/vscode/install_protoc.sh
