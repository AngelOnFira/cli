# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.3.1](https://github.com/AngelOnFira/cli/compare/rivet-cli-v1.3.0...rivet-cli-v1.3.1) (2024-05-28)


### Bug Fixes

* test ([f9819c3](https://github.com/AngelOnFira/cli/commit/f9819c3d7ec828cabaccbf30295c65c0b895509c))

## [1.3.0](https://github.com/AngelOnFira/cli/compare/rivet-cli-v1.1.0...rivet-cli-v1.3.0) (2024-05-27)


### Features

* add opengb db command passthrough ([#216](https://github.com/AngelOnFira/cli/issues/216)) ([7b78870](https://github.com/AngelOnFira/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/AngelOnFira/cli/issues/231)) ([2fc3021](https://github.com/AngelOnFira/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add support for sh and url db commands ([#217](https://github.com/AngelOnFira/cli/issues/217)) ([bbeeaba](https://github.com/AngelOnFira/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* Implement OpenGB related commands ([#215](https://github.com/AngelOnFira/cli/issues/215)) ([ce57364](https://github.com/AngelOnFira/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/AngelOnFira/cli/issues/226)) ([8a62d97](https://github.com/AngelOnFira/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/AngelOnFira/cli/issues/237)) ([64b3489](https://github.com/AngelOnFira/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/AngelOnFira/cli/issues/208)) ([214fe29](https://github.com/AngelOnFira/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/AngelOnFira/cli/issues/224)) ([2f04ea3](https://github.com/AngelOnFira/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* read_generated_manifest fn name ([#241](https://github.com/AngelOnFira/cli/issues/241)) ([72970c7](https://github.com/AngelOnFira/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/AngelOnFira/cli/issues/220)) ([c1d33c5](https://github.com/AngelOnFira/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* **test:** update region names ([#223](https://github.com/AngelOnFira/cli/issues/223)) ([c605561](https://github.com/AngelOnFira/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update sdks for opengb ([#233](https://github.com/AngelOnFira/cli/issues/233)) ([7feb70b](https://github.com/AngelOnFira/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/AngelOnFira/cli/issues/210)) ([2b928df](https://github.com/AngelOnFira/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/AngelOnFira/cli/issues/229)) ([28d9d93](https://github.com/AngelOnFira/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Continuous Integration

* ignore failing e2e test ([#243](https://github.com/AngelOnFira/cli/issues/243)) ([242e291](https://github.com/AngelOnFira/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Release ([b10bc24](https://github.com/AngelOnFira/cli/commit/b10bc2414434bc1a93690ea2948feb52003f4bcd))
* Release ([6c40a0e](https://github.com/AngelOnFira/cli/commit/6c40a0ea758ba9e845539d2bfc64fcd1bf35b7a9))
* Release ([e9ff7fc](https://github.com/AngelOnFira/cli/commit/e9ff7fc971969963427e9dc50600c3177bab54f0))
* Release ([db9e6ab](https://github.com/AngelOnFira/cli/commit/db9e6abef659db20857bc500f2236323477a3f19))
* Release ([bcd1d34](https://github.com/AngelOnFira/cli/commit/bcd1d3471364ff7a634133b1c0fc8b39594cf636))
* Release ([d1ac3c6](https://github.com/AngelOnFira/cli/commit/d1ac3c69ddf1bb15f284da3886d47bcb9a39e232))
* Release ([d0e2d6b](https://github.com/AngelOnFira/cli/commit/d0e2d6b8557099b203ceb13264dcff95f79b7f05))
* Release ([bd657c8](https://github.com/AngelOnFira/cli/commit/bd657c868f727032d87d3c392adc6ff8709b5272))
* Release ([1a6bec0](https://github.com/AngelOnFira/cli/commit/1a6bec0254b5c6a3f4bda2d690d1129f83bfe702))
* Release ([68819c3](https://github.com/AngelOnFira/cli/commit/68819c36514e7d0cc370e0fbb451a5e0f1fbf80e))
* Release ([2f23b00](https://github.com/AngelOnFira/cli/commit/2f23b009dca0947ef58f619413bc04540b3078c6))
* Release ([d057971](https://github.com/AngelOnFira/cli/commit/d057971c08a872fa23ddffc2c126840ac37a86dd))
* Release ([260ea03](https://github.com/AngelOnFira/cli/commit/260ea0354c13e4c1edd0381ab093dacad624d5e5))
* Release ([9350c01](https://github.com/AngelOnFira/cli/commit/9350c01ee444ce78570f846ffb6c2531cb7dbfe1))
* Release ([dfe5722](https://github.com/AngelOnFira/cli/commit/dfe5722b1bea9d3ce483230f9c1c8db265fc9e5b))
* Release ([2e6508c](https://github.com/AngelOnFira/cli/commit/2e6508ccdecfe204794022068c25f62e117c0f8f))
* Release ([dc96983](https://github.com/AngelOnFira/cli/commit/dc969832d6049b27fc1e3b5a81729c59f6d3be17))
* Release ([3e52457](https://github.com/AngelOnFira/cli/commit/3e52457103a909a3d54954149882de7aa646ec47))
* Release ([cc4f21d](https://github.com/AngelOnFira/cli/commit/cc4f21da33a713f67720e1efa2583df52f8a3a7e))
* remove experimental flag from run & exec commands ([#222](https://github.com/AngelOnFira/cli/issues/222)) ([c9cfae6](https://github.com/AngelOnFira/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/AngelOnFira/cli/issues/221)) ([0f47917](https://github.com/AngelOnFira/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* update docker root user help link ([#214](https://github.com/AngelOnFira/cli/issues/214)) ([30fdc56](https://github.com/AngelOnFira/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))

## [v1.1.0] - 2024-04-13

### Added

- `rivet run` and `rivet exec` are no longer experimental

### Changed

- Rename `--rivet-servers` to `--servers` and `--this-machine` to `--dev` for `rivet run` and `rivet exec`

### Fixed

- `rivet exec` does not respect `--rivet-servers` flag

## [v1.0.2] - 2024-02-29

### Changed

- Progress bars will consolidate to 1 if there are more than 40 files being uploaded
- Update SDKs

### Fixed

- `cdn.build_env` not being passed to `cdn.build_cmd`

## [v1.0.1] - 2024-01-29

### Changed

- Improved progress indicators on file uploads

### Fixed

- Docker image UID & GID validation not getting ran
- Lack of a newline printed by `rivet token create` causing EOL mark to appear on zsh shells

## [v1.0.0] - 2024-01-23

## [v1.0.0-rc.3] - 2024-01-19

### Added

- Shorthand API endpoints can now be passed without the scheme (e.g. `api.mydomain.com` or `127.0.0.1:8080`)
- `rivet global-config read-project` command
- `rivet global-config path` command to get the path to the global config
- `--format` now supports `json-compact`

### Changed

- `--format json` now defaults to pretty-printed JSON

### Fixed

- `rivet unlink` now works even if the credentials are invalid
- Docker image UID & GID validation no longer disabled by default

## [v1.0.0-rc.2] - 2024-01-13

### Added

- `rivet exec` command to run arbitrary commands with `RIVET_API_ENDPOINT` and `RIVET_TOKEN` environment variables
- `rivet run` command to run scripts from the `scripts` portion of `rivet.yaml` with `RIVET_API_ENDPOINT`, `RIVET_TOKEN`, and `RIVET_NAMESPACE` environment variables
- `rivet deploy` now can now specify the namespace inline (e.g. `rivet deploy prod` instead of `rivet deploy -n prod`)
- `matchmaker.docker.build_args` to configure Docker build args
- `cdn.build_env` to configure environment variables for building the site
- `RIVET_API_ENDPOINT` and `RIVET_NAMESPACE` arg is passed to `docker build` by default
- `RIVET_TOKEN` and `RIVET_NAMESPACE` now additionally passed to `cdn.build_command`

### Changed

- Reworked `rivet init` process to cleanly communicate next steps & unique links for the selected engine
- Updated generated `rivet.yaml` on `rivet init` to be more concise and helpful & unique content for the selected engine
- Update OCI bundle archival process to operate on TAR streams instead of using the host's file system to preserve ownership & permissions
- **[BREAKING]** `rivet deploy` now requires a `--no-namespace` flag if no namespace is provided

### Fixed

- Overriding `matchmaker.docker.image_id` getting ignored
- `rivet config validate` now uses `--print` flag instead of a positional argument
- Validate Docker images do not run as GID 0

## [v1.0.0-rc.1] - 2023-12-24

### Added

- Add `x86_64-unknown-linux-musl` artifact
- Version names are now generated with incrementing indexes on the backend without race conditions
- Warning if running unauthenticated commands as a sudo user
- `sidekick unlink` subcommand to unlink the current project from the
  Rivet CLI
- `sidekick generate-config` subcommand to generate a Rivet config file
- `sidekick get-namespace-dev-token` and `sidekick
get-namespace-public-token` subcommands to get a Rivet token for a namespace
- `sidekick get-bootstrap-data` subcommand to get the initial data about
  the signed-in user
- `sidekick get-cli-version` subcommand to get the version of the Rivet
  CLI
- `sidekick deploy` to do the process of deploying a build to Rivet
- ability for `sidekick` to open terminal in a new window for commands
  that need to be shown (e.g. `sidekick deploy`)
- `sidekick get-version` subcommand to get the manage version URL in the
  hub
- `sidekick get-token` subcommand to get a Rivet token for a user
- `sidekick check-login-state` subcommand to see if a user is logged in
  through the CLI
- `sidekick wait-for-login` subcommand to long-poll for a user to sign in
- `sidekick get-link` subcommand to get a sign-in link for a user
- hidden `Sidekick` subcommand to be used by external tools (e.g. engine
  plugins) to interact with the Rivet CLI

### Changed

- Cleaner unauthenticated error
- Changed `sidekick` to a more modular architecture
- Changed error handling in CLI to use `GlobalResult` from main repo instead of
  `anyhow`
- Unix install script can now take the environment variable `BIN_DIR` to specify
  the installation directory, preventing the need for sudo in certain cases
- Rivet CLI now references the `rivet-cli-api` from the Rivet main repo rather
  than storing its own copy
- Update `cargo-dist` to 0.6.2

### Fixed

- Custom engines no longer get prompted to select engine when running `rivet init` for the second time
- Windows compilation no longer fails with `nix` dependency
- `--telemetry-disabled` no longer requires explicit `true`
- Collect system metrics using `sysinfo::System` instead of `uname` command for compatability with Windows
- CDN URL on deploy complete now pulls dynamic DNS from bootstrap API
- CDN URL on deploy complete is no longer displayed if CDN is not enabled for the game

## [v0.4.0] - 2023-12-20

### Added

- Auto-generate GitHub Actions with `rivet ci generate github`
- Development token cache to make `rivet token create development` run faster
- Shorthand `-n` for `--namespace` flag in `rivet token create development`
- `rivet deploy` validates config before building & uploading resources
- `rivet unlink` command to remove authentication token
- Pretty-printed errors instead of default debug format
- Error reporting to Sentry

### Changed

- Removed engine prompt if Rivet config already exists
- **[BREAKING]** No longer automatically creates/updates `.env` file in favor of using `rivet token create development`
- Global flags (`--api-endpoint`, `--token`, and `--disable-telemetry`) can now be used in subcommands (e.g. `rivet init --token foobar` instead of `rivet --token foobar init`)
- Moved project metadata to global configuration file
- Removed `.rivet` from auto-generated `.gitignore`
- `rivet namespace create` can be called without specifying `--version`
- **[BREAKING]** Change `TELEMETRY_DISABLED` env var to `RIVET_TELEMETRY_DISABLED`
- Remove trailing line break from `rivet token create development`
- Rename `rivet site` subcommands to `rivet cdn` (alias still supported)
- Rename `rivet image` subcommands to `rivet docker` (alias still supported)
- Rename `dashboard` subcommands to `view` (alias still supported)
- Move `rivet version deploy` to `rivet deploy`
- Move `rivet version config-validate` to `rivet config validate`
- Move `RIVET_CONCURRENT_UPLOADS` env var to CLI flag on appropriate commands (env var still works)
- Streamline `rivet init` experience
- Add `rivet token create public` command

### Fixed

- Fix `matchmaker.game_modes.*.docker.image_id` falling back to `matchmaker.docker.image_id`
- **Install script** Now installs non-prerelease GitHub releases

## [v0.3.0] - 2023-12-10

### Added

- **Install script (Unix)** Configure installation directory by passing `$BIN_DIR`
- **Install script (Unix)** Warning if `$BIN_DIR` is not in `$PATH`

### Changed

- Auto-generated & recommended config is now a `rivet.yaml` file
- Default version names are now generated as `YYYY.MM (X)` format (where `X` is an incrementing index)
- Merged `.rivet/cloud_token` and `.rivet/config.toml` in to unified internal `.rivet/config.yaml` config file
- **[BREAKING]** Removed support for file formats that are not YAML, TOML, or JSON in order to simplify maintaining forward compatibility
- **[BREAKING]** Throw error if both `.yaml` and `.yml` config exist

### Fixed

- **Install script (Unix)** Installing ARM64 `jq` binary on ARM-based Macs
- **Install script (Unix)** Automatically create `$BIN_DIR` if doesn't exist, specifically on macOS Sonoma which does not provide a `/usr/local/bin` by default

## [v0.2.0] - 2023-12-1

### Added

- Support for building OCI bundles
- Support for LZ4 compression of builds
- **[BREAKING]** Expose `RIVET_API_ENDPOINT` to `cdn.build_command` to help automate deploying to multiple clusters
- **[BREAKING]** Unset `RIVET_TOKEN` to `cdn.build_command` in order to ensure the cloud token isn't accidentally baked in a build
- `image build-push` command to automatically build & push an image
- `site build-push` command to automatially build and push a site
- E2E cross-platform tests in GitHub Actions

### Changed

- **[BREAKING]** Support new single-origin API endpoint (configured with `RIVET_API_ENDPOINT` environment variable or `--api-endpoint` flag)
- **[BREAKING]** Rename `RIVET_CLOUD_TOKEN` environment variable to `RIVET_TOKEN`
- **[BREAKING]** Rename `--cloud-token` flag to `--token`
- **[BREAKING]** Removed `RIVET_API_CLOUD_URL` in favor of `RIVET_API_ENDPOINT`
- **[BREAKING]** Updated custom games config schema
- **[BREAKING]** Removed domain map from turnstile configuration, replaced with `site_key` and `secret_key`
- Added telemetry beacon for fatal errors. Opt out with `--telemetry-disabled` or `TELEMETRY_DISABLED=1`
- Added internal config to store api endpoint and telemetry options
- Implemented multipart uploads for builds and sites, disable multipart uploads with `_RIVET_UPLOAD_DISABLE_MULTIPART`

## [v0.1.4] - 2023-12-9

### Added

- Darwin ARM release artifact

### Changed

- Update `cargo-dist` to 0.5.0

## [v0.1.3] - 2023-12-3

### Changed

- Replace Smithy-generated API library with OpenAPI-generated library in order to fix `invalid certificate timestamp: UnknownLog` error

## [v0.1.2] - 2023-08-26

### Changed

- Added custom games + lobby state + external verification

## [v0.1.1] - 2023-07-17

### Changed

- `rivet deploy` now gracefully falls back to the native build method if Docker Buildx is not installed

## [v0.1.0] - 2023-07-17

### Added

- Unreal helper in `rivet init`
- Installer for the Unreal Engine plugin with `rivet unreal install-plugin` or `rivet init --unreal`

### Changed

- Renamed `rivet.version.toml` to `rivet.toml`. All changes are backwards compatible.
- Renamed `rivet publish` command to `rivet deploy` since this is the more commonly used alias
- `rivet token create dev` now prints token in plain text

### Fixed

- Broken links to old docs
- Docker builder now catches missing builder errors correctly for older Docker versions

## [v0.0.51] - 2023-04-26

### Fixed

- Docker builder now catches missing builder errors correctly for older Docker versions

### Changed

- Remove `PORT`, `RIVET_LOBBY_TOKEN`, and `RIVET_PUBLIC_TOKEN` from generated .env file
- Document development token in .env

## [v0.0.50] - 2023-04-18

### Changed

- Description, homepage, and repository to Cargo.toml

### Fixed

- Incorrect package version

## [v0.0.49] - 2023-04-18

### Added

- Experimental build configuration flag `_RIVET_DOCKER_BUILD_METHOD` can be set to `buildx` or `native`

### Changed

- Default Docker build method is now Buildx, even if the native platform is x86
- Update dependency: `rivet-api`
- Upgrade dependency: `tokio 1.27`
- Removed unnecessary feature flags from `tokio`
