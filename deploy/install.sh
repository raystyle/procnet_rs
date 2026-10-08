#!/bin/sh
# procnet + Vector 安装(linux;root 跑;参数:procnet 二进制 vector tarball;env:VECTOR_CH_PASSWORD)
set -e
install -m755 "$1" /usr/local/bin/procnet
tar xzf "$2" -C /tmp
V=$(ls -d /tmp/vector-* | head -1)
install -m755 "$V/bin/vector" /usr/local/bin/vector 2>/dev/null || install -m755 "$V/vector-x86_64-unknown-linux-musl/bin/vector" /usr/local/bin/vector
mkdir -p /etc/vector
sed "s|__VECTOR_CH_PASSWORD__|$VECTOR_CH_PASSWORD|" "$(dirname "$0")/vector.toml" > /etc/vector/vector.toml
cp "$(dirname "$0")/vector.service" /etc/systemd/system/vector.service
systemctl daemon-reload
systemctl enable --now vector
