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

  # mpl_bubblegum
  def cnft_create_tree_config_instruction(
        _tree_config,
        _merkle_tree,
        _payer,
        _tree_creator,
        _max_depth,
        _max_buffer_size
      ),
      do: error()

  def cnft_mint_v1_instruction(
        _merkle_tree,
        _payer,
        _leaf_delegate,
        _leaf_owner,
        _tree_config,
        _tree_creator_or_delegate,
        _metadata
      ),
      do: error()

  def cnft_transfer_instruction(
        _tree_config,
        _leaf_owner,
        _leaf_delegate,
        _new_leaf_owner,
        _merkle_tree
      ),
      do: error()

  defmodule Collection do
    defstruct verified: false, key: nil
  end

  defmodule Uses do
    defstruct use_method: "", remaining: 0, total: 0
  end

  defmodule Creator do
    defstruct address: nil, verified: false, share: 0
  end

  defmodule MetadataArgs do
    defstruct name: "",
              symbol: "",
              uri: "",
              seller_fee_basis_points: 0,
              primary_sale_happened: false,
              is_mutable: false,
              edition_nonce: nil,
              token_standard: nil,
              collection: nil,
              uses: nil,
              token_program_version: "",
              creators: nil
  end
end
