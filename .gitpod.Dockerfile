FROM gitpod/workspace-full-vnc

RUN brew install mold \
  && echo "[target.x86_64-unknown-linux-gnu]" >> ~/.cargo/config.toml \
  && echo "linker = \"clang\"" >> ~/.cargo/config.toml \
  && echo "rustflags = [\"-C\", \"link-arg=-fuse-ld=$(which mold)\"]" >> ~/.cargo/config.toml