
### SeaORM Migration

#### Install SeaORM CLI

```bash
cargo install sea-orm-cli
```

#### Generate Migration
```bash
sea-orm-cli migrate generate create_wallet_encryptio_secret_wallet_share
```

### Test

Test can be run with:

```bash
cargo test
```

Or run only specific file:

```bash
cargo test --test join
```


solana balance 6CxR6QhuJwz1sqxXiuPTHrjzKWrW7m7jEekP5sMxrHmz --url https://api.devnet.solana.com

spl-token accounts --owner 6CxR6QhuJwz1sqxXiuPTHrjzKWrW7m7jEekP5sMxrHmz --url https://api.devnet.solana.com
