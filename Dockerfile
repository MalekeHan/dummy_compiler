# Use a Fedora base image
FROM fedora:latest

# Install necessary packages
RUN dnf update -y && \
    dnf install -y rust gcc llvm zlib-devel libffi-devel libxml2-devel clang libcxx-devel && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    dnf clean all

# Set the environment variable to include Rust binaries
ENV PATH="/root/.cargo/bin:${PATH}"

# Your application setup steps here

CMD ["/bin/bash"]

# Use Fedora 39 as the base image
FROM fedora:39

# Install system updates and required packages
RUN dnf update -y && \
    dnf install -y \
    gcc \
    gcc-c++ \
    llvm16 \
    llvm16-devel \
    libedit-devel \
    llvm16-libs \
    llvm16-static \
    ncurses-c++-libs \
    ncurses-devel \
    curl \
    libffi-devel \
    zlib-devel \
    clang \
    gdb && \
    dnf clean all

# install rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Rust binaries in PATH for future RUN commands and when the container starts
ENV PATH="/root/.cargo/bin:${PATH}"

# set the working directory in the container
WORKDIR /project

# default command to run when the container starts
CMD ["bash"]