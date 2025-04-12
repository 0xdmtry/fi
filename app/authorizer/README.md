
### SeaORM Migration

#### Install SeaORM CLI

```bash
cargo install sea-orm-cli
```

#### Generate Migration
```bash
sea-orm-cli migrate generate create_user_and_passcode
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
