cargo build --release
cp target/release/sshimas deb/bin/
cp bin/plink deb/bin/bin/

dpkg-deb --build deb .
