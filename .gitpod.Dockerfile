FROM gitpod/workspace-full

# Install custom tools, runtimes, etc.
# For example "bastet", a command-line tetris clone:
# RUN brew install bastet
#
# More information: https://www.gitpod.io/docs/config-docker/
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > sh.rustup.rs 
RUN chmod +x sh.rustup.rs 
RUN ./sh.rustup.rs -y
RUN sudo apt install source -y
RUN source $HOME/.cargo/env
