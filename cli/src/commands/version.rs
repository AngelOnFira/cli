use anyhow::{ensure, Context, Error, Result};
use clap::Parser;
use cli_core::rivet_api::models;
use serde::Serialize;
use serde_json::json;
use tabled::Tabled;
use tokio::process::Command;
use uuid::Uuid;

use crate::{
	commands::{build, site},
	util::{fmt, gen, struct_fmt, term},
};

#[derive(Parser)]
pub enum SubCommand {
	/// List all versions
	List,

	/// Get details about a version
	Get {
		/// The version ID
		version: String,
	},

	/// Pushes the build and site and creates a new version
	#[clap(alias = "push-and-create", alias = "create")]
	Publish(PublishOpts),

	/// Returns the config for a version
	#[clap(alias = "read-config")]
	ValidateConfig {
		/// Override specific properties of the config
		#[clap(long = "override", short)]
		overrides: Vec<String>,

		/// The namespace ID to deploy to
		#[clap(short = 'n', long)]
		namespace: Option<String>,
	},

	/// Show the a version's dashboard
	#[clap(alias("dash"))]
	Dashboard { version: String },
}

impl SubCommand {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
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
				let namespaces = game.namespaces().context("game.namespaces")?;

				#[derive(Tabled)]
				struct Version {
					#[tabled(rename = "Name")]
					display_name: String,
					#[tabled(rename = "Namespaces")]
					namespaces: String,
					#[tabled(rename = "Created")]
					created: String,
					#[tabled(rename = "ID")]
					version_id: String,
				}

				let mut version = game
					.versions()
					.context("game.versions")?
					.iter()
					.map(|version| {
						let ns = namespaces
							.iter()
							.filter(|ns| ns.version_id() == version.version_id())
							.filter_map(|ns| ns.display_name())
							.collect::<Vec<&str>>()
							.join(", ");

						Ok(Version {
							display_name: version
								.display_name()
								.context("version.display_name")?
								.to_string(),
							namespaces: ns,
							created: fmt::date(version.create_ts().context("version.create_ts")?),
							version_id: version
								.version_id()
								.context("version.version_id")?
								.to_string(),
						})
					})
					.collect::<Result<Vec<_>>>()?;
				version.reverse();
				term::table(&version);

				Ok(())
			}
			SubCommand::Get { version } => {
				print_version(ctx, &version).await?;

				Ok(())
			}
			SubCommand::Publish(opts) => opts.execute(ctx).await,
			SubCommand::ValidateConfig {
				overrides,
				namespace,
			} => {
				let overrides = parse_config_override_args(overrides)?;
				let config = read_config(overrides, namespace.as_ref().map(String::as_str)).await?;
				println!("{:#?}", config);

				Ok(())
			}
			SubCommand::Dashboard { version } => {
				// Check the version exists
				ctx.client()
					.get_game_version_by_id()
					.game_id(&ctx.game_id)
					.version_id(version)
					.send()
					.await
					.context("client.get_game_version_by_id")?;

				eprintln!("{}", term::link(dashboard_url(&ctx.game_id, version)));

				Ok(())
			}
		}
	}
}

#[derive(Parser)]
pub struct PublishOpts {
	/// Name of the version to create
	#[clap(long = "name", alias = "display-name")]
	display_name: Option<String>,

	/// Override specific properties of the config
	#[clap(long = "override", short)]
	overrides: Vec<String>,

	/// Namespace ID to deploy to
	#[clap(short = 'n', long)]
	namespace: Option<String>,

	/// Deprecated.
	///
	/// The build tag to upload
	#[clap(long)]
	build_tag: Option<String>,

	/// Deprecated.
	///
	/// The name to assign to the build
	#[clap(long)]
	build_name: Option<String>,

	/// Deprecated.
	///
	/// The path to the site directory to upload
	#[clap(long)]
	site_path: Option<String>,

	/// Deprecated.
	///
	/// The name of the site that will be created
	#[clap(long)]
	site_name: Option<String>,

	#[clap(long, value_parser)]
	format: Option<struct_fmt::Format>,
}

impl PublishOpts {
	pub async fn execute(&self, ctx: &cli_core::Ctx) -> Result<()> {
		// Parse overrides
		let mut overrides = parse_config_override_args(&self.overrides)?;

		build_and_push_compat(
			ctx,
			&mut overrides,
			&self.build_tag,
			&self.build_name,
			&self.site_path,
			&self.site_name,
			&self.format,
		)
		.await?;

		let output = create(
			ctx,
			self.display_name.as_ref().map(String::as_str),
			overrides,
			self.namespace.as_ref().map(String::as_str),
			self.format.as_ref(),
		)
		.await?;
		struct_fmt::print_opt(self.format.as_ref(), &output)?;

		Ok(())
	}
}

/// Prints information about a game version
async fn print_version(ctx: &cli_core::Ctx, version_id: &str) -> Result<()> {
	let version_res = ctx
		.client()
		.get_game_version_by_id()
		.game_id(&ctx.game_id)
		.version_id(version_id)
		.send()
		.await
		.context("client.get_game_version_by_id")?;
	let version = version_res.version().context("version_res.version")?;

	println!("{version:#?}");

	Ok(())
}

/// Parses config parameters passed to override version parameters
pub fn parse_config_override_args(
	overrides: &[String],
) -> Result<Vec<(String, serde_json::Value)>> {
	overrides
		.iter()
		.map(|value| {
			value
				.split_once("=")
				.context("override needs equal")
				.and_then(|(key, value)| {
					let value_json = serde_json::from_str::<serde_json::Value>(value)
						.context("invalid override value json")?;
					Ok((key.to_string(), value_json))
				})
		})
		.collect::<Result<Vec<_>, Error>>()
}

/// Reads the Rivet version configuration file and applies overrides. Uses the
/// namespace to read override files.
///
/// For example, in the namespace `foobar`, Rivet would first read
/// `rivet.version.toml` then override with properties from
/// `rivet.version.foobar.toml`.
pub async fn read_config(
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
) -> Result<models::CloudVersionConfig> {
	// Build base config
	let mut config_builder = config::ConfigBuilder::<config::builder::AsyncState>::default()
		.add_source(config::File::with_name("rivet.version"));

	if let Some(namespace) = namespace {
		config_builder = config_builder.add_source(
			config::File::with_name(&format!("rivet.version.{namespace}")).required(false),
		);
	}

	// Apply overrides
	for (k, v) in overrides {
		#[derive(serde::Serialize)]
		struct Empty {
			root: serde_json::Value,
		}

		// Parse the JSON data to a config Value that we can pass as an
		// override.
		//
		// We have to embed the value in `Empty` because the value can't be at
		// the root of the config.
		let config_value = config::Config::try_from(&Empty { root: v })
			.context("read override value to config value")?
			.get::<config::Value>("root")?;

		// Add the override
		config_builder = config_builder
			.set_override(k, config_value)
			.context("set override")?;
	}

	// Read config
	let config = config_builder
		.build()
		.await
		.context("find version config")?;
	let version = config
		.try_deserialize::<models::CloudVersionConfig>()
		.context("deserialize version config")?;

	Ok(version)
}

/// Builds the Docker image and CDN site if needed.
pub async fn build_config_dependencies(
	ctx: &cli_core::Ctx,
	version: &mut models::CloudVersionConfig,
	format: Option<&struct_fmt::Format>,
) -> Result<()> {
	// TODO: Do this for all possible docker endpoints

	if let Some(matchmaker) = version.matchmaker.as_mut() {
		if let Some(docker) = matchmaker.docker.as_mut() {
			build_image(ctx, docker, format).await?;
		}

		if let Some(game_modes) = matchmaker.game_modes.as_mut() {
			for (_, game_mode) in game_modes.iter_mut() {
				if let Some(docker) = game_mode.docker.as_mut() {
					build_image(ctx, docker, format).await?;
				}
			}
		}
	}

	// Build CDN
	if let Some(cdn) = version.cdn.as_mut() {
		build_site(ctx, cdn, format).await?;
	}

	Ok(())
}

pub async fn build_image(
	ctx: &cli_core::Ctx,
	docker: &mut Box<models::CloudVersionMatchmakerGameModeRuntimeDocker>,
	format: Option<&struct_fmt::Format>,
) -> Result<()> {
	if docker.image_id.is_none() {
		if let Some(dockerfile) = docker.dockerfile.as_ref() {
			// Build image
			let tag = format!("rivet-game:{}", Uuid::new_v4());
			let mut build_cmd = Command::new("docker");
			build_cmd
				.arg("build")
				.arg("--file")
				.arg(dockerfile)
				.arg("--tag")
				.arg(&tag)
				.arg(".");
			let build_status = build_cmd.status().await?;
			ensure!(build_status.success(), "Docker image failed to build");

			// Upload build
			let push_output = build::push(
				ctx,
				&build::BuildPushOpts {
					tag,
					name: Some(gen::display_name_from_date()),
					format: format.cloned(),
				},
			)
			.await?;
			docker.image_id = Some(Uuid::parse_str(&push_output.build_id)?);
		}
	}

	Ok(())
}

pub async fn build_site(
	ctx: &cli_core::Ctx,
	cdn: &mut Box<models::CloudVersionCdnConfig>,
	format: Option<&struct_fmt::Format>,
) -> Result<()> {
	if cdn.site_id.is_none() {
		if let Some(build_output) = cdn.build_output.as_ref() {
			if let Some(build_command) = cdn.build_command.as_ref() {
				// TODO: Check Windows support
				let mut build_cmd = Command::new("/bin/sh");
				build_cmd.arg("-c").arg(build_command);
				let build_status = build_cmd.status().await?;
				ensure!(build_status.success(), "site failed to build");
			}

			// Upload site
			let push_output = site::push(
				ctx,
				&site::SitePushOpts {
					path: build_output.clone(),
					name: Some(gen::display_name_from_date()),
					format: format.cloned(),
				},
			)
			.await?;
			cdn.site_id = Some(push_output.site_id);
		}
	}

	Ok(())
}

pub fn dashboard_url(game_id: &str, version_id: &str) -> String {
	format!(
		"https://hub.rivet.gg/developer/games/{game_id}/versions/{version_id}",
		game_id = game_id,
		version_id = version_id
	)
}

#[derive(Serialize)]
pub struct CreateOutput {
	pub version_id: String,
}

/// Creates a new Rivet version.
pub async fn create(
	ctx: &cli_core::Ctx,
	display_name: Option<&str>,
	overrides: Vec<(String, serde_json::Value)>,
	namespace: Option<&str>,
	format: Option<&struct_fmt::Format>,
) -> Result<CreateOutput> {
	let display_name = display_name.map_or_else(gen::display_name_from_date, |x| x.to_string());
	// Parse config
	let mut rivet_config = read_config(overrides, namespace).await?;
	build_config_dependencies(ctx, &mut rivet_config, format).await?;

	// Create game version
	let version_res =
		cli_core::rivet_api::apis::cloud_games_versions_api::cloud_games_versions_create_game_version(
			&ctx.openapi_config_cloud,
			&ctx.game_id,
			cli_core::rivet_api::models::CloudGamesCreateGameVersionInput {
				display_name: display_name.clone(),
				config: Box::new(rivet_config),
			},
		)
		.await;
	if let Err(err) = version_res.as_ref() {
		println!("Error: {err:?}");
	}
	let version_res = version_res.context("versions_create_game_version")?;
	let version_id = version_res.version_id;

	term::status::success("Published", &display_name);
	term::status::info("Dashboard", dashboard_url(&ctx.game_id, &version_id));

	Ok(CreateOutput { version_id })
}

/// Backwards compatibility for site & Docker build pushing
///
/// Developers should use the parameters inside the config itself instead
pub async fn build_and_push_compat(
	ctx: &cli_core::Ctx,
	overrides: &mut Vec<(String, serde_json::Value)>,
	build_tag: &Option<String>,
	build_name: &Option<String>,
	site_path: &Option<String>,
	site_name: &Option<String>,
	format: &Option<struct_fmt::Format>,
) -> Result<()> {
	let site_output = if let Some(site_path) = site_path {
		Some(
			site::push(
				ctx,
				&site::SitePushOpts {
					path: site_path.clone(),
					name: site_name.clone(),
					format: format.clone(),
				},
			)
			.await?,
		)
	} else {
		None
	};

	let build_output = if let Some(build_tag) = build_tag {
		Some(
			build::push(
				ctx,
				&build::BuildPushOpts {
					tag: build_tag.clone(),
					name: build_name.clone(),
					format: format.clone(),
				},
			)
			.await?,
		)
	} else {
		None
	};

	if let Some(site_output) = site_output {
		overrides.push(("cdn.site".into(), json!(site_output.site_id)));
	}
	if let Some(build_output) = build_output {
		overrides.push((
			"matchmaker.docker.build".into(),
			json!(build_output.build_id),
		));
	}

	Ok(())
}
