use anyhow::{Context, Result};
use clap::Parser;
use tabled::Tabled;

use crate::util::{fmt, term};

#[derive(Parser)]
pub enum SubCommand {
	List,
	Get {
		namespace_id: String,
	},
	Create {
		#[clap(long)]
		display_name: String,
		#[clap(long)]
		version: String,
		#[clap(long)]
		name_id: String,
	},
	SetVersion,
	Dashboard,
	Visit,
}

impl SubCommand {
	pub async fn execute(&self, ctx: &rivetctl::Ctx) -> Result<()> {
		match self {
			SubCommand::List => {
				let game_res = ctx
					.client()
					.get_game_by_id()
					.game_id(&ctx.game_id)
					.send()
					.await
					.context("client.get_game_by_id")?;
				let game = game_res.game.context("game_res.game")?;
				let game_versions = game.versions().context("game.versions")?;

				#[derive(Tabled)]
				struct Namespace {
					#[tabled(rename = "Name ID")]
					name_id: String,
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Version")]
					version: String,
					#[tabled(rename = "Created")]
					created: String,
					#[tabled(rename = "ID")]
					namespace_id: String,
				}

				let mut ns = game
					.namespaces()
					.context("game.namespaces")?
					.iter()
					.map(|ns| {
						let version_id = ns.version_id().context("ns.version_id")?.to_string();
						let version_name = game_versions
							.iter()
							.find(|x| x.version_id().map_or(false, |id| id == version_id))
							.and_then(|x| x.display_name())
							.map_or_else(|| version_id.to_string(), |x| x.to_string());

						Ok(Namespace {
							display_name: ns.display_name().context("ns.display_name")?.to_string(),
							name_id: ns.name_id().context("ns.name_id")?.to_string(),
							namespace_id: ns.namespace_id().context("ns.namespace_id")?.to_string(),
							version: version_name,
							created: fmt::date(ns.create_ts().context("ns.create_ts")?),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				ns.reverse();
				term::table(&ns);

				Ok(())
			}
			SubCommand::Get { namespace_id } => {
				let ns_res = ctx
					.client()
					.get_game_namespace_by_id()
					.game_id(&ctx.game_id)
					.namespace_id(namespace_id)
					.send()
					.await
					.context("client.get_game_version_by_id")?;
				let ns = ns_res.namespace().context("ns_res.namespace")?;
				println!("{ns:#?}");

				Ok(())
			}
			SubCommand::Create {
				display_name,
				version,
				name_id,
			} => {
				ctx.client()
					.create_game_namespace()
					.display_name(display_name)
					.name_id(name_id)
					.version_id(version)
					.send()
					.await
					.context("client.create_game_namespace")?;

				Ok(())
			}
			SubCommand::SetVersion => todo!(),
			SubCommand::Dashboard => todo!(),
			SubCommand::Visit => todo!(),
		}
	}
}
