defmodule CnftElixirTest do
  use ExUnit.Case
  doctest CnftElixir

  test "Create an account" do
    {:ok, keypair} = Solana.keypair_read_from_file("/home/piyush/.config/solana/id.json")
    new_keypair = Solana.new_keypair()

    client =
      Solana.new_rpc_client("https://api.devnet.solana.com")

    size = 0
    {:ok, lamports} = Solana.get_minimum_balance_for_rent_exemption(client, size)

    instruction =
      Solana.create_account_instruction(
        Solana.keypair_to_public_key(keypair),
        Solana.keypair_to_public_key(new_keypair),
        lamports,
        size,
        Solana.keypair_to_public_key(keypair)
      )

    {:ok, latest_block_hash} = Solana.get_latest_blockhash(client)

    transaction =
      Solana.new_signed_transaction_with_payer(
        [instruction],
        Solana.keypair_to_public_key(keypair),
        [keypair, new_keypair],
        latest_block_hash
      )

    {:ok, signature} = Solana.send_and_confirm_transaction(client, transaction)
    IO.puts(signature)
  end
end
