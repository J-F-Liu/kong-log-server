## Kong Log Server

Receive and collect logs from Kong's HTTP Log plugin.

## Features

- [x] Receive logs from Kong's HTTP Log plugin
- [x] Save to logfile through Redirection
- [ ] Save logs to MongoDB
- [ ] Query logs from MongoDB

### Build and Deploy
```
docker start -i centos-rust
cd /Rust/kong-log-server
source $HOME/.cargo/env
cargo build --release
exit
scp ./target/release/kong-log-server junfeng@10.10.100.17:/home/junfeng
nohup ./kong-log-server > kong-2019-06-20.log &
sudo /usr/local/bin/kong reload
```
