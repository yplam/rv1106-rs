FROM ubuntu:22.04

ARG TIMEZONE=Asia/Shanghai
ARG UID=1000
ARG GID=1000
ENV DEBIAN_FRONTEND=noninteractive

RUN \
    sed -e "s/archive.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g" \
        -e "s/security.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g" -i /etc/apt/sources.list

# Set timezone
RUN ln -snf /usr/share/zoneinfo/${TIMEZONE} /etc/localtime && echo ${TIMEZONE} > /etc/timezone \
&& "date"

RUN apt-get update && apt-get install -y git ssh make gcc gcc-multilib \
  g++-multilib module-assistant expect g++ gawk texinfo libssl-dev bison \
  flex fakeroot cmake unzip gperf autoconf device-tree-compiler \
  libncurses5-dev pkg-config bc python-is-python3 passwd openssl \
  openssh-server openssh-client vim file cpio rsync