sshPath="root@pkjy.xyz" 
dateTime=`date +%Y%m%d%H%M%S`

cargo build --release --target x86_64-unknown-linux-musl &&
scp target/x86_64-unknown-linux-musl/release/hiss root@pkjy.xyz:/opt/hiss/hiss_prerelease &&
ssh ${sshPath} "pm2 stop hiss" &&
ssh ${sshPath} "mv /opt/hiss/hiss /opt/archive/hiss.${dateTime} || true" &&
ssh ${sshPath} "mv /opt/hiss/hiss_prerelease /opt/hiss/hiss" &&
ssh ${sshPath} "pm2 restart hiss"
