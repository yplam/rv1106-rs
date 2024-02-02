FROM ubuntu:22.04

ARG TIMEZONE=Asia/Shanghai
ARG UNAME=rv1106
ARG UID=1000
ARG GID=1000
ENV DEBIAN_FRONTEND=noninteractive

ADD ./arm-rockchip830-linux-uclibcgnueabihf.tar.xz /opt/
ENV PATH="$PATH:/opt/arm-rockchip830-linux-uclibcgnueabihf/bin"
ADD ./rust_rv1106.tar.xz /opt/

RUN \
    sed -e "s/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g" \
        -e "s/security.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g" -i /etc/apt/sources.list \
        && ln -snf /usr/share/zoneinfo/${TIMEZONE} /etc/localtime && echo ${TIMEZONE} > /etc/timezone \
        && apt-get update && apt-get install -y git ssh make gcc gcc-multilib \
          g++-multilib module-assistant expect g++ gawk texinfo libssl-dev bison \
          flex fakeroot cmake unzip gperf autoconf device-tree-compiler \
          libncurses5-dev pkg-config bc python-is-python3 passwd openssl \
          openssh-server openssh-client vim file cpio rsync iproute2 repo time \
          python3-pip libprotobuf-dev zlib1g zlib1g-dev libsm6 \
          libgl1 libglib2.0-0 android-tools-adb libclang-dev curl zsh

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=nightly

RUN set -eux; \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        amd64) rustArch='x86_64-unknown-linux-gnu'; rustupSha256='0b2f6c8f85a3d02fde2efc0ced4657869d73fccfce59defb4e8d29233116e6db' ;; \
        armhf) rustArch='armv7-unknown-linux-gnueabihf'; rustupSha256='f21c44b01678c645d8fbba1e55e4180a01ac5af2d38bcbd14aa665e0d96ed69a' ;; \
        arm64) rustArch='aarch64-unknown-linux-gnu'; rustupSha256='673e336c81c65e6b16dcdede33f4cc9ed0f08bde1dbe7a935f113605292dc800' ;; \
        i386) rustArch='i686-unknown-linux-gnu'; rustupSha256='e7b0f47557c1afcd86939b118cbcf7fb95a5d1d917bdd355157b63ca00fc4333' ;; \
        ppc64el) rustArch='powerpc64le-unknown-linux-gnu'; rustupSha256='1032934fb154ad2d365e02dcf770c6ecfaec6ab2987204c618c21ba841c97b44' ;; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    url="https://static.rust-lang.org/rustup/archive/1.26.0/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version;

RUN rustup toolchain link rv1106 /opt/rust_rv1106
RUN rustup component add rustfmt

RUN mkdir /src
RUN groupadd -g $GID -o $UNAME \
    && useradd -m -u $UID -g $GID -o -s /bin/zsh $UNAME

USER $UNAME
RUN wget https://github.com/robbyrussell/oh-my-zsh/raw/master/tools/install.sh -O - | zsh || true

WORKDIR /src
ENTRYPOINT ["tail", "-f", "/dev/null"]