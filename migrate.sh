sshPath="root@pkjy.xyz" 
dateTime=`date +%Y%m%d%H%M%S`

cd migration && 
cargo build --release --target x86_64-unknown-linux-musl &&
scp target/x86_64-unknown-linux-musl/release/migration root@pkjy.xyz:/opt/hiss/migration &&
ssh ${sshPath} "cd /opt/hiss && ./migration" 
