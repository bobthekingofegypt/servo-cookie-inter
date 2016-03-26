# -*- mode: ruby -*-
# vi: set ft=ruby :
#
# To boot this Vagrant machine, run: vagrant up --provider virtualbox
#
Vagrant.configure(2) do |config|
  config.vm.box = 'ubuntu/trusty64'

  config.vm.provision 'shell', inline: <<-SHELL
    set -e
    apt-get update
    DEBIAN_FRONTEND=noninteractive apt-get install -y git libssl-dev
    curl -sSf https://static.rust-lang.org/rustup.sh > /tmp/rustup.sh && \
        sh /tmp/rustup.sh --yes --channel=stable
    cd /vagrant && git clone https://github.com/google/pywebsocket.git
SHELL
end
