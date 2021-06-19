FROM arm64v8/debian:unstable-slim

COPY common/apt/00-mobian-priority /etc/apt/preferences.d/
COPY common/apt/mobian.list /etc/apt/sources.list.d/
COPY common/apt/mobian.gpg /etc/apt/trusted.gpg.d/
COPY common/sudo/mobian /etc/sudoers.d/

ARG DEBIAN_FRONTEND=noninteractive
ARG UNAME=mobian
ARG UID=1000
ARG GID=1000

# Add Salsa-CI scripts
ADD https://salsa.debian.org/salsa-ci-team/pipeline/-/raw/master/images/scripts/filter-output /usr/local/bin/filter-output
ADD https://salsa.debian.org/salsa-ci-team/pipeline/-/raw/master/images/scripts/install-build-deps.sh /usr/local/bin/install-build-deps.sh
ADD https://salsa.debian.org/salsa-ci-team/pipeline/-/raw/master/images/scripts/add_extra_repository.sh /usr/local/bin/add_extra_repository.sh
ADD https://salsa.debian.org/salsa-ci-team/pipeline/-/raw/master/images/files/base/initialize /usr/local/bin/initialize
RUN chmod 755 /usr/local/bin/filter-output \
              /usr/local/bin/install-build-deps.sh \
              /usr/local/bin/add_extra_repository.sh \
              /usr/local/bin/initialize

RUN chmod 644 /etc/apt/preferences.d/00-mobian-priority \
              /etc/apt/sources.list.d/mobian.list \
              /etc/apt/trusted.gpg.d/mobian.gpg \
              /etc/sudoers.d/mobian

RUN apt-get update && apt-get dist-upgrade -y && \
    apt-get install -y sudo git build-essential vim eatmydata curl librust-glib-sys-dev libcairo2-dev libavutil-dev libavformat-dev libavfilter-dev libavdevice-dev libatk1.0-dev libpango1.0-dev libgstreamer1.0-dev librust-gdk-sys-dev libclang-dev libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad gstreamer1.0-plugins-ugly gstreamer1.0-libav libgstrtspserver-1.0-dev libgstreamer-plugins-bad1.0-dev


RUN groupadd -g $GID -o $UNAME
RUN useradd -m -u $UID -g $GID -o -s /bin/bash $UNAME
RUN adduser $UNAME sudo
RUN echo "mobian:pass" | chpasswd

USER mobian
ENV HOME /home/mobian
ENV USER mobian 
ENV SHELL /bin/bash
WORKDIR /home/mobian

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
