/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudVersionEngineUnrealConfig {
	/// Docker image to use for running the server locally & building the server to run on Rivet. See versions [here](https://github.com/orgs/EpicGames/packages/container/package/unreal-engine). _Configures Rivet CLI behavior. Has no effect on server behavior._
	#[serde(rename = "base_docker_image")]
	pub base_docker_image: String,
	/// Name of the Unreal module that holds the game code. This is usually the value of `$.Modules[0].Name` in the file `MyProject.unproject`. _Configures Rivet CLI behavior. Has no effect on server behavior._
	#[serde(rename = "game_module")]
	pub game_module: String,
}

impl CloudVersionEngineUnrealConfig {
	pub fn new(base_docker_image: String, game_module: String) -> CloudVersionEngineUnrealConfig {
		CloudVersionEngineUnrealConfig {
			base_docker_image,
			game_module,
		}
	}
}
