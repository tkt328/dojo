use anyhow::Result;
use clap::Args;
use scarb::core::Config;
use sozo_ops::execute;
use starknet::core::types::FieldElement;
use tracing::trace;

use super::options::account::AccountOptions;
use super::options::starknet::StarknetOptions;
use super::options::transaction::TransactionOptions;
use super::options::world::WorldOptions;
use crate::utils;

#[derive(Debug, Args)]
#[command(about = "Execute a system with the given calldata.")]
pub struct ExecuteArgs {
    #[arg(help = "The address of the contract to be executed. Or fully qualified contract name \
                  (ex: dojo_example::actions::actions")]
    pub contract: String,

    #[arg(help = "The name of the entrypoint to be executed.")]
    pub entrypoint: String,

    #[arg(short, long)]
    #[arg(value_delimiter = ',')]
    #[arg(help = "The calldata to be passed to the system. Comma separated values e.g., \
                  0x12345,0x69420.")]
    pub calldata: Vec<FieldElement>,

    #[command(flatten)]
    pub starknet: StarknetOptions,

    #[command(flatten)]
    pub account: AccountOptions,

    #[command(flatten)]
    pub world: WorldOptions,

    #[command(flatten)]
    pub transaction: TransactionOptions,
}

impl ExecuteArgs {
    pub fn run(self, config: &Config) -> Result<()> {
        trace!(args = ?self);
        let env_metadata = utils::load_metadata_from_config(config)?;

        config.tokio_handle().block_on(async {
            let world = utils::world_from_env_metadata(
                self.world,
                self.account,
                self.starknet,
                &env_metadata,
            )
            .await
            .unwrap();
            let tx_config = self.transaction.into();

            trace!(
                contract=?self.contract,
                entrypoint=self.entrypoint,
                calldata=?self.calldata,
                "Executing Execute command."
            );

            execute::execute(
                &config.ui(),
                self.contract,
                self.entrypoint,
                self.calldata,
                &world,
                &tx_config,
            )
            .await
        })
    }
}
