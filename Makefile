start:
	cargo watch -x 'test-bpf -- --show-output'

key:
	solana-keygen new --outfile netw/program.json --force

update:
	cargo build-bpf -- --ignore-rust-version
	solana program deploy target/deploy/nft.so --program-id netw/program.json --keypair netw/admin.json --upgrade-authority netw/admin.json

buffer:
	solana program deploy --buffer /home/vitalii/.config/solana/id.json target/deploy/nft.so --upgrade-authority netw/admin.json

close:
	solana program close --buffers /home/vitalii/.config/solana/id.json --authority /home/vitalii/apps/m_bottt/rust/netw/admin.json