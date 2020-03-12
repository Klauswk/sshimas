cargo build --release
mkdir -p deb/bin/
mkdir -p deb/bin/bin/

cp target/release/sshimas deb/bin/
cp bin/plink deb/bin/bin/

dpkg-deb --build deb .
