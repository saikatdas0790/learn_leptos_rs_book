# Install cargo binstall
http get https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz | save cargo-binstall.tgz;
tar -xzf cargo-binstall.tgz;
mkdir ~/.local/bin;
mv ./cargo-binstall ~/.local/bin/cargo-binstall;
chmod +x ~/.local/bin/cargo-binstall;
rm cargo-binstall.tgz;

# Install trunk using cargo binstall
cargo binstall trunk --no-confirm;