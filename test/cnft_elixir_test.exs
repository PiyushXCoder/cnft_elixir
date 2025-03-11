defmodule CnftElixirTest do
  use ExUnit.Case
  doctest CnftElixir

  test "Create an account and configure merkle tree" do
    {:ok, keypair} = Solana.keypair_read_from_file("/home/piyush/.config/solana/id.json")
    merkle_tree = Solana.new_keypair()

    {:ok, merkle_tree_rogram_id} =
      Solana.pubkey_from_string("cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK")

    client =
      Solana.new_rpc_client("https://api.devnet.solana.com")

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

    {:ok, latest_block_hash} = Solana.get_latest_blockhash(client)

    transaction =
      Solana.new_signed_transaction_with_payer(
        [create_account_instruction, create_tree_config_instruction],
        Solana.keypair_to_public_key(keypair),
        [keypair, merkle_tree],
        latest_block_hash
      )

    {:ok, signature} = Solana.send_and_confirm_transaction(client, transaction)
    IO.puts("transaction signature = " <> signature)
  end
end
