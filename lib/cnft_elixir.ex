defmodule CnftElixir do
end

defmodule Solana do
  use Rustler, otp_app: :cnft_elixir, crate: :solana
  defp error, do: :erlang.nif_error(:nif_not_found)

  # Client
  def new_rpc_client(_url), do: error()
  def send_and_confirm_transaction(_client, _transaction), do: error()
  def get_minimum_balance_for_rent_exemption(_client, _size), do: error()
  def get_latest_blockhash(_client), do: error()

  # Pubkey
  def keypair_to_public_key(_keypair), do: error()

  # Keypair
  def new_keypair(), do: error()
  def keypair_read_from_file(_file), do: error()

  # Transaction
  def new_signed_transaction_with_payer(
        _instructions,
        _payer,
        _signing_keypair,
        _latest_blockhash
      ),
      do: error()

  # System Program
  def create_account_instruction(_from_pubkey, _to_pubkey, _lamports, _space, _owner), do: error()
end
