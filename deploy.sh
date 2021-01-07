TARGET=x86_64-unknown-linux-musl

cargo build --release --target $TARGET
ssh azure 'pm2 stop astra'
scp -r ./target/$TARGET/release/astra azure:/home/noodles/astra/astra
#ssh azure 'pm2 start astra'