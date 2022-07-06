SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "${SCRIPT_DIR}/../src"
cargo build -r
cd target/release
chmod +x dbh
sudo mv dbh /usr/local/bin
