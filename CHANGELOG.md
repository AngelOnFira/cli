# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.5.0](https://github.com/AngelOnFira/cli/compare/v2.4.0...v2.5.0) (2024-08-10)


### Bug Fixes

* remove sentry feature ([86b054c](https://github.com/AngelOnFira/cli/commit/86b054cee56e1460a96878cf45588a2be6619b19))


### Chores

* release 2.5.0 ([3b9ad02](https://github.com/AngelOnFira/cli/commit/3b9ad029f08239a30b490c1988678debe4432c6c))

## [2.4.0](https://github.com/AngelOnFira/cli/compare/v2.3.0...v2.4.0) (2024-08-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/AngelOnFira/cli/issues/262)) ([6255e07](https://github.com/AngelOnFira/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))
* add ffi ([#291](https://github.com/AngelOnFira/cli/issues/291)) ([ffd015b](https://github.com/AngelOnFira/cli/commit/ffd015b3cc711bef5b533e3d002f6d79c38cd26e))
* add opengb db command passthrough ([#216](https://github.com/AngelOnFira/cli/issues/216)) ([7b78870](https://github.com/AngelOnFira/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/AngelOnFira/cli/issues/231)) ([2fc3021](https://github.com/AngelOnFira/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add settings file ([#278](https://github.com/AngelOnFira/cli/issues/278)) ([c004d05](https://github.com/AngelOnFira/cli/commit/c004d05e7255713a402b4d049b62615d1d14dfd2))
* add sidekick exec command ([#275](https://github.com/AngelOnFira/cli/issues/275)) ([3121f94](https://github.com/AngelOnFira/cli/commit/3121f940e4955e3cfa5772b8f98486f1ad323064))
* add sidekick show-term command ([#266](https://github.com/AngelOnFira/cli/issues/266)) ([5fc1c89](https://github.com/AngelOnFira/cli/commit/5fc1c89c3205f93411fd672934dbebf7bb1306ff))
* add support for sh and url db commands ([#217](https://github.com/AngelOnFira/cli/issues/217)) ([bbeeaba](https://github.com/AngelOnFira/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* allow configuring opengb config path ([#283](https://github.com/AngelOnFira/cli/issues/283)) ([aa8c212](https://github.com/AngelOnFira/cli/commit/aa8c21284855489e2548be803e0e68a12ceeaf3e))
* **cli:** add config command ([#296](https://github.com/AngelOnFira/cli/issues/296)) ([3787d0c](https://github.com/AngelOnFira/cli/commit/3787d0c49d013477942f424c23f684a2f0266f7c))
* get lobby and logs links in sidekick ([#235](https://github.com/AngelOnFira/cli/issues/235)) ([7c63efd](https://github.com/AngelOnFira/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))
* Implement OpenGB related commands ([#215](https://github.com/AngelOnFira/cli/issues/215)) ([ce57364](https://github.com/AngelOnFira/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))
* run opengb using docker by default ([#254](https://github.com/AngelOnFira/cli/issues/254)) ([598ce8d](https://github.com/AngelOnFira/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))
* **sidekick:** add backend dev command ([#274](https://github.com/AngelOnFira/cli/issues/274)) ([7547384](https://github.com/AngelOnFira/cli/commit/75473841fb6fa2780f50c5daea6f86daeca730f7))
* **sidekick:** add backend gen command ([#273](https://github.com/AngelOnFira/cli/issues/273)) ([2f1358e](https://github.com/AngelOnFira/cli/commit/2f1358e9f857fec27b3310ee6357ba369af583d2))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/AngelOnFira/cli/issues/226)) ([8a62d97](https://github.com/AngelOnFira/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/AngelOnFira/cli/issues/237)) ([64b3489](https://github.com/AngelOnFira/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/AngelOnFira/cli/issues/208)) ([214fe29](https://github.com/AngelOnFira/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* ci again ([f11f7a5](https://github.com/AngelOnFira/cli/commit/f11f7a5056606fabab4274456a4470342c9ccb9a))
* ci thing ([4549d7b](https://github.com/AngelOnFira/cli/commit/4549d7b3b0d4e9975e41cbbfbcf44fc01da07e24))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/AngelOnFira/cli/issues/224)) ([2f04ea3](https://github.com/AngelOnFira/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* dirty ([023870c](https://github.com/AngelOnFira/cli/commit/023870c8d74b0514fb8cc1ef2a5fef2b3d74a05e))
* disable hyper connection pooling ([#293](https://github.com/AngelOnFira/cli/issues/293)) ([bddfa6d](https://github.com/AngelOnFira/cli/commit/bddfa6dc996c42b476f9c8cc301a0cfd5981440f))
* **opengb:** one db per env ([#256](https://github.com/AngelOnFira/cli/issues/256)) ([a3c4e10](https://github.com/AngelOnFira/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* prevent asking user for terminal permissions ([#236](https://github.com/AngelOnFira/cli/issues/236)) ([a1a75d8](https://github.com/AngelOnFira/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/AngelOnFira/cli/issues/241)) ([72970c7](https://github.com/AngelOnFira/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/AngelOnFira/cli/issues/238)) ([e177ad4](https://github.com/AngelOnFira/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))
* revert regression with config-rs dependency ([#270](https://github.com/AngelOnFira/cli/issues/270)) ([ff3afa8](https://github.com/AngelOnFira/cli/commit/ff3afa8d12b1743b20365103b95fece74b4d1a39))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/AngelOnFira/cli/issues/220)) ([c1d33c5](https://github.com/AngelOnFira/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* show term args on linux ([#286](https://github.com/AngelOnFira/cli/issues/286)) ([6e94ab5](https://github.com/AngelOnFira/cli/commit/6e94ab594f320a78fd6a57f1f2b8d33d30723969))
* temp disable flakey macos test ([#212](https://github.com/AngelOnFira/cli/issues/212)) ([6d1ed9b](https://github.com/AngelOnFira/cli/commit/6d1ed9b87b7aa9cdc249a895c7d5ea47ef9d5be7))
* **test:** update region names ([#223](https://github.com/AngelOnFira/cli/issues/223)) ([c605561](https://github.com/AngelOnFira/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update cargo-release version ([450d25b](https://github.com/AngelOnFira/cli/commit/450d25b060da494a6ec44990b7575772e765db3e))
* update sdks for opengb ([#233](https://github.com/AngelOnFira/cli/issues/233)) ([7feb70b](https://github.com/AngelOnFira/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/AngelOnFira/cli/issues/210)) ([2b928df](https://github.com/AngelOnFira/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/AngelOnFira/cli/issues/229)) ([28d9d93](https://github.com/AngelOnFira/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Documentation

* release script instructions ([#248](https://github.com/AngelOnFira/cli/issues/248)) ([0d9edb3](https://github.com/AngelOnFira/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Code Refactoring

* move global config to meta config ([#279](https://github.com/AngelOnFira/cli/issues/279)) ([8ae2ed1](https://github.com/AngelOnFira/cli/commit/8ae2ed1ecf69eaac7f044cdddd270883906152a2))
* move shared functionality to toolchain ([#277](https://github.com/AngelOnFira/cli/issues/277)) ([d479186](https://github.com/AngelOnFira/cli/commit/d479186499db118a4aef2d54bd7cb64ab4581187))


### Continuous Integration

* and release please pr ([#244](https://github.com/AngelOnFira/cli/issues/244)) ([9862c5a](https://github.com/AngelOnFira/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* bring back release please ([#299](https://github.com/AngelOnFira/cli/issues/299)) ([730b694](https://github.com/AngelOnFira/cli/commit/730b69491dc8fd981a2248502b9c5780ce201a47))
* change label update to merged PRs ([#249](https://github.com/AngelOnFira/cli/issues/249)) ([06a938c](https://github.com/AngelOnFira/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))
* change release-please pr labels on release ([#247](https://github.com/AngelOnFira/cli/issues/247)) ([336f789](https://github.com/AngelOnFira/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/AngelOnFira/cli/issues/242)) ([f14b17e](https://github.com/AngelOnFira/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/AngelOnFira/cli/issues/243)) ([242e291](https://github.com/AngelOnFira/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* add back cli ([#288](https://github.com/AngelOnFira/cli/issues/288)) ([9ca98e4](https://github.com/AngelOnFira/cli/commit/9ca98e49c9536515dbd434e853f846f19d8af0e0))
* add back loading config ([#280](https://github.com/AngelOnFira/cli/issues/280)) ([06347e1](https://github.com/AngelOnFira/cli/commit/06347e168367e26cfcbe00264b2d8d4b9f23bbba))
* add deploy command ([#294](https://github.com/AngelOnFira/cli/issues/294)) ([e9bffdb](https://github.com/AngelOnFira/cli/commit/e9bffdbf163e70c39c5357d6f95321edfa791f91))
* add show_term task ([#289](https://github.com/AngelOnFira/cli/issues/289)) ([72b37e3](https://github.com/AngelOnFira/cli/commit/72b37e376d2d6db838c8b148d67c0089200fe8de))
* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/AngelOnFira/cli/issues/228)) ([a192e35](https://github.com/AngelOnFira/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* bump version ([71c215f](https://github.com/AngelOnFira/cli/commit/71c215febeebff27fad0cac1938e4e6663a12cdd))
* check system requirements ([#282](https://github.com/AngelOnFira/cli/issues/282)) ([7858080](https://github.com/AngelOnFira/cli/commit/7858080b62def9edfcf90fcc2d9632cae8816781))
* choose free port for opengb ([#281](https://github.com/AngelOnFira/cli/issues/281)) ([1601f55](https://github.com/AngelOnFira/cli/commit/1601f55e8729802940f1c09614c850b2ee846e34))
* default RunConfig will print to stdout ([#290](https://github.com/AngelOnFira/cli/issues/290)) ([aa7de1a](https://github.com/AngelOnFira/cli/commit/aa7de1a3c4d2eec9aa768ae45a5f1774bf3607ad))
* fmt sdk ([#251](https://github.com/AngelOnFira/cli/issues/251)) ([28eade7](https://github.com/AngelOnFira/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* format ([#297](https://github.com/AngelOnFira/cli/issues/297)) ([8ba0ec6](https://github.com/AngelOnFira/cli/commit/8ba0ec60adcd24187e8f74838533dace65aac618))
* **main:** release 1.2.0 ([555fec1](https://github.com/AngelOnFira/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))
* **main:** release 1.3.0 ([#246](https://github.com/AngelOnFira/cli/issues/246)) ([cfa546c](https://github.com/AngelOnFira/cli/commit/cfa546cf84ca89f7234220f030e9080a1197abb2))
* **main:** release 1.3.1 ([#250](https://github.com/AngelOnFira/cli/issues/250)) ([9a76764](https://github.com/AngelOnFira/cli/commit/9a76764f9e832736e036a3fb3bdb1b6efdad3b5b))
* **main:** release 1.3.2 ([#255](https://github.com/AngelOnFira/cli/issues/255)) ([e232c5f](https://github.com/AngelOnFira/cli/commit/e232c5f0d41b7c9d33d745c3737c008c29d45803))
* **main:** release 1.3.3 ([#257](https://github.com/AngelOnFira/cli/issues/257)) ([3d1c35f](https://github.com/AngelOnFira/cli/commit/3d1c35f3fe24d0efd7ff50e9ca2bbe063cb2da31))
* **main:** release 1.3.4 ([#272](https://github.com/AngelOnFira/cli/issues/272)) ([3d5d757](https://github.com/AngelOnFira/cli/commit/3d5d7574b6e628c968d4be91a1db9bc6e5ed9c97))
* **main:** release 1.9.0 ([#10](https://github.com/AngelOnFira/cli/issues/10)) ([83f8817](https://github.com/AngelOnFira/cli/commit/83f8817c2a9b41d5b8ca9fb9bd8fd44a23505786))
* **main:** release 2.0.0-rc.1 ([#292](https://github.com/AngelOnFira/cli/issues/292)) ([3e378b3](https://github.com/AngelOnFira/cli/commit/3e378b39caca911c231fb1a1f045965278fa67fc))
* **main:** release 2.2.0 ([#11](https://github.com/AngelOnFira/cli/issues/11)) ([7485172](https://github.com/AngelOnFira/cli/commit/7485172362df1e6c95d783149e2859b2f9435109))
* **main:** release 2.3.0 ([#12](https://github.com/AngelOnFira/cli/issues/12)) ([4fd2e36](https://github.com/AngelOnFira/cli/commit/4fd2e365cfecd7618e4d7a0e325021fb4bf83925))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/AngelOnFira/cli/issues/253)) ([4b31887](https://github.com/AngelOnFira/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/AngelOnFira/cli/issues/234)) ([a9c1b29](https://github.com/AngelOnFira/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))
* Release ([fcbbb99](https://github.com/AngelOnFira/cli/commit/fcbbb9952c39ba5c93c622b146091f91cd3f27ee))
* Release ([76e486d](https://github.com/AngelOnFira/cli/commit/76e486d99e2a90b8066c8aec68ecefcff64e24ed))
* Release ([3d3f548](https://github.com/AngelOnFira/cli/commit/3d3f548f059def37e538301c2d587ac75148d074))
* Release ([04d004e](https://github.com/AngelOnFira/cli/commit/04d004efed9f6693c53eb3d0e5224476dadf8391))
* Release ([4951a00](https://github.com/AngelOnFira/cli/commit/4951a001465ceeeab209978b47053b03d6e32ec1))
* Release ([48b18ba](https://github.com/AngelOnFira/cli/commit/48b18baca3cf99cd80870bedeabe966bbb77b0cd))
* Release ([ab5e081](https://github.com/AngelOnFira/cli/commit/ab5e081f3f1eb4ac12c9b54f90953318e640afb6))
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
* release 1.3.2 ([50bd2be](https://github.com/AngelOnFira/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))
* release 1.3.3 ([aed2a03](https://github.com/AngelOnFira/cli/commit/aed2a039b01d27c96008f9fbc82c8b25251e2eb8))
* release 1.9.0 ([24eac90](https://github.com/AngelOnFira/cli/commit/24eac905528b1b138eb6754f2618e6d13ee9ddc3))
* release 2.0.0-rc.1 ([8882ad8](https://github.com/AngelOnFira/cli/commit/8882ad88d8b50422fabdb732ebb96aed3e7940c1))
* release 2.2.0 ([3990651](https://github.com/AngelOnFira/cli/commit/3990651501d7a47b5458d0bd2e91e96ace5d2a28))
* release 2.3.0 ([7d08e9e](https://github.com/AngelOnFira/cli/commit/7d08e9e67cc803f23585d7ecf94c1e3850d5b25a))
* release 2.4.0 ([f61286d](https://github.com/AngelOnFira/cli/commit/f61286d3c4792cf64acd116d82fc905126a61953))
* remove experimental flag from run & exec commands ([#222](https://github.com/AngelOnFira/cli/issues/222)) ([c9cfae6](https://github.com/AngelOnFira/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/AngelOnFira/cli/issues/221)) ([0f47917](https://github.com/AngelOnFira/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* reorg package in prep for toolchain ([#276](https://github.com/AngelOnFira/cli/issues/276)) ([371ac16](https://github.com/AngelOnFira/cli/commit/371ac16c28e293b7f46f066c16e53dcf9b233eae))
* return backend project on bootstrap ([#284](https://github.com/AngelOnFira/cli/issues/284)) ([40ea899](https://github.com/AngelOnFira/cli/commit/40ea8998935afda712bba36cddf9b4297538c140))
* Update cargo dist ([#271](https://github.com/AngelOnFira/cli/issues/271)) ([c2355ae](https://github.com/AngelOnFira/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))
* update docker root user help link ([#214](https://github.com/AngelOnFira/cli/issues/214)) ([30fdc56](https://github.com/AngelOnFira/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))
* update opengb flag --path -&gt; --project ([#295](https://github.com/AngelOnFira/cli/issues/295)) ([6b5407e](https://github.com/AngelOnFira/cli/commit/6b5407e5f0b03cfcda22bb3d9640cac52aa82ecb))
* update posthog api key ([#263](https://github.com/AngelOnFira/cli/issues/263)) ([5f2ee58](https://github.com/AngelOnFira/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))
* update rivet-api ([#209](https://github.com/AngelOnFira/cli/issues/209)) ([b057d00](https://github.com/AngelOnFira/cli/commit/b057d00c0fc897b9f663a55c4aff8845685745c4))
* update sdk for env endpoint ([#285](https://github.com/AngelOnFira/cli/issues/285)) ([73c7120](https://github.com/AngelOnFira/cli/commit/73c7120e08da4881fa901bc153f7a77b31cf4262))

## [2.3.0](https://github.com/AngelOnFira/cli/compare/v2.2.0...v2.3.0) (2024-08-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/AngelOnFira/cli/issues/262)) ([6255e07](https://github.com/AngelOnFira/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))
* add ffi ([#291](https://github.com/AngelOnFira/cli/issues/291)) ([ffd015b](https://github.com/AngelOnFira/cli/commit/ffd015b3cc711bef5b533e3d002f6d79c38cd26e))
* add opengb db command passthrough ([#216](https://github.com/AngelOnFira/cli/issues/216)) ([7b78870](https://github.com/AngelOnFira/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/AngelOnFira/cli/issues/231)) ([2fc3021](https://github.com/AngelOnFira/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add settings file ([#278](https://github.com/AngelOnFira/cli/issues/278)) ([c004d05](https://github.com/AngelOnFira/cli/commit/c004d05e7255713a402b4d049b62615d1d14dfd2))
* add sidekick exec command ([#275](https://github.com/AngelOnFira/cli/issues/275)) ([3121f94](https://github.com/AngelOnFira/cli/commit/3121f940e4955e3cfa5772b8f98486f1ad323064))
* add sidekick show-term command ([#266](https://github.com/AngelOnFira/cli/issues/266)) ([5fc1c89](https://github.com/AngelOnFira/cli/commit/5fc1c89c3205f93411fd672934dbebf7bb1306ff))
* add support for sh and url db commands ([#217](https://github.com/AngelOnFira/cli/issues/217)) ([bbeeaba](https://github.com/AngelOnFira/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* allow configuring opengb config path ([#283](https://github.com/AngelOnFira/cli/issues/283)) ([aa8c212](https://github.com/AngelOnFira/cli/commit/aa8c21284855489e2548be803e0e68a12ceeaf3e))
* **cli:** add config command ([#296](https://github.com/AngelOnFira/cli/issues/296)) ([3787d0c](https://github.com/AngelOnFira/cli/commit/3787d0c49d013477942f424c23f684a2f0266f7c))
* get lobby and logs links in sidekick ([#235](https://github.com/AngelOnFira/cli/issues/235)) ([7c63efd](https://github.com/AngelOnFira/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))
* Implement OpenGB related commands ([#215](https://github.com/AngelOnFira/cli/issues/215)) ([ce57364](https://github.com/AngelOnFira/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))
* run opengb using docker by default ([#254](https://github.com/AngelOnFira/cli/issues/254)) ([598ce8d](https://github.com/AngelOnFira/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))
* **sidekick:** add backend dev command ([#274](https://github.com/AngelOnFira/cli/issues/274)) ([7547384](https://github.com/AngelOnFira/cli/commit/75473841fb6fa2780f50c5daea6f86daeca730f7))
* **sidekick:** add backend gen command ([#273](https://github.com/AngelOnFira/cli/issues/273)) ([2f1358e](https://github.com/AngelOnFira/cli/commit/2f1358e9f857fec27b3310ee6357ba369af583d2))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/AngelOnFira/cli/issues/226)) ([8a62d97](https://github.com/AngelOnFira/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/AngelOnFira/cli/issues/237)) ([64b3489](https://github.com/AngelOnFira/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/AngelOnFira/cli/issues/208)) ([214fe29](https://github.com/AngelOnFira/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* ci again ([f11f7a5](https://github.com/AngelOnFira/cli/commit/f11f7a5056606fabab4274456a4470342c9ccb9a))
* ci thing ([4549d7b](https://github.com/AngelOnFira/cli/commit/4549d7b3b0d4e9975e41cbbfbcf44fc01da07e24))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/AngelOnFira/cli/issues/224)) ([2f04ea3](https://github.com/AngelOnFira/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* disable hyper connection pooling ([#293](https://github.com/AngelOnFira/cli/issues/293)) ([bddfa6d](https://github.com/AngelOnFira/cli/commit/bddfa6dc996c42b476f9c8cc301a0cfd5981440f))
* **opengb:** one db per env ([#256](https://github.com/AngelOnFira/cli/issues/256)) ([a3c4e10](https://github.com/AngelOnFira/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* prevent asking user for terminal permissions ([#236](https://github.com/AngelOnFira/cli/issues/236)) ([a1a75d8](https://github.com/AngelOnFira/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/AngelOnFira/cli/issues/241)) ([72970c7](https://github.com/AngelOnFira/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/AngelOnFira/cli/issues/238)) ([e177ad4](https://github.com/AngelOnFira/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))
* revert regression with config-rs dependency ([#270](https://github.com/AngelOnFira/cli/issues/270)) ([ff3afa8](https://github.com/AngelOnFira/cli/commit/ff3afa8d12b1743b20365103b95fece74b4d1a39))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/AngelOnFira/cli/issues/220)) ([c1d33c5](https://github.com/AngelOnFira/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* show term args on linux ([#286](https://github.com/AngelOnFira/cli/issues/286)) ([6e94ab5](https://github.com/AngelOnFira/cli/commit/6e94ab594f320a78fd6a57f1f2b8d33d30723969))
* temp disable flakey macos test ([#212](https://github.com/AngelOnFira/cli/issues/212)) ([6d1ed9b](https://github.com/AngelOnFira/cli/commit/6d1ed9b87b7aa9cdc249a895c7d5ea47ef9d5be7))
* **test:** update region names ([#223](https://github.com/AngelOnFira/cli/issues/223)) ([c605561](https://github.com/AngelOnFira/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update cargo-release version ([450d25b](https://github.com/AngelOnFira/cli/commit/450d25b060da494a6ec44990b7575772e765db3e))
* update sdks for opengb ([#233](https://github.com/AngelOnFira/cli/issues/233)) ([7feb70b](https://github.com/AngelOnFira/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/AngelOnFira/cli/issues/210)) ([2b928df](https://github.com/AngelOnFira/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/AngelOnFira/cli/issues/229)) ([28d9d93](https://github.com/AngelOnFira/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Documentation

* release script instructions ([#248](https://github.com/AngelOnFira/cli/issues/248)) ([0d9edb3](https://github.com/AngelOnFira/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Code Refactoring

* move global config to meta config ([#279](https://github.com/AngelOnFira/cli/issues/279)) ([8ae2ed1](https://github.com/AngelOnFira/cli/commit/8ae2ed1ecf69eaac7f044cdddd270883906152a2))
* move shared functionality to toolchain ([#277](https://github.com/AngelOnFira/cli/issues/277)) ([d479186](https://github.com/AngelOnFira/cli/commit/d479186499db118a4aef2d54bd7cb64ab4581187))


### Continuous Integration

* and release please pr ([#244](https://github.com/AngelOnFira/cli/issues/244)) ([9862c5a](https://github.com/AngelOnFira/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* bring back release please ([#299](https://github.com/AngelOnFira/cli/issues/299)) ([730b694](https://github.com/AngelOnFira/cli/commit/730b69491dc8fd981a2248502b9c5780ce201a47))
* change label update to merged PRs ([#249](https://github.com/AngelOnFira/cli/issues/249)) ([06a938c](https://github.com/AngelOnFira/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))
* change release-please pr labels on release ([#247](https://github.com/AngelOnFira/cli/issues/247)) ([336f789](https://github.com/AngelOnFira/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/AngelOnFira/cli/issues/242)) ([f14b17e](https://github.com/AngelOnFira/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/AngelOnFira/cli/issues/243)) ([242e291](https://github.com/AngelOnFira/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* add back cli ([#288](https://github.com/AngelOnFira/cli/issues/288)) ([9ca98e4](https://github.com/AngelOnFira/cli/commit/9ca98e49c9536515dbd434e853f846f19d8af0e0))
* add back loading config ([#280](https://github.com/AngelOnFira/cli/issues/280)) ([06347e1](https://github.com/AngelOnFira/cli/commit/06347e168367e26cfcbe00264b2d8d4b9f23bbba))
* add deploy command ([#294](https://github.com/AngelOnFira/cli/issues/294)) ([e9bffdb](https://github.com/AngelOnFira/cli/commit/e9bffdbf163e70c39c5357d6f95321edfa791f91))
* add show_term task ([#289](https://github.com/AngelOnFira/cli/issues/289)) ([72b37e3](https://github.com/AngelOnFira/cli/commit/72b37e376d2d6db838c8b148d67c0089200fe8de))
* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/AngelOnFira/cli/issues/228)) ([a192e35](https://github.com/AngelOnFira/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* bump version ([71c215f](https://github.com/AngelOnFira/cli/commit/71c215febeebff27fad0cac1938e4e6663a12cdd))
* check system requirements ([#282](https://github.com/AngelOnFira/cli/issues/282)) ([7858080](https://github.com/AngelOnFira/cli/commit/7858080b62def9edfcf90fcc2d9632cae8816781))
* choose free port for opengb ([#281](https://github.com/AngelOnFira/cli/issues/281)) ([1601f55](https://github.com/AngelOnFira/cli/commit/1601f55e8729802940f1c09614c850b2ee846e34))
* default RunConfig will print to stdout ([#290](https://github.com/AngelOnFira/cli/issues/290)) ([aa7de1a](https://github.com/AngelOnFira/cli/commit/aa7de1a3c4d2eec9aa768ae45a5f1774bf3607ad))
* fmt sdk ([#251](https://github.com/AngelOnFira/cli/issues/251)) ([28eade7](https://github.com/AngelOnFira/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* format ([#297](https://github.com/AngelOnFira/cli/issues/297)) ([8ba0ec6](https://github.com/AngelOnFira/cli/commit/8ba0ec60adcd24187e8f74838533dace65aac618))
* **main:** release 1.2.0 ([555fec1](https://github.com/AngelOnFira/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))
* **main:** release 1.3.0 ([#246](https://github.com/AngelOnFira/cli/issues/246)) ([cfa546c](https://github.com/AngelOnFira/cli/commit/cfa546cf84ca89f7234220f030e9080a1197abb2))
* **main:** release 1.3.1 ([#250](https://github.com/AngelOnFira/cli/issues/250)) ([9a76764](https://github.com/AngelOnFira/cli/commit/9a76764f9e832736e036a3fb3bdb1b6efdad3b5b))
* **main:** release 1.3.2 ([#255](https://github.com/AngelOnFira/cli/issues/255)) ([e232c5f](https://github.com/AngelOnFira/cli/commit/e232c5f0d41b7c9d33d745c3737c008c29d45803))
* **main:** release 1.3.3 ([#257](https://github.com/AngelOnFira/cli/issues/257)) ([3d1c35f](https://github.com/AngelOnFira/cli/commit/3d1c35f3fe24d0efd7ff50e9ca2bbe063cb2da31))
* **main:** release 1.3.4 ([#272](https://github.com/AngelOnFira/cli/issues/272)) ([3d5d757](https://github.com/AngelOnFira/cli/commit/3d5d7574b6e628c968d4be91a1db9bc6e5ed9c97))
* **main:** release 1.9.0 ([#10](https://github.com/AngelOnFira/cli/issues/10)) ([83f8817](https://github.com/AngelOnFira/cli/commit/83f8817c2a9b41d5b8ca9fb9bd8fd44a23505786))
* **main:** release 2.0.0-rc.1 ([#292](https://github.com/AngelOnFira/cli/issues/292)) ([3e378b3](https://github.com/AngelOnFira/cli/commit/3e378b39caca911c231fb1a1f045965278fa67fc))
* **main:** release 2.2.0 ([#11](https://github.com/AngelOnFira/cli/issues/11)) ([7485172](https://github.com/AngelOnFira/cli/commit/7485172362df1e6c95d783149e2859b2f9435109))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/AngelOnFira/cli/issues/253)) ([4b31887](https://github.com/AngelOnFira/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/AngelOnFira/cli/issues/234)) ([a9c1b29](https://github.com/AngelOnFira/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))
* Release ([fcbbb99](https://github.com/AngelOnFira/cli/commit/fcbbb9952c39ba5c93c622b146091f91cd3f27ee))
* Release ([76e486d](https://github.com/AngelOnFira/cli/commit/76e486d99e2a90b8066c8aec68ecefcff64e24ed))
* Release ([3d3f548](https://github.com/AngelOnFira/cli/commit/3d3f548f059def37e538301c2d587ac75148d074))
* Release ([04d004e](https://github.com/AngelOnFira/cli/commit/04d004efed9f6693c53eb3d0e5224476dadf8391))
* Release ([4951a00](https://github.com/AngelOnFira/cli/commit/4951a001465ceeeab209978b47053b03d6e32ec1))
* Release ([48b18ba](https://github.com/AngelOnFira/cli/commit/48b18baca3cf99cd80870bedeabe966bbb77b0cd))
* Release ([ab5e081](https://github.com/AngelOnFira/cli/commit/ab5e081f3f1eb4ac12c9b54f90953318e640afb6))
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
* release 1.3.2 ([50bd2be](https://github.com/AngelOnFira/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))
* release 1.3.3 ([aed2a03](https://github.com/AngelOnFira/cli/commit/aed2a039b01d27c96008f9fbc82c8b25251e2eb8))
* release 1.9.0 ([24eac90](https://github.com/AngelOnFira/cli/commit/24eac905528b1b138eb6754f2618e6d13ee9ddc3))
* release 2.0.0-rc.1 ([8882ad8](https://github.com/AngelOnFira/cli/commit/8882ad88d8b50422fabdb732ebb96aed3e7940c1))
* release 2.2.0 ([3990651](https://github.com/AngelOnFira/cli/commit/3990651501d7a47b5458d0bd2e91e96ace5d2a28))
* release 2.3.0 ([7d08e9e](https://github.com/AngelOnFira/cli/commit/7d08e9e67cc803f23585d7ecf94c1e3850d5b25a))
* remove experimental flag from run & exec commands ([#222](https://github.com/AngelOnFira/cli/issues/222)) ([c9cfae6](https://github.com/AngelOnFira/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/AngelOnFira/cli/issues/221)) ([0f47917](https://github.com/AngelOnFira/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* reorg package in prep for toolchain ([#276](https://github.com/AngelOnFira/cli/issues/276)) ([371ac16](https://github.com/AngelOnFira/cli/commit/371ac16c28e293b7f46f066c16e53dcf9b233eae))
* return backend project on bootstrap ([#284](https://github.com/AngelOnFira/cli/issues/284)) ([40ea899](https://github.com/AngelOnFira/cli/commit/40ea8998935afda712bba36cddf9b4297538c140))
* Update cargo dist ([#271](https://github.com/AngelOnFira/cli/issues/271)) ([c2355ae](https://github.com/AngelOnFira/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))
* update docker root user help link ([#214](https://github.com/AngelOnFira/cli/issues/214)) ([30fdc56](https://github.com/AngelOnFira/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))
* update opengb flag --path -&gt; --project ([#295](https://github.com/AngelOnFira/cli/issues/295)) ([6b5407e](https://github.com/AngelOnFira/cli/commit/6b5407e5f0b03cfcda22bb3d9640cac52aa82ecb))
* update posthog api key ([#263](https://github.com/AngelOnFira/cli/issues/263)) ([5f2ee58](https://github.com/AngelOnFira/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))
* update rivet-api ([#209](https://github.com/AngelOnFira/cli/issues/209)) ([b057d00](https://github.com/AngelOnFira/cli/commit/b057d00c0fc897b9f663a55c4aff8845685745c4))
* update sdk for env endpoint ([#285](https://github.com/AngelOnFira/cli/issues/285)) ([73c7120](https://github.com/AngelOnFira/cli/commit/73c7120e08da4881fa901bc153f7a77b31cf4262))

## [2.2.0](https://github.com/AngelOnFira/cli/compare/v1.9.0...v2.2.0) (2024-08-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/AngelOnFira/cli/issues/262)) ([6255e07](https://github.com/AngelOnFira/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))
* add ffi ([#291](https://github.com/AngelOnFira/cli/issues/291)) ([ffd015b](https://github.com/AngelOnFira/cli/commit/ffd015b3cc711bef5b533e3d002f6d79c38cd26e))
* add opengb db command passthrough ([#216](https://github.com/AngelOnFira/cli/issues/216)) ([7b78870](https://github.com/AngelOnFira/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/AngelOnFira/cli/issues/231)) ([2fc3021](https://github.com/AngelOnFira/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add settings file ([#278](https://github.com/AngelOnFira/cli/issues/278)) ([c004d05](https://github.com/AngelOnFira/cli/commit/c004d05e7255713a402b4d049b62615d1d14dfd2))
* add sidekick exec command ([#275](https://github.com/AngelOnFira/cli/issues/275)) ([3121f94](https://github.com/AngelOnFira/cli/commit/3121f940e4955e3cfa5772b8f98486f1ad323064))
* add sidekick show-term command ([#266](https://github.com/AngelOnFira/cli/issues/266)) ([5fc1c89](https://github.com/AngelOnFira/cli/commit/5fc1c89c3205f93411fd672934dbebf7bb1306ff))
* add support for sh and url db commands ([#217](https://github.com/AngelOnFira/cli/issues/217)) ([bbeeaba](https://github.com/AngelOnFira/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* allow configuring opengb config path ([#283](https://github.com/AngelOnFira/cli/issues/283)) ([aa8c212](https://github.com/AngelOnFira/cli/commit/aa8c21284855489e2548be803e0e68a12ceeaf3e))
* **cli:** add config command ([#296](https://github.com/AngelOnFira/cli/issues/296)) ([3787d0c](https://github.com/AngelOnFira/cli/commit/3787d0c49d013477942f424c23f684a2f0266f7c))
* get lobby and logs links in sidekick ([#235](https://github.com/AngelOnFira/cli/issues/235)) ([7c63efd](https://github.com/AngelOnFira/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))
* Implement OpenGB related commands ([#215](https://github.com/AngelOnFira/cli/issues/215)) ([ce57364](https://github.com/AngelOnFira/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))
* run opengb using docker by default ([#254](https://github.com/AngelOnFira/cli/issues/254)) ([598ce8d](https://github.com/AngelOnFira/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))
* **sidekick:** add backend dev command ([#274](https://github.com/AngelOnFira/cli/issues/274)) ([7547384](https://github.com/AngelOnFira/cli/commit/75473841fb6fa2780f50c5daea6f86daeca730f7))
* **sidekick:** add backend gen command ([#273](https://github.com/AngelOnFira/cli/issues/273)) ([2f1358e](https://github.com/AngelOnFira/cli/commit/2f1358e9f857fec27b3310ee6357ba369af583d2))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/AngelOnFira/cli/issues/226)) ([8a62d97](https://github.com/AngelOnFira/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/AngelOnFira/cli/issues/237)) ([64b3489](https://github.com/AngelOnFira/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/AngelOnFira/cli/issues/208)) ([214fe29](https://github.com/AngelOnFira/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* ci thing ([4549d7b](https://github.com/AngelOnFira/cli/commit/4549d7b3b0d4e9975e41cbbfbcf44fc01da07e24))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/AngelOnFira/cli/issues/224)) ([2f04ea3](https://github.com/AngelOnFira/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* disable hyper connection pooling ([#293](https://github.com/AngelOnFira/cli/issues/293)) ([bddfa6d](https://github.com/AngelOnFira/cli/commit/bddfa6dc996c42b476f9c8cc301a0cfd5981440f))
* **opengb:** one db per env ([#256](https://github.com/AngelOnFira/cli/issues/256)) ([a3c4e10](https://github.com/AngelOnFira/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* prevent asking user for terminal permissions ([#236](https://github.com/AngelOnFira/cli/issues/236)) ([a1a75d8](https://github.com/AngelOnFira/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/AngelOnFira/cli/issues/241)) ([72970c7](https://github.com/AngelOnFira/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/AngelOnFira/cli/issues/238)) ([e177ad4](https://github.com/AngelOnFira/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))
* revert regression with config-rs dependency ([#270](https://github.com/AngelOnFira/cli/issues/270)) ([ff3afa8](https://github.com/AngelOnFira/cli/commit/ff3afa8d12b1743b20365103b95fece74b4d1a39))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/AngelOnFira/cli/issues/220)) ([c1d33c5](https://github.com/AngelOnFira/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* show term args on linux ([#286](https://github.com/AngelOnFira/cli/issues/286)) ([6e94ab5](https://github.com/AngelOnFira/cli/commit/6e94ab594f320a78fd6a57f1f2b8d33d30723969))
* temp disable flakey macos test ([#212](https://github.com/AngelOnFira/cli/issues/212)) ([6d1ed9b](https://github.com/AngelOnFira/cli/commit/6d1ed9b87b7aa9cdc249a895c7d5ea47ef9d5be7))
* **test:** update region names ([#223](https://github.com/AngelOnFira/cli/issues/223)) ([c605561](https://github.com/AngelOnFira/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update cargo-release version ([450d25b](https://github.com/AngelOnFira/cli/commit/450d25b060da494a6ec44990b7575772e765db3e))
* update sdks for opengb ([#233](https://github.com/AngelOnFira/cli/issues/233)) ([7feb70b](https://github.com/AngelOnFira/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/AngelOnFira/cli/issues/210)) ([2b928df](https://github.com/AngelOnFira/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/AngelOnFira/cli/issues/229)) ([28d9d93](https://github.com/AngelOnFira/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Documentation

* release script instructions ([#248](https://github.com/AngelOnFira/cli/issues/248)) ([0d9edb3](https://github.com/AngelOnFira/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Code Refactoring

* move global config to meta config ([#279](https://github.com/AngelOnFira/cli/issues/279)) ([8ae2ed1](https://github.com/AngelOnFira/cli/commit/8ae2ed1ecf69eaac7f044cdddd270883906152a2))
* move shared functionality to toolchain ([#277](https://github.com/AngelOnFira/cli/issues/277)) ([d479186](https://github.com/AngelOnFira/cli/commit/d479186499db118a4aef2d54bd7cb64ab4581187))


### Continuous Integration

* and release please pr ([#244](https://github.com/AngelOnFira/cli/issues/244)) ([9862c5a](https://github.com/AngelOnFira/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* bring back release please ([#299](https://github.com/AngelOnFira/cli/issues/299)) ([730b694](https://github.com/AngelOnFira/cli/commit/730b69491dc8fd981a2248502b9c5780ce201a47))
* change label update to merged PRs ([#249](https://github.com/AngelOnFira/cli/issues/249)) ([06a938c](https://github.com/AngelOnFira/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))
* change release-please pr labels on release ([#247](https://github.com/AngelOnFira/cli/issues/247)) ([336f789](https://github.com/AngelOnFira/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/AngelOnFira/cli/issues/242)) ([f14b17e](https://github.com/AngelOnFira/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/AngelOnFira/cli/issues/243)) ([242e291](https://github.com/AngelOnFira/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* add back cli ([#288](https://github.com/AngelOnFira/cli/issues/288)) ([9ca98e4](https://github.com/AngelOnFira/cli/commit/9ca98e49c9536515dbd434e853f846f19d8af0e0))
* add back loading config ([#280](https://github.com/AngelOnFira/cli/issues/280)) ([06347e1](https://github.com/AngelOnFira/cli/commit/06347e168367e26cfcbe00264b2d8d4b9f23bbba))
* add deploy command ([#294](https://github.com/AngelOnFira/cli/issues/294)) ([e9bffdb](https://github.com/AngelOnFira/cli/commit/e9bffdbf163e70c39c5357d6f95321edfa791f91))
* add show_term task ([#289](https://github.com/AngelOnFira/cli/issues/289)) ([72b37e3](https://github.com/AngelOnFira/cli/commit/72b37e376d2d6db838c8b148d67c0089200fe8de))
* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/AngelOnFira/cli/issues/228)) ([a192e35](https://github.com/AngelOnFira/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* bump version ([71c215f](https://github.com/AngelOnFira/cli/commit/71c215febeebff27fad0cac1938e4e6663a12cdd))
* check system requirements ([#282](https://github.com/AngelOnFira/cli/issues/282)) ([7858080](https://github.com/AngelOnFira/cli/commit/7858080b62def9edfcf90fcc2d9632cae8816781))
* choose free port for opengb ([#281](https://github.com/AngelOnFira/cli/issues/281)) ([1601f55](https://github.com/AngelOnFira/cli/commit/1601f55e8729802940f1c09614c850b2ee846e34))
* default RunConfig will print to stdout ([#290](https://github.com/AngelOnFira/cli/issues/290)) ([aa7de1a](https://github.com/AngelOnFira/cli/commit/aa7de1a3c4d2eec9aa768ae45a5f1774bf3607ad))
* fmt sdk ([#251](https://github.com/AngelOnFira/cli/issues/251)) ([28eade7](https://github.com/AngelOnFira/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* format ([#297](https://github.com/AngelOnFira/cli/issues/297)) ([8ba0ec6](https://github.com/AngelOnFira/cli/commit/8ba0ec60adcd24187e8f74838533dace65aac618))
* **main:** release 1.2.0 ([555fec1](https://github.com/AngelOnFira/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))
* **main:** release 1.3.0 ([#246](https://github.com/AngelOnFira/cli/issues/246)) ([cfa546c](https://github.com/AngelOnFira/cli/commit/cfa546cf84ca89f7234220f030e9080a1197abb2))
* **main:** release 1.3.1 ([#250](https://github.com/AngelOnFira/cli/issues/250)) ([9a76764](https://github.com/AngelOnFira/cli/commit/9a76764f9e832736e036a3fb3bdb1b6efdad3b5b))
* **main:** release 1.3.2 ([#255](https://github.com/AngelOnFira/cli/issues/255)) ([e232c5f](https://github.com/AngelOnFira/cli/commit/e232c5f0d41b7c9d33d745c3737c008c29d45803))
* **main:** release 1.3.3 ([#257](https://github.com/AngelOnFira/cli/issues/257)) ([3d1c35f](https://github.com/AngelOnFira/cli/commit/3d1c35f3fe24d0efd7ff50e9ca2bbe063cb2da31))
* **main:** release 1.3.4 ([#272](https://github.com/AngelOnFira/cli/issues/272)) ([3d5d757](https://github.com/AngelOnFira/cli/commit/3d5d7574b6e628c968d4be91a1db9bc6e5ed9c97))
* **main:** release 1.9.0 ([#10](https://github.com/AngelOnFira/cli/issues/10)) ([83f8817](https://github.com/AngelOnFira/cli/commit/83f8817c2a9b41d5b8ca9fb9bd8fd44a23505786))
* **main:** release 2.0.0-rc.1 ([#292](https://github.com/AngelOnFira/cli/issues/292)) ([3e378b3](https://github.com/AngelOnFira/cli/commit/3e378b39caca911c231fb1a1f045965278fa67fc))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/AngelOnFira/cli/issues/253)) ([4b31887](https://github.com/AngelOnFira/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/AngelOnFira/cli/issues/234)) ([a9c1b29](https://github.com/AngelOnFira/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))
* Release ([76e486d](https://github.com/AngelOnFira/cli/commit/76e486d99e2a90b8066c8aec68ecefcff64e24ed))
* Release ([3d3f548](https://github.com/AngelOnFira/cli/commit/3d3f548f059def37e538301c2d587ac75148d074))
* Release ([04d004e](https://github.com/AngelOnFira/cli/commit/04d004efed9f6693c53eb3d0e5224476dadf8391))
* Release ([4951a00](https://github.com/AngelOnFira/cli/commit/4951a001465ceeeab209978b47053b03d6e32ec1))
* Release ([48b18ba](https://github.com/AngelOnFira/cli/commit/48b18baca3cf99cd80870bedeabe966bbb77b0cd))
* Release ([ab5e081](https://github.com/AngelOnFira/cli/commit/ab5e081f3f1eb4ac12c9b54f90953318e640afb6))
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
* release 1.3.2 ([50bd2be](https://github.com/AngelOnFira/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))
* release 1.3.3 ([aed2a03](https://github.com/AngelOnFira/cli/commit/aed2a039b01d27c96008f9fbc82c8b25251e2eb8))
* release 1.9.0 ([24eac90](https://github.com/AngelOnFira/cli/commit/24eac905528b1b138eb6754f2618e6d13ee9ddc3))
* release 2.0.0-rc.1 ([8882ad8](https://github.com/AngelOnFira/cli/commit/8882ad88d8b50422fabdb732ebb96aed3e7940c1))
* release 2.2.0 ([3990651](https://github.com/AngelOnFira/cli/commit/3990651501d7a47b5458d0bd2e91e96ace5d2a28))
* remove experimental flag from run & exec commands ([#222](https://github.com/AngelOnFira/cli/issues/222)) ([c9cfae6](https://github.com/AngelOnFira/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/AngelOnFira/cli/issues/221)) ([0f47917](https://github.com/AngelOnFira/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* reorg package in prep for toolchain ([#276](https://github.com/AngelOnFira/cli/issues/276)) ([371ac16](https://github.com/AngelOnFira/cli/commit/371ac16c28e293b7f46f066c16e53dcf9b233eae))
* return backend project on bootstrap ([#284](https://github.com/AngelOnFira/cli/issues/284)) ([40ea899](https://github.com/AngelOnFira/cli/commit/40ea8998935afda712bba36cddf9b4297538c140))
* Update cargo dist ([#271](https://github.com/AngelOnFira/cli/issues/271)) ([c2355ae](https://github.com/AngelOnFira/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))
* update docker root user help link ([#214](https://github.com/AngelOnFira/cli/issues/214)) ([30fdc56](https://github.com/AngelOnFira/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))
* update opengb flag --path -&gt; --project ([#295](https://github.com/AngelOnFira/cli/issues/295)) ([6b5407e](https://github.com/AngelOnFira/cli/commit/6b5407e5f0b03cfcda22bb3d9640cac52aa82ecb))
* update posthog api key ([#263](https://github.com/AngelOnFira/cli/issues/263)) ([5f2ee58](https://github.com/AngelOnFira/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))
* update rivet-api ([#209](https://github.com/AngelOnFira/cli/issues/209)) ([b057d00](https://github.com/AngelOnFira/cli/commit/b057d00c0fc897b9f663a55c4aff8845685745c4))
* update sdk for env endpoint ([#285](https://github.com/AngelOnFira/cli/issues/285)) ([73c7120](https://github.com/AngelOnFira/cli/commit/73c7120e08da4881fa901bc153f7a77b31cf4262))

## [1.9.0](https://github.com/AngelOnFira/cli/compare/v2.0.0-rc.1...v1.9.0) (2024-08-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/AngelOnFira/cli/issues/262)) ([6255e07](https://github.com/AngelOnFira/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))
* add ffi ([#291](https://github.com/AngelOnFira/cli/issues/291)) ([ffd015b](https://github.com/AngelOnFira/cli/commit/ffd015b3cc711bef5b533e3d002f6d79c38cd26e))
* add opengb db command passthrough ([#216](https://github.com/AngelOnFira/cli/issues/216)) ([7b78870](https://github.com/AngelOnFira/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/AngelOnFira/cli/issues/231)) ([2fc3021](https://github.com/AngelOnFira/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add settings file ([#278](https://github.com/AngelOnFira/cli/issues/278)) ([c004d05](https://github.com/AngelOnFira/cli/commit/c004d05e7255713a402b4d049b62615d1d14dfd2))
* add sidekick exec command ([#275](https://github.com/AngelOnFira/cli/issues/275)) ([3121f94](https://github.com/AngelOnFira/cli/commit/3121f940e4955e3cfa5772b8f98486f1ad323064))
* add sidekick show-term command ([#266](https://github.com/AngelOnFira/cli/issues/266)) ([5fc1c89](https://github.com/AngelOnFira/cli/commit/5fc1c89c3205f93411fd672934dbebf7bb1306ff))
* add support for sh and url db commands ([#217](https://github.com/AngelOnFira/cli/issues/217)) ([bbeeaba](https://github.com/AngelOnFira/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* allow configuring opengb config path ([#283](https://github.com/AngelOnFira/cli/issues/283)) ([aa8c212](https://github.com/AngelOnFira/cli/commit/aa8c21284855489e2548be803e0e68a12ceeaf3e))
* **cli:** add config command ([#296](https://github.com/AngelOnFira/cli/issues/296)) ([3787d0c](https://github.com/AngelOnFira/cli/commit/3787d0c49d013477942f424c23f684a2f0266f7c))
* get lobby and logs links in sidekick ([#235](https://github.com/AngelOnFira/cli/issues/235)) ([7c63efd](https://github.com/AngelOnFira/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))
* Implement OpenGB related commands ([#215](https://github.com/AngelOnFira/cli/issues/215)) ([ce57364](https://github.com/AngelOnFira/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))
* run opengb using docker by default ([#254](https://github.com/AngelOnFira/cli/issues/254)) ([598ce8d](https://github.com/AngelOnFira/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))
* **sidekick:** add backend dev command ([#274](https://github.com/AngelOnFira/cli/issues/274)) ([7547384](https://github.com/AngelOnFira/cli/commit/75473841fb6fa2780f50c5daea6f86daeca730f7))
* **sidekick:** add backend gen command ([#273](https://github.com/AngelOnFira/cli/issues/273)) ([2f1358e](https://github.com/AngelOnFira/cli/commit/2f1358e9f857fec27b3310ee6357ba369af583d2))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/AngelOnFira/cli/issues/226)) ([8a62d97](https://github.com/AngelOnFira/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/AngelOnFira/cli/issues/237)) ([64b3489](https://github.com/AngelOnFira/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/AngelOnFira/cli/issues/208)) ([214fe29](https://github.com/AngelOnFira/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/AngelOnFira/cli/issues/224)) ([2f04ea3](https://github.com/AngelOnFira/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* disable hyper connection pooling ([#293](https://github.com/AngelOnFira/cli/issues/293)) ([bddfa6d](https://github.com/AngelOnFira/cli/commit/bddfa6dc996c42b476f9c8cc301a0cfd5981440f))
* **opengb:** one db per env ([#256](https://github.com/AngelOnFira/cli/issues/256)) ([a3c4e10](https://github.com/AngelOnFira/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* prevent asking user for terminal permissions ([#236](https://github.com/AngelOnFira/cli/issues/236)) ([a1a75d8](https://github.com/AngelOnFira/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/AngelOnFira/cli/issues/241)) ([72970c7](https://github.com/AngelOnFira/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/AngelOnFira/cli/issues/238)) ([e177ad4](https://github.com/AngelOnFira/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))
* revert regression with config-rs dependency ([#270](https://github.com/AngelOnFira/cli/issues/270)) ([ff3afa8](https://github.com/AngelOnFira/cli/commit/ff3afa8d12b1743b20365103b95fece74b4d1a39))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/AngelOnFira/cli/issues/220)) ([c1d33c5](https://github.com/AngelOnFira/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* show term args on linux ([#286](https://github.com/AngelOnFira/cli/issues/286)) ([6e94ab5](https://github.com/AngelOnFira/cli/commit/6e94ab594f320a78fd6a57f1f2b8d33d30723969))
* temp disable flakey macos test ([#212](https://github.com/AngelOnFira/cli/issues/212)) ([6d1ed9b](https://github.com/AngelOnFira/cli/commit/6d1ed9b87b7aa9cdc249a895c7d5ea47ef9d5be7))
* **test:** update region names ([#223](https://github.com/AngelOnFira/cli/issues/223)) ([c605561](https://github.com/AngelOnFira/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update cargo-release version ([450d25b](https://github.com/AngelOnFira/cli/commit/450d25b060da494a6ec44990b7575772e765db3e))
* update sdks for opengb ([#233](https://github.com/AngelOnFira/cli/issues/233)) ([7feb70b](https://github.com/AngelOnFira/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/AngelOnFira/cli/issues/210)) ([2b928df](https://github.com/AngelOnFira/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/AngelOnFira/cli/issues/229)) ([28d9d93](https://github.com/AngelOnFira/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Documentation

* release script instructions ([#248](https://github.com/AngelOnFira/cli/issues/248)) ([0d9edb3](https://github.com/AngelOnFira/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Code Refactoring

* move global config to meta config ([#279](https://github.com/AngelOnFira/cli/issues/279)) ([8ae2ed1](https://github.com/AngelOnFira/cli/commit/8ae2ed1ecf69eaac7f044cdddd270883906152a2))
* move shared functionality to toolchain ([#277](https://github.com/AngelOnFira/cli/issues/277)) ([d479186](https://github.com/AngelOnFira/cli/commit/d479186499db118a4aef2d54bd7cb64ab4581187))


### Continuous Integration

* and release please pr ([#244](https://github.com/AngelOnFira/cli/issues/244)) ([9862c5a](https://github.com/AngelOnFira/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* bring back release please ([#299](https://github.com/AngelOnFira/cli/issues/299)) ([730b694](https://github.com/AngelOnFira/cli/commit/730b69491dc8fd981a2248502b9c5780ce201a47))
* change label update to merged PRs ([#249](https://github.com/AngelOnFira/cli/issues/249)) ([06a938c](https://github.com/AngelOnFira/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))
* change release-please pr labels on release ([#247](https://github.com/AngelOnFira/cli/issues/247)) ([336f789](https://github.com/AngelOnFira/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/AngelOnFira/cli/issues/242)) ([f14b17e](https://github.com/AngelOnFira/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/AngelOnFira/cli/issues/243)) ([242e291](https://github.com/AngelOnFira/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* add back cli ([#288](https://github.com/AngelOnFira/cli/issues/288)) ([9ca98e4](https://github.com/AngelOnFira/cli/commit/9ca98e49c9536515dbd434e853f846f19d8af0e0))
* add back loading config ([#280](https://github.com/AngelOnFira/cli/issues/280)) ([06347e1](https://github.com/AngelOnFira/cli/commit/06347e168367e26cfcbe00264b2d8d4b9f23bbba))
* add deploy command ([#294](https://github.com/AngelOnFira/cli/issues/294)) ([e9bffdb](https://github.com/AngelOnFira/cli/commit/e9bffdbf163e70c39c5357d6f95321edfa791f91))
* add show_term task ([#289](https://github.com/AngelOnFira/cli/issues/289)) ([72b37e3](https://github.com/AngelOnFira/cli/commit/72b37e376d2d6db838c8b148d67c0089200fe8de))
* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/AngelOnFira/cli/issues/228)) ([a192e35](https://github.com/AngelOnFira/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* bump version ([71c215f](https://github.com/AngelOnFira/cli/commit/71c215febeebff27fad0cac1938e4e6663a12cdd))
* check system requirements ([#282](https://github.com/AngelOnFira/cli/issues/282)) ([7858080](https://github.com/AngelOnFira/cli/commit/7858080b62def9edfcf90fcc2d9632cae8816781))
* choose free port for opengb ([#281](https://github.com/AngelOnFira/cli/issues/281)) ([1601f55](https://github.com/AngelOnFira/cli/commit/1601f55e8729802940f1c09614c850b2ee846e34))
* default RunConfig will print to stdout ([#290](https://github.com/AngelOnFira/cli/issues/290)) ([aa7de1a](https://github.com/AngelOnFira/cli/commit/aa7de1a3c4d2eec9aa768ae45a5f1774bf3607ad))
* fmt sdk ([#251](https://github.com/AngelOnFira/cli/issues/251)) ([28eade7](https://github.com/AngelOnFira/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* format ([#297](https://github.com/AngelOnFira/cli/issues/297)) ([8ba0ec6](https://github.com/AngelOnFira/cli/commit/8ba0ec60adcd24187e8f74838533dace65aac618))
* **main:** release 1.2.0 ([555fec1](https://github.com/AngelOnFira/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))
* **main:** release 1.3.0 ([#246](https://github.com/AngelOnFira/cli/issues/246)) ([cfa546c](https://github.com/AngelOnFira/cli/commit/cfa546cf84ca89f7234220f030e9080a1197abb2))
* **main:** release 1.3.1 ([#250](https://github.com/AngelOnFira/cli/issues/250)) ([9a76764](https://github.com/AngelOnFira/cli/commit/9a76764f9e832736e036a3fb3bdb1b6efdad3b5b))
* **main:** release 1.3.2 ([#255](https://github.com/AngelOnFira/cli/issues/255)) ([e232c5f](https://github.com/AngelOnFira/cli/commit/e232c5f0d41b7c9d33d745c3737c008c29d45803))
* **main:** release 1.3.3 ([#257](https://github.com/AngelOnFira/cli/issues/257)) ([3d1c35f](https://github.com/AngelOnFira/cli/commit/3d1c35f3fe24d0efd7ff50e9ca2bbe063cb2da31))
* **main:** release 1.3.4 ([#272](https://github.com/AngelOnFira/cli/issues/272)) ([3d5d757](https://github.com/AngelOnFira/cli/commit/3d5d7574b6e628c968d4be91a1db9bc6e5ed9c97))
* **main:** release 2.0.0-rc.1 ([#292](https://github.com/AngelOnFira/cli/issues/292)) ([3e378b3](https://github.com/AngelOnFira/cli/commit/3e378b39caca911c231fb1a1f045965278fa67fc))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/AngelOnFira/cli/issues/253)) ([4b31887](https://github.com/AngelOnFira/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/AngelOnFira/cli/issues/234)) ([a9c1b29](https://github.com/AngelOnFira/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))
* Release ([3d3f548](https://github.com/AngelOnFira/cli/commit/3d3f548f059def37e538301c2d587ac75148d074))
* Release ([04d004e](https://github.com/AngelOnFira/cli/commit/04d004efed9f6693c53eb3d0e5224476dadf8391))
* Release ([4951a00](https://github.com/AngelOnFira/cli/commit/4951a001465ceeeab209978b47053b03d6e32ec1))
* Release ([48b18ba](https://github.com/AngelOnFira/cli/commit/48b18baca3cf99cd80870bedeabe966bbb77b0cd))
* Release ([ab5e081](https://github.com/AngelOnFira/cli/commit/ab5e081f3f1eb4ac12c9b54f90953318e640afb6))
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
* release 1.3.2 ([50bd2be](https://github.com/AngelOnFira/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))
* release 1.3.3 ([aed2a03](https://github.com/AngelOnFira/cli/commit/aed2a039b01d27c96008f9fbc82c8b25251e2eb8))
* release 1.9.0 ([24eac90](https://github.com/AngelOnFira/cli/commit/24eac905528b1b138eb6754f2618e6d13ee9ddc3))
* release 2.0.0-rc.1 ([8882ad8](https://github.com/AngelOnFira/cli/commit/8882ad88d8b50422fabdb732ebb96aed3e7940c1))
* remove experimental flag from run & exec commands ([#222](https://github.com/AngelOnFira/cli/issues/222)) ([c9cfae6](https://github.com/AngelOnFira/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/AngelOnFira/cli/issues/221)) ([0f47917](https://github.com/AngelOnFira/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* reorg package in prep for toolchain ([#276](https://github.com/AngelOnFira/cli/issues/276)) ([371ac16](https://github.com/AngelOnFira/cli/commit/371ac16c28e293b7f46f066c16e53dcf9b233eae))
* return backend project on bootstrap ([#284](https://github.com/AngelOnFira/cli/issues/284)) ([40ea899](https://github.com/AngelOnFira/cli/commit/40ea8998935afda712bba36cddf9b4297538c140))
* Update cargo dist ([#271](https://github.com/AngelOnFira/cli/issues/271)) ([c2355ae](https://github.com/AngelOnFira/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))
* update docker root user help link ([#214](https://github.com/AngelOnFira/cli/issues/214)) ([30fdc56](https://github.com/AngelOnFira/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))
* update opengb flag --path -&gt; --project ([#295](https://github.com/AngelOnFira/cli/issues/295)) ([6b5407e](https://github.com/AngelOnFira/cli/commit/6b5407e5f0b03cfcda22bb3d9640cac52aa82ecb))
* update posthog api key ([#263](https://github.com/AngelOnFira/cli/issues/263)) ([5f2ee58](https://github.com/AngelOnFira/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))
* update rivet-api ([#209](https://github.com/AngelOnFira/cli/issues/209)) ([b057d00](https://github.com/AngelOnFira/cli/commit/b057d00c0fc897b9f663a55c4aff8845685745c4))
* update sdk for env endpoint ([#285](https://github.com/AngelOnFira/cli/issues/285)) ([73c7120](https://github.com/AngelOnFira/cli/commit/73c7120e08da4881fa901bc153f7a77b31cf4262))

## [2.0.0-rc.1](https://github.com/rivet-gg/cli/compare/v1.3.4...v2.0.0-rc.1) (2024-08-09)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/rivet-gg/cli/issues/262)) ([6255e07](https://github.com/rivet-gg/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))
* add ffi ([#291](https://github.com/rivet-gg/cli/issues/291)) ([ffd015b](https://github.com/rivet-gg/cli/commit/ffd015b3cc711bef5b533e3d002f6d79c38cd26e))
* add opengb db command passthrough ([#216](https://github.com/rivet-gg/cli/issues/216)) ([7b78870](https://github.com/rivet-gg/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/rivet-gg/cli/issues/231)) ([2fc3021](https://github.com/rivet-gg/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add settings file ([#278](https://github.com/rivet-gg/cli/issues/278)) ([c004d05](https://github.com/rivet-gg/cli/commit/c004d05e7255713a402b4d049b62615d1d14dfd2))
* add sidekick exec command ([#275](https://github.com/rivet-gg/cli/issues/275)) ([3121f94](https://github.com/rivet-gg/cli/commit/3121f940e4955e3cfa5772b8f98486f1ad323064))
* add sidekick show-term command ([#266](https://github.com/rivet-gg/cli/issues/266)) ([5fc1c89](https://github.com/rivet-gg/cli/commit/5fc1c89c3205f93411fd672934dbebf7bb1306ff))
* add support for sh and url db commands ([#217](https://github.com/rivet-gg/cli/issues/217)) ([bbeeaba](https://github.com/rivet-gg/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* allow configuring opengb config path ([#283](https://github.com/rivet-gg/cli/issues/283)) ([aa8c212](https://github.com/rivet-gg/cli/commit/aa8c21284855489e2548be803e0e68a12ceeaf3e))
* **cli:** add config command ([#296](https://github.com/rivet-gg/cli/issues/296)) ([3787d0c](https://github.com/rivet-gg/cli/commit/3787d0c49d013477942f424c23f684a2f0266f7c))
* get lobby and logs links in sidekick ([#235](https://github.com/rivet-gg/cli/issues/235)) ([7c63efd](https://github.com/rivet-gg/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))
* Implement OpenGB related commands ([#215](https://github.com/rivet-gg/cli/issues/215)) ([ce57364](https://github.com/rivet-gg/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))
* run opengb using docker by default ([#254](https://github.com/rivet-gg/cli/issues/254)) ([598ce8d](https://github.com/rivet-gg/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))
* **sidekick:** add backend dev command ([#274](https://github.com/rivet-gg/cli/issues/274)) ([7547384](https://github.com/rivet-gg/cli/commit/75473841fb6fa2780f50c5daea6f86daeca730f7))
* **sidekick:** add backend gen command ([#273](https://github.com/rivet-gg/cli/issues/273)) ([2f1358e](https://github.com/rivet-gg/cli/commit/2f1358e9f857fec27b3310ee6357ba369af583d2))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/rivet-gg/cli/issues/226)) ([8a62d97](https://github.com/rivet-gg/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/rivet-gg/cli/issues/237)) ([64b3489](https://github.com/rivet-gg/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* cdn.build_env not working ([#208](https://github.com/rivet-gg/cli/issues/208)) ([214fe29](https://github.com/rivet-gg/cli/commit/214fe297e612f6e88d06df7f57041be06f44949d))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/rivet-gg/cli/issues/224)) ([2f04ea3](https://github.com/rivet-gg/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* disable hyper connection pooling ([#293](https://github.com/rivet-gg/cli/issues/293)) ([bddfa6d](https://github.com/rivet-gg/cli/commit/bddfa6dc996c42b476f9c8cc301a0cfd5981440f))
* **opengb:** one db per env ([#256](https://github.com/rivet-gg/cli/issues/256)) ([a3c4e10](https://github.com/rivet-gg/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* prevent asking user for terminal permissions ([#236](https://github.com/rivet-gg/cli/issues/236)) ([a1a75d8](https://github.com/rivet-gg/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/rivet-gg/cli/issues/238)) ([e177ad4](https://github.com/rivet-gg/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))
* revert regression with config-rs dependency ([#270](https://github.com/rivet-gg/cli/issues/270)) ([ff3afa8](https://github.com/rivet-gg/cli/commit/ff3afa8d12b1743b20365103b95fece74b4d1a39))
* rivet exec does not work with --rivet-servers ([#220](https://github.com/rivet-gg/cli/issues/220)) ([c1d33c5](https://github.com/rivet-gg/cli/commit/c1d33c5e251d29edd270d9a84a05d37fe39357ee))
* show term args on linux ([#286](https://github.com/rivet-gg/cli/issues/286)) ([6e94ab5](https://github.com/rivet-gg/cli/commit/6e94ab594f320a78fd6a57f1f2b8d33d30723969))
* temp disable flakey macos test ([#212](https://github.com/rivet-gg/cli/issues/212)) ([6d1ed9b](https://github.com/rivet-gg/cli/commit/6d1ed9b87b7aa9cdc249a895c7d5ea47ef9d5be7))
* **test:** update region names ([#223](https://github.com/rivet-gg/cli/issues/223)) ([c605561](https://github.com/rivet-gg/cli/commit/c605561bdf4b4206b0f367f7dc5e716f1f0f5f76))
* update cargo-release version ([450d25b](https://github.com/rivet-gg/cli/commit/450d25b060da494a6ec44990b7575772e765db3e))
* update sdks for opengb ([#233](https://github.com/rivet-gg/cli/issues/233)) ([7feb70b](https://github.com/rivet-gg/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* update sentry issue url ([#210](https://github.com/rivet-gg/cli/issues/210)) ([2b928df](https://github.com/rivet-gg/cli/commit/2b928dfc38f18e7f33865bf8c76614c88c8ce384))
* **upload:** increase upload buffer size ([#229](https://github.com/rivet-gg/cli/issues/229)) ([28d9d93](https://github.com/rivet-gg/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Documentation

* release script instructions ([#248](https://github.com/rivet-gg/cli/issues/248)) ([0d9edb3](https://github.com/rivet-gg/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Code Refactoring

* move global config to meta config ([#279](https://github.com/rivet-gg/cli/issues/279)) ([8ae2ed1](https://github.com/rivet-gg/cli/commit/8ae2ed1ecf69eaac7f044cdddd270883906152a2))
* move shared functionality to toolchain ([#277](https://github.com/rivet-gg/cli/issues/277)) ([d479186](https://github.com/rivet-gg/cli/commit/d479186499db118a4aef2d54bd7cb64ab4581187))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* bring back release please ([#299](https://github.com/rivet-gg/cli/issues/299)) ([730b694](https://github.com/rivet-gg/cli/commit/730b69491dc8fd981a2248502b9c5780ce201a47))
* change label update to merged PRs ([#249](https://github.com/rivet-gg/cli/issues/249)) ([06a938c](https://github.com/rivet-gg/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))
* change release-please pr labels on release ([#247](https://github.com/rivet-gg/cli/issues/247)) ([336f789](https://github.com/rivet-gg/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* add back cli ([#288](https://github.com/rivet-gg/cli/issues/288)) ([9ca98e4](https://github.com/rivet-gg/cli/commit/9ca98e49c9536515dbd434e853f846f19d8af0e0))
* add back loading config ([#280](https://github.com/rivet-gg/cli/issues/280)) ([06347e1](https://github.com/rivet-gg/cli/commit/06347e168367e26cfcbe00264b2d8d4b9f23bbba))
* add deploy command ([#294](https://github.com/rivet-gg/cli/issues/294)) ([e9bffdb](https://github.com/rivet-gg/cli/commit/e9bffdbf163e70c39c5357d6f95321edfa791f91))
* add show_term task ([#289](https://github.com/rivet-gg/cli/issues/289)) ([72b37e3](https://github.com/rivet-gg/cli/commit/72b37e376d2d6db838c8b148d67c0089200fe8de))
* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/rivet-gg/cli/issues/228)) ([a192e35](https://github.com/rivet-gg/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* bump version ([71c215f](https://github.com/rivet-gg/cli/commit/71c215febeebff27fad0cac1938e4e6663a12cdd))
* check system requirements ([#282](https://github.com/rivet-gg/cli/issues/282)) ([7858080](https://github.com/rivet-gg/cli/commit/7858080b62def9edfcf90fcc2d9632cae8816781))
* choose free port for opengb ([#281](https://github.com/rivet-gg/cli/issues/281)) ([1601f55](https://github.com/rivet-gg/cli/commit/1601f55e8729802940f1c09614c850b2ee846e34))
* default RunConfig will print to stdout ([#290](https://github.com/rivet-gg/cli/issues/290)) ([aa7de1a](https://github.com/rivet-gg/cli/commit/aa7de1a3c4d2eec9aa768ae45a5f1774bf3607ad))
* fmt sdk ([#251](https://github.com/rivet-gg/cli/issues/251)) ([28eade7](https://github.com/rivet-gg/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* format ([#297](https://github.com/rivet-gg/cli/issues/297)) ([8ba0ec6](https://github.com/rivet-gg/cli/commit/8ba0ec60adcd24187e8f74838533dace65aac618))
* **main:** release 1.2.0 ([555fec1](https://github.com/rivet-gg/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))
* **main:** release 1.3.0 ([#246](https://github.com/rivet-gg/cli/issues/246)) ([cfa546c](https://github.com/rivet-gg/cli/commit/cfa546cf84ca89f7234220f030e9080a1197abb2))
* **main:** release 1.3.1 ([#250](https://github.com/rivet-gg/cli/issues/250)) ([9a76764](https://github.com/rivet-gg/cli/commit/9a76764f9e832736e036a3fb3bdb1b6efdad3b5b))
* **main:** release 1.3.2 ([#255](https://github.com/rivet-gg/cli/issues/255)) ([e232c5f](https://github.com/rivet-gg/cli/commit/e232c5f0d41b7c9d33d745c3737c008c29d45803))
* **main:** release 1.3.3 ([#257](https://github.com/rivet-gg/cli/issues/257)) ([3d1c35f](https://github.com/rivet-gg/cli/commit/3d1c35f3fe24d0efd7ff50e9ca2bbe063cb2da31))
* **main:** release 1.3.4 ([#272](https://github.com/rivet-gg/cli/issues/272)) ([3d5d757](https://github.com/rivet-gg/cli/commit/3d5d7574b6e628c968d4be91a1db9bc6e5ed9c97))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/rivet-gg/cli/issues/253)) ([4b31887](https://github.com/rivet-gg/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/rivet-gg/cli/issues/234)) ([a9c1b29](https://github.com/rivet-gg/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))
* Release ([04d004e](https://github.com/rivet-gg/cli/commit/04d004efed9f6693c53eb3d0e5224476dadf8391))
* Release ([4951a00](https://github.com/rivet-gg/cli/commit/4951a001465ceeeab209978b47053b03d6e32ec1))
* Release ([48b18ba](https://github.com/rivet-gg/cli/commit/48b18baca3cf99cd80870bedeabe966bbb77b0cd))
* Release ([ab5e081](https://github.com/rivet-gg/cli/commit/ab5e081f3f1eb4ac12c9b54f90953318e640afb6))
* Release ([b10bc24](https://github.com/rivet-gg/cli/commit/b10bc2414434bc1a93690ea2948feb52003f4bcd))
* Release ([6c40a0e](https://github.com/rivet-gg/cli/commit/6c40a0ea758ba9e845539d2bfc64fcd1bf35b7a9))
* Release ([e9ff7fc](https://github.com/rivet-gg/cli/commit/e9ff7fc971969963427e9dc50600c3177bab54f0))
* Release ([db9e6ab](https://github.com/rivet-gg/cli/commit/db9e6abef659db20857bc500f2236323477a3f19))
* Release ([bcd1d34](https://github.com/rivet-gg/cli/commit/bcd1d3471364ff7a634133b1c0fc8b39594cf636))
* Release ([d1ac3c6](https://github.com/rivet-gg/cli/commit/d1ac3c69ddf1bb15f284da3886d47bcb9a39e232))
* Release ([d0e2d6b](https://github.com/rivet-gg/cli/commit/d0e2d6b8557099b203ceb13264dcff95f79b7f05))
* Release ([bd657c8](https://github.com/rivet-gg/cli/commit/bd657c868f727032d87d3c392adc6ff8709b5272))
* Release ([1a6bec0](https://github.com/rivet-gg/cli/commit/1a6bec0254b5c6a3f4bda2d690d1129f83bfe702))
* Release ([68819c3](https://github.com/rivet-gg/cli/commit/68819c36514e7d0cc370e0fbb451a5e0f1fbf80e))
* Release ([2f23b00](https://github.com/rivet-gg/cli/commit/2f23b009dca0947ef58f619413bc04540b3078c6))
* Release ([d057971](https://github.com/rivet-gg/cli/commit/d057971c08a872fa23ddffc2c126840ac37a86dd))
* Release ([260ea03](https://github.com/rivet-gg/cli/commit/260ea0354c13e4c1edd0381ab093dacad624d5e5))
* Release ([9350c01](https://github.com/rivet-gg/cli/commit/9350c01ee444ce78570f846ffb6c2531cb7dbfe1))
* Release ([dfe5722](https://github.com/rivet-gg/cli/commit/dfe5722b1bea9d3ce483230f9c1c8db265fc9e5b))
* Release ([2e6508c](https://github.com/rivet-gg/cli/commit/2e6508ccdecfe204794022068c25f62e117c0f8f))
* Release ([dc96983](https://github.com/rivet-gg/cli/commit/dc969832d6049b27fc1e3b5a81729c59f6d3be17))
* release 1.3.2 ([50bd2be](https://github.com/rivet-gg/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))
* release 1.3.3 ([aed2a03](https://github.com/rivet-gg/cli/commit/aed2a039b01d27c96008f9fbc82c8b25251e2eb8))
* release 2.0.0-rc.1 ([8882ad8](https://github.com/rivet-gg/cli/commit/8882ad88d8b50422fabdb732ebb96aed3e7940c1))
* remove experimental flag from run & exec commands ([#222](https://github.com/rivet-gg/cli/issues/222)) ([c9cfae6](https://github.com/rivet-gg/cli/commit/c9cfae60971e465c7fde5e50be654daa49a5abfd))
* rename --rivet-servers to --server & --this-machine to --dev ([#221](https://github.com/rivet-gg/cli/issues/221)) ([0f47917](https://github.com/rivet-gg/cli/commit/0f479176a496c123035eb679c68d164fdbbdb354))
* reorg package in prep for toolchain ([#276](https://github.com/rivet-gg/cli/issues/276)) ([371ac16](https://github.com/rivet-gg/cli/commit/371ac16c28e293b7f46f066c16e53dcf9b233eae))
* return backend project on bootstrap ([#284](https://github.com/rivet-gg/cli/issues/284)) ([40ea899](https://github.com/rivet-gg/cli/commit/40ea8998935afda712bba36cddf9b4297538c140))
* Update cargo dist ([#271](https://github.com/rivet-gg/cli/issues/271)) ([c2355ae](https://github.com/rivet-gg/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))
* update docker root user help link ([#214](https://github.com/rivet-gg/cli/issues/214)) ([30fdc56](https://github.com/rivet-gg/cli/commit/30fdc56adb2c686496a7ded0406c2bb2255691d5))
* update opengb flag --path -&gt; --project ([#295](https://github.com/rivet-gg/cli/issues/295)) ([6b5407e](https://github.com/rivet-gg/cli/commit/6b5407e5f0b03cfcda22bb3d9640cac52aa82ecb))
* update posthog api key ([#263](https://github.com/rivet-gg/cli/issues/263)) ([5f2ee58](https://github.com/rivet-gg/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))
* update rivet-api ([#209](https://github.com/rivet-gg/cli/issues/209)) ([b057d00](https://github.com/rivet-gg/cli/commit/b057d00c0fc897b9f663a55c4aff8845685745c4))
* update sdk for env endpoint ([#285](https://github.com/rivet-gg/cli/issues/285)) ([73c7120](https://github.com/rivet-gg/cli/commit/73c7120e08da4881fa901bc153f7a77b31cf4262))

## [1.3.4](https://github.com/rivet-gg/cli/compare/v1.3.3...v1.3.4) (2024-07-11)


### Chores

* Update cargo dist ([#271](https://github.com/rivet-gg/cli/issues/271)) ([c2355ae](https://github.com/rivet-gg/cli/commit/c2355ae705fea6a1bd007412b7628758ccf1a78e))

## [1.3.3](https://github.com/rivet-gg/cli/compare/v1.3.2...v1.3.3) (2024-07-10)


### Features

* add --skip-migrate flag to opengb deploy ([#262](https://github.com/rivet-gg/cli/issues/262)) ([6255e07](https://github.com/rivet-gg/cli/commit/6255e0770dcea42d97b26357559fd0672257e675))


### Bug Fixes

* **opengb:** one db per env ([#256](https://github.com/rivet-gg/cli/issues/256)) ([a3c4e10](https://github.com/rivet-gg/cli/commit/a3c4e109d6ec316fc72c296b97efa4ef1aef11f9))
* revert regression with config-rs dependency ([#270](https://github.com/rivet-gg/cli/issues/270)) ([2dbb0b9](https://github.com/rivet-gg/cli/commit/2dbb0b9871e3bd8e342ab48f08dc8104677df1b7))


### Chores

* release 1.3.3 ([aae8e3b](https://github.com/rivet-gg/cli/commit/aae8e3bd2e58baa4fc75888b40ecf1a194077205))
* update posthog api key ([#263](https://github.com/rivet-gg/cli/issues/263)) ([5f2ee58](https://github.com/rivet-gg/cli/commit/5f2ee58a5b5bb204dd7d4178446bb1de304a9c0f))

## [1.3.2](https://github.com/rivet-gg/cli/compare/v1.3.1...v1.3.2) (2024-06-13)


### Features

* run opengb using docker by default ([#254](https://github.com/rivet-gg/cli/issues/254)) ([598ce8d](https://github.com/rivet-gg/cli/commit/598ce8da485ac035a834ab74bae33701b34af226))


### Chores

* release 1.3.2 ([50bd2be](https://github.com/rivet-gg/cli/commit/50bd2be003cf8561f5d444a6abd611e3f1078af5))

## [1.3.1](https://github.com/rivet-gg/cli/compare/v1.3.0...v1.3.1) (2024-06-10)


### Continuous Integration

* change label update to merged PRs ([#249](https://github.com/rivet-gg/cli/issues/249)) ([06a938c](https://github.com/rivet-gg/cli/commit/06a938cafedb2ed794358b0a9a370453fd2859dd))


### Chores

* fmt sdk ([#251](https://github.com/rivet-gg/cli/issues/251)) ([28eade7](https://github.com/rivet-gg/cli/commit/28eade7f45a032f08d3fb9c55ddc3b3eda3a1be5))
* **opengb:** migrate from backend.yaml -&gt; backend.json ([#253](https://github.com/rivet-gg/cli/issues/253)) ([4b31887](https://github.com/rivet-gg/cli/commit/4b31887bd166e71958155b8cc5a75bc8246b6248))
* **readme:** add note about openssl when building from source ([#234](https://github.com/rivet-gg/cli/issues/234)) ([a9c1b29](https://github.com/rivet-gg/cli/commit/a9c1b295a4819fb88f9be5a0e52780d5ab92bf27))

## [1.3.0](https://github.com/rivet-gg/cli/compare/v1.2.0...v1.3.0) (2024-05-29)


### Features

* get lobby and logs links in sidekick ([#235](https://github.com/rivet-gg/cli/issues/235)) ([7c63efd](https://github.com/rivet-gg/cli/commit/7c63efd86a2ca4a659b0df8c88b3764f904f2938))


### Bug Fixes

* prevent asking user for terminal permissions ([#236](https://github.com/rivet-gg/cli/issues/236)) ([a1a75d8](https://github.com/rivet-gg/cli/commit/a1a75d858a99ab3830ed14917a26e8f06f446c4f))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* reading byte-order marks on Windows ([#238](https://github.com/rivet-gg/cli/issues/238)) ([e177ad4](https://github.com/rivet-gg/cli/commit/e177ad4917945f6c99b8cd2f03c35bec3ba91941))


### Documentation

* release script instructions ([#248](https://github.com/rivet-gg/cli/issues/248)) ([0d9edb3](https://github.com/rivet-gg/cli/commit/0d9edb3737989709ad9d3221d13c5471f997e6e2))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* change release-please pr labels on release ([#247](https://github.com/rivet-gg/cli/issues/247)) ([336f789](https://github.com/rivet-gg/cli/commit/336f789b3909392fe92180ba75382f12d005c8de))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Bump the cargo group across 1 directory with 4 updates ([#228](https://github.com/rivet-gg/cli/issues/228)) ([a192e35](https://github.com/rivet-gg/cli/commit/a192e35aa5d5076be10d0f3b23836cfcc28ad1b0))
* **main:** release 1.2.0 ([555fec1](https://github.com/rivet-gg/cli/commit/555fec1a2e1dacc08bc03cbfaed733f146d06220))

## [1.2.0](https://github.com/rivet-gg/cli/compare/v1.1.0...v1.2.0) (2024-05-28)


### Features

* add opengb db command passthrough ([#216](https://github.com/rivet-gg/cli/issues/216)) ([7b78870](https://github.com/rivet-gg/cli/commit/7b788705687bd98387380e785614dbcc8c1190dd))
* add passthrough env var ([#231](https://github.com/rivet-gg/cli/issues/231)) ([2fc3021](https://github.com/rivet-gg/cli/commit/2fc30210e63e0230f88c9a7e04b54a66bb385fab))
* add support for sh and url db commands ([#217](https://github.com/rivet-gg/cli/issues/217)) ([bbeeaba](https://github.com/rivet-gg/cli/commit/bbeeaba7245839047c02f1f461869ab8c434e0ba))
* Implement OpenGB related commands ([#215](https://github.com/rivet-gg/cli/issues/215)) ([ce57364](https://github.com/rivet-gg/cli/commit/ce57364d138d80ea48902733df1b3f796d51cd05))


### Bug Fixes

* add concurrency constraint to generated github action ([#226](https://github.com/rivet-gg/cli/issues/226)) ([8a62d97](https://github.com/rivet-gg/cli/commit/8a62d97bcea701983df02502f801d4ca8f403eef))
* **backend:** check opengb and deno installation using which crate ([#237](https://github.com/rivet-gg/cli/issues/237)) ([64b3489](https://github.com/rivet-gg/cli/commit/64b3489f61206f58299cff59a5583c45b4663bac))
* **ci:** update ci script to use json-compact instead of json ([#224](https://github.com/rivet-gg/cli/issues/224)) ([2f04ea3](https://github.com/rivet-gg/cli/commit/2f04ea3c0639065a10f4b2ecbf4cfc2bf587f353))
* read_generated_manifest fn name ([#241](https://github.com/rivet-gg/cli/issues/241)) ([72970c7](https://github.com/rivet-gg/cli/commit/72970c7240f1dfa19a4fb75a9e009e8bde3799b5))
* update sdks for opengb ([#233](https://github.com/rivet-gg/cli/issues/233)) ([7feb70b](https://github.com/rivet-gg/cli/commit/7feb70b2056d96ac31a69102d8a172ad6c0e0905))
* **upload:** increase upload buffer size ([#229](https://github.com/rivet-gg/cli/issues/229)) ([28d9d93](https://github.com/rivet-gg/cli/commit/28d9d93a9e7d6df959fa2a731c7433febfbe47b0))


### Continuous Integration

* and release please pr ([#244](https://github.com/rivet-gg/cli/issues/244)) ([9862c5a](https://github.com/rivet-gg/cli/commit/9862c5ada4f935d64cc457d0ecd760a6d7d252b0))
* explicitly fmt check members ([#242](https://github.com/rivet-gg/cli/issues/242)) ([f14b17e](https://github.com/rivet-gg/cli/commit/f14b17ed23a33b34f738975489a92d431dae1c59))
* ignore failing e2e test ([#243](https://github.com/rivet-gg/cli/issues/243)) ([242e291](https://github.com/rivet-gg/cli/commit/242e291db0febec9f70632536d1da39c1293ff30))


### Chores

* Release ([b10bc24](https://github.com/rivet-gg/cli/commit/b10bc2414434bc1a93690ea2948feb52003f4bcd))

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
