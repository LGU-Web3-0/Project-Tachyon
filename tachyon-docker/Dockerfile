FROM archlinux:latest

RUN echo 'Server = https://opentuna.cn/archlinux/$repo/os/$arch' > /etc/pacman.d/mirrorlist && \
    cat /etc/pacman.d/mirrorlist && \
    pacman -Syy --noconfirm && pacman -Syu --noconfirm lld clang rustup gulp yarn npm nodejs openssl base-devel cmake nettle

RUN mkdir -p $HOME/.cargo/ && echo $'\
[source.crates-io] \n\
replace-with = "tuna" \n\
[source.tuna] \n\
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git" \n\
'>$HOME/.cargo/config

RUN echo $'\
registry = "https://registry.npm.taobao.org" \n\
'>$HOME/.npmrc

RUN rustup toolchain install nightly

ENV PATH=$PATH:$HOME/.cargo/bin

RUN cargo install cargo-watch sea-orm-cli

WORKDIR /tachyon

ENTRYPOINT cargo watch \
    -x "run --all-features --bin migration -- up" \
    -x "run --all-features --bin tachyon-core -- -c /tachyon/tachyon-core/misc/config.toml"


