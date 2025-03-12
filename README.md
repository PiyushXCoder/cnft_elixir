# CnftElixir

An attempt to make `nif` for `Solana` and `mpl_bubblegum`.

> Since it is still a proof of concept and experimental. It is not recommended to be used in production. 

## How to use?

The library is made in a way to look and feel quite similar to `@solana/web3.js`

### Steps

- Import or generate all the Keypair or public key you need
``` elixir
    {:ok, keypair} = Solana.keypair_read_from_file("/home/piyush/.config/solana/id.json")
    merkle_tree = Solana.new_keypair()

    {:ok, merkle_tree_rogram_id} =
      Solana.pubkey_from_string("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK")
```
- Connect to the RPC 
``` elixir
    client =
      Solana.new_rpc_client("https://api.devnet.solana.com")
```
- Create the instructions
``` elixir
    size = 4024
    {:ok, lamports} = Solana.get_minimum_balance_for_rent_exemption(client, size)

    create_account_instruction =
      Solana.create_account_instruction(
        Solana.keypair_to_public_key(keypair),
        Solana.keypair_to_public_key(merkle_tree),
        lamports,
        size,
        merkle_tree_rogram_id
      )

    tree_config = Solana.tree_config_find_pda(Solana.keypair_to_public_key(merkle_tree))

    create_tree_config_instruction =
      Solana.cnft_create_tree_config_instruction(
        tree_config,
        Solana.keypair_to_public_key(merkle_tree),
        Solana.keypair_to_public_key(keypair),
        Solana.keypair_to_public_key(keypair),
        6,
        16
      )
```
- Get recent block hash
``` elixir
    {:ok, latest_block_hash} = Solana.get_latest_blockhash(client)
```
- Create a transaction. Note that we add all instructions in the transaction as an array
``` elixir
    transaction =
      Solana.new_signed_transaction_with_payer(
        [create_account_instruction, create_tree_config_instruction],
        Solana.keypair_to_public_key(keypair),
        [keypair, merkle_tree],
        latest_block_hash
      )
```
- Send and confirm the transaction
``` elixir 
    {:ok, signature} = Solana.send_and_confirm_transaction(client, transaction)
```

### Putting all Together
Check the test script to see it all together. [cnft_elixir_test.exs](test/cnft_elixir_test.exs)

## Contribution 
The code is open for the contributions. Feel free to Connect me on x at [@piyushxcoder](https://x.com/PiyushXCoder) for questions.

Try your best to handle errors as much as possible because the panic to `nif` can panic `erlang`!

I might make a comprehensive guide on migrating  rust functions and structs, but for the time being you may refer [native/solana/src/pubkey.rs](native/solana/src/pubkey.rs) and [lib/cnft_elixir.ex](lib/cnft_elixir.ex) for getting started. 

The project is based on [rustler](https://github.com/rusterlium/rustler), so it might be a good idea to check their repository too.
