use super::{instruction::InstructionWrapper, pubkey::PubkeyWrapper};
use mpl_bubblegum::{accounts::TreeConfig, instructions::CreateTreeConfigBuilder};
use rustler::{NifStruct, ResourceArc};

#[rustler::nif]
fn cnft_create_tree_config_instruction(
    tree_config: ResourceArc<PubkeyWrapper>,
    merkle_tree: ResourceArc<PubkeyWrapper>,
    payer: ResourceArc<PubkeyWrapper>,
    tree_creator: ResourceArc<PubkeyWrapper>,
    max_buffer_size: u32,
    max_depth: u32,
) -> ResourceArc<InstructionWrapper> {
    let instruction = CreateTreeConfigBuilder::new()
        .tree_config(tree_config.pubkey)
        .merkle_tree(merkle_tree.pubkey)
        .payer(payer.pubkey)
        .tree_creator(tree_creator.pubkey)
        .max_buffer_size(max_buffer_size)
        .max_depth(max_depth)
        .instruction();
    ResourceArc::new(InstructionWrapper::from(instruction))
}

#[derive(NifStruct)]
#[module = "MetaDataArgsCollection"]
pub struct Collection {
    pub verified: bool,
    pub key: ResourceArc<PubkeyWrapper>,
}

impl From<Collection> for mpl_bubblegum::types::Collection {
    fn from(value: Collection) -> Self {
        mpl_bubblegum::types::Collection {
            verified: value.verified,
            key: value.key.pubkey,
        }
    }
}

#[derive(NifStruct)]
#[module = "MetaDataArgsUses"]
pub struct Uses {
    pub use_method: String,
    pub remaining: u64,
    pub total: u64,
}

impl From<Uses> for mpl_bubblegum::types::Uses {
    fn from(value: Uses) -> Self {
        let use_method = match value.use_method.as_str() {
            "Burn" => mpl_bubblegum::types::UseMethod::Burn,
            "Multiple" => mpl_bubblegum::types::UseMethod::Multiple,
            "Single" => mpl_bubblegum::types::UseMethod::Single,
            _ => panic!("Invalid use method in uses of metadata!"),
        };
        mpl_bubblegum::types::Uses {
            use_method,
            remaining: value.remaining,
            total: value.total,
        }
    }
}

#[derive(NifStruct)]
#[module = "MetaDataArgsCreator"]
pub struct Creator {
    pub address: ResourceArc<PubkeyWrapper>,
    pub verified: bool,
    pub share: u8,
}

impl From<Creator> for mpl_bubblegum::types::Creator {
    fn from(value: Creator) -> Self {
        mpl_bubblegum::types::Creator {
            address: value.address.pubkey,
            verified: value.verified,
            share: value.share,
        }
    }
}

#[derive(NifStruct)]
#[module = "MetaDataArgs"]
struct MetadataArgs {
    /// The name of the asset
    pub name: String,
    /// The symbol for the asset
    pub symbol: String,
    /// URI pointing to JSON representing the asset
    pub uri: String,
    /// Royalty basis points that goes to creators in secondary sales (0-10000)
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    /// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    /// Since we cannot easily change Metadata, we add the new DataV2 fields here at the end.
    pub token_standard: Option<String>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
    pub token_program_version: String,
    pub creators: Vec<Creator>,
}

impl From<MetadataArgs> for mpl_bubblegum::types::MetadataArgs {
    fn from(value: MetadataArgs) -> Self {
        let token_standard = match value.token_standard {
            Some(ts) => match ts.as_str() {
                "NonFungible" => Some(mpl_bubblegum::types::TokenStandard::NonFungible),
                "FungibleAsset" => Some(mpl_bubblegum::types::TokenStandard::FungibleAsset),
                "Fungible" => Some(mpl_bubblegum::types::TokenStandard::Fungible),
                "NonFungibleEdition" => {
                    Some(mpl_bubblegum::types::TokenStandard::NonFungibleEdition)
                }
                _ => None,
            },
            None => None,
        };

        let token_program_version = match value.token_program_version.as_str() {
            "Original" => mpl_bubblegum::types::TokenProgramVersion::Original,
            "Token2022" => mpl_bubblegum::types::TokenProgramVersion::Token2022,
            _ => panic!("Invalid use method in token_program_version of metadata!"),
        };

        let creators = value
            .creators
            .into_iter()
            .map(|c| c.into())
            .collect::<Vec<mpl_bubblegum::types::Creator>>();

        mpl_bubblegum::types::MetadataArgs {
            name: value.name,
            symbol: value.symbol,
            uri: value.uri,
            seller_fee_basis_points: value.seller_fee_basis_points,
            primary_sale_happened: value.primary_sale_happened,
            is_mutable: value.is_mutable,
            edition_nonce: value.edition_nonce,
            token_standard,
            collection: value.collection.map(|c| c.into()),
            uses: value.uses.map(|u| u.into()),
            token_program_version,
            creators,
        }
    }
}

#[rustler::nif]
fn cnft_mint_v1_instruction(
    merkle_tree: ResourceArc<PubkeyWrapper>,
    payer: ResourceArc<PubkeyWrapper>,
    leaf_delegate: ResourceArc<PubkeyWrapper>,
    leaf_owner: ResourceArc<PubkeyWrapper>,
    tree_config: ResourceArc<PubkeyWrapper>,
    tree_creator_or_delegate: ResourceArc<PubkeyWrapper>,
    metadata: MetadataArgs,
) -> ResourceArc<InstructionWrapper> {
    let instruction = mpl_bubblegum::instructions::MintV1Builder::new()
        .merkle_tree(merkle_tree.pubkey)
        .payer(payer.pubkey)
        .leaf_delegate(leaf_delegate.pubkey)
        .leaf_owner(leaf_owner.pubkey)
        .tree_config(tree_config.pubkey)
        .tree_creator_or_delegate(tree_creator_or_delegate.pubkey)
        .metadata(metadata.into())
        .instruction();
    ResourceArc::new(InstructionWrapper::from(instruction))
}

#[rustler::nif]
fn cnft_transfer_instruction(
    tree_config: ResourceArc<PubkeyWrapper>,
    leaf_owner: (ResourceArc<PubkeyWrapper>, bool),
    leaf_delegate: (ResourceArc<PubkeyWrapper>, bool),
    new_leaf_owner: ResourceArc<PubkeyWrapper>,
    merkle_tree: ResourceArc<PubkeyWrapper>,
) -> ResourceArc<InstructionWrapper> {
    let instruction = mpl_bubblegum::instructions::TransferBuilder::new()
        .tree_config(tree_config.pubkey)
        .leaf_owner(leaf_owner.0.pubkey, leaf_owner.1)
        .leaf_delegate(leaf_delegate.0.pubkey, leaf_delegate.1)
        .new_leaf_owner(new_leaf_owner.pubkey)
        .merkle_tree(merkle_tree.pubkey)
        .instruction();
    // TODO: Transfer Instruction doesn't look complete
    ResourceArc::new(InstructionWrapper::from(instruction))
}

#[rustler::nif]
fn tree_config_find_pda(merkle_tree: ResourceArc<PubkeyWrapper>) -> ResourceArc<PubkeyWrapper> {
    let (tree_config, _) = TreeConfig::find_pda(&merkle_tree.pubkey);
    ResourceArc::new(PubkeyWrapper::from(tree_config))
}
