
## [1.1.5](https://github.com/harilvfs/npltz/compare/v1.1.4...v1.1.5) - 2026-07-14


### 🐛 Bug Fixes


- [84e2c87](https://github.com/harilvfs/npltz/commit/84e2c87859c5c943810b91ca63c0a73a5e2ecb7e) Correct calendar data errors and off-by-one date conversion bug by @harilvfs

- [29d3ad5](https://github.com/harilvfs/npltz/commit/29d3ad5d1707baecfd83d8343c869e0b125da2ea) Replace calendar data with verified source and fix epoch bug by @harilvfs



## [1.1.4](https://github.com/harilvfs/npltz/compare/v1.1.3...v1.1.4) - 2026-07-01


### 🚀 Features


- [600b41c](https://github.com/harilvfs/npltz/commit/600b41ca3223b3c6fb953e838fecb2ac5f693cc9)  *(tui)* Add month jump (1-9, 0+1-3) and hover AD preview (#38) by @harilvfs in [#38](https://github.com/harilvfs/npltz/pull/38)

- [c905b28](https://github.com/harilvfs/npltz/commit/c905b28d3a407d9fa1fbfa1cfc75d4fb44c59c3b)  *(tui)* Make log to capture verbose output by @harilvfs



### 🐛 Bug Fixes


- [0b6a0dd](https://github.com/harilvfs/npltz/commit/0b6a0ddf3f47fc9c4f3ca1b18f109615ae2c79f1)  *(cli)* Replace println with writeln to prevent panic when stdout is not a terminal by @harilvfs



### ⚡ Performance


- [f356cff](https://github.com/harilvfs/npltz/commit/f356cff0e0f8a3098487a187d30510484bf6b442) Reduce redundant bs_to_ad calls and buffer log writes by @harilvfs



### ⚙️ Miscellaneous Tasks


- [e9c3153](https://github.com/harilvfs/npltz/commit/e9c3153b84e7b0c6c21c293329b6b7737778bc5d)  *(tui)* Make help popup width small by 10 by @harilvfs

- [e2cf557](https://github.com/harilvfs/npltz/commit/e2cf557e6c56fd312159c61cf196a4eaa27558a1)  *(preview)* Increase time for year overview by @harilvfs



## [1.1.3](https://github.com/harilvfs/npltz/compare/v1.1.2...v1.1.3) - 2026-06-29


### 🚀 Features


- [9f7673d](https://github.com/harilvfs/npltz/commit/9f7673d1979fbc8201c146a1652b2062d16026dd)  *(tui)* Add mouse support and increase popup size for `year overview` by @harilvfs



### 🐛 Bug Fixes


- [973a1dc](https://github.com/harilvfs/npltz/commit/973a1dc58c369b3b10a0bce3dcf5fbf0e8cd6369)  *(tui)* Handle infinite scroll and panic crash by @harilvfs



### 🚜 Refactor


- [741ff70](https://github.com/harilvfs/npltz/commit/741ff7021dbf80e7fe8b463e073de9b1221f71bf)  *(help)* Change style and add mouse support in help popup by @harilvfs

- [abde584](https://github.com/harilvfs/npltz/commit/abde58492248afe603d4ca007ba88c6cbb118754)  *(goto)* Improve layouting/work of goto popup by @harilvfs



### 🎨 Styling


- [fb08a33](https://github.com/harilvfs/npltz/commit/fb08a3303d90677d5f5934269f00438e4acd1afc)  *(tui)* Redone styling of theme selector popup by @harilvfs



### ⚙️ Miscellaneous Tasks


- [94651f6](https://github.com/harilvfs/npltz/commit/94651f66d3dfa94fa02484180a992fecd43f81c4)  *(app)* Update npltz description by @harilvfs

- [07afb59](https://github.com/harilvfs/npltz/commit/07afb597f1d644db1834b4af31314489c09bbb59)  *(docs)* Correct typo for warning info by @harilvfs

- [4bd5e30](https://github.com/harilvfs/npltz/commit/4bd5e30073ee706d1021fc2666a49c66c8db1033)  *(preview)* Add year-overview command to tape by @harilvfs

- [71f86c3](https://github.com/harilvfs/npltz/commit/71f86c35bb51b72795bf0c42f1974204a8757e77)  *(cargo)* Correct keywords metadata by @harilvfs



## [1.1.2](https://github.com/harilvfs/npltz/compare/v1.1.1...v1.1.2) - 2026-06-28


### 🚀 Features


- [7d5d03b](https://github.com/harilvfs/npltz/commit/7d5d03b4f64cad39e5bea55dd51128c0b7e39341) Add year overview, export/upcoming/week cmds and color-coded days (#33) by @harilvfs in [#33](https://github.com/harilvfs/npltz/pull/33)



### 🐛 Bug Fixes


- [fea96e3](https://github.com/harilvfs/npltz/commit/fea96e370d5fe0721f1778683753b4f5fb9c939e) Remove completions, man pages, and desktop file during uninstall by @harilvfs



### ⚙️ Miscellaneous Tasks


- [324bf2b](https://github.com/harilvfs/npltz/commit/324bf2be01ceea3291da550bf398445f4c8bf8ec)  *(docs)* Remove defined completions dir by @harilvfs



## [1.1.1](https://github.com/harilvfs/npltz/compare/v1.1.0...v1.1.1) - 2026-06-27


### 🚀 Features


- [0a26d66](https://github.com/harilvfs/npltz/commit/0a26d663915be9bab5899943476b188afa0bc276) Add cargo-binstall for update cmd by @harilvfs



## [1.1.0](https://github.com/harilvfs/npltz/compare/v1.0.7...v1.1.0) - 2026-06-27


### 🐛 Bug Fixes


- [f0fc4d8](https://github.com/harilvfs/npltz/commit/f0fc4d8e33c890719a6edd63cc86c428db5bc443) Correct binary download with latest changes by @harilvfs

- [050256c](https://github.com/harilvfs/npltz/commit/050256ced18c9e4bee39e67c610fdf2891c92db7) Correct workspace repository by @harilvfs



### ⚙️ Miscellaneous Tasks


- [a6a068d](https://github.com/harilvfs/npltz/commit/a6a068d3ded92dea29fc5e2b2435447696036186) For .desktop make target os linux by @harilvfs



## [1.0.7](https://github.com/harilvfs/npltz/compare/v1.0.6...v1.0.7) - 2026-06-27


### 🚀 Features


- [779f358](https://github.com/harilvfs/npltz/commit/779f3581497b96819c35fd2468c78be69b837936) Embed .desktop to setup by @harilvfs



### 🐛 Bug Fixes


- [ab3b8f4](https://github.com/harilvfs/npltz/commit/ab3b8f4411beeb9fb8816b604312ded860a296da) Add repository metadata by @harilvfs

- [e941833](https://github.com/harilvfs/npltz/commit/e9418331fb7ce5e903526d4a36d68a3c2916288a)  *(script)* Correct binary-assets by @harilvfs



### ⚙️ Miscellaneous Tasks


- [9f5432c](https://github.com/harilvfs/npltz/commit/9f5432ce782cb338b2d486e19b8e4bdc4f9b447b)  *(man-pages)* Match the layout same as xtask by @harilvfs



## [1.0.6](https://github.com/harilvfs/npltz/compare/v1.0.5...v1.0.6) - 2026-06-27


### 🚀 Features


- [9dc79c5](https://github.com/harilvfs/npltz/commit/9dc79c56bcd1dd3b32e6a2763cf236fe00d0047b) Add self-management cmds, cross-platform releases, and cargo-binstall support (#21) by @harilvfs in [#21](https://github.com/harilvfs/npltz/pull/21)

- [faf4b4f](https://github.com/harilvfs/npltz/commit/faf4b4fc6b466f513f18b878de93c99bca55f233)  *(script)* Add verify shasum by @harilvfs



### 🐛 Bug Fixes


- [b1c214a](https://github.com/harilvfs/npltz/commit/b1c214a70a5371db5150e81fc330168738c854af)  *(build)* Add rustls-tls feature for reqwest openssl by @harilvfs

- [b7bbe3b](https://github.com/harilvfs/npltz/commit/b7bbe3baf3d00efb8ccba1dc81df0289612e90c9)  *(macos)* Correct shasum arguments by @harilvfs



### ⚙️ Miscellaneous Tasks


- [122fb94](https://github.com/harilvfs/npltz/commit/122fb94345d92843a5076d745492dc1ff3f6883c)  *(tui)* Add `g` key to close goto popup by @harilvfs

- [d12b2e7](https://github.com/harilvfs/npltz/commit/d12b2e7242622d6e862d719d8c38d738d0164944)  *(release)* Remove macos-13 for build by @harilvfs

- [5f5e6c7](https://github.com/harilvfs/npltz/commit/5f5e6c77f95fa6d86169dc72c7130d5718c6f873)  *(deny)* Allow CDLA-Permissive-2.0 & ISC license by @harilvfs

- [c362411](https://github.com/harilvfs/npltz/commit/c3624110a0996f446b220c27b5b5ef4bd81e6b66)  *(ci)* If case for prerelease by @harilvfs



## [1.0.5](https://github.com/harilvfs/npltz/compare/v1.0.4...v1.0.5) - 2026-06-25


### 🚀 Features


- [bfdd9c1](https://github.com/harilvfs/npltz/commit/bfdd9c1d4ac04d0783b98c160a4a26945104e1e4)  *(tui)* Add detail to side panel by @harilvfs



### 🎨 Styling


- [ac9d83d](https://github.com/harilvfs/npltz/commit/ac9d83ded6ade2722360db11acc9eafca0e4e897)  *(tui)* Make theme popup bindings at tui border by @harilvfs



### ⚙️ Miscellaneous Tasks


- [682aa5b](https://github.com/harilvfs/npltz/commit/682aa5be396d0b04fcf8374054dab0ac565f0117)  *(docs)* Add info about available themes by @harilvfs



## [1.0.4](https://github.com/harilvfs/npltz/compare/v1.0.3...v1.0.4) - 2026-06-25


### 🚜 Refactor


- [91b7947](https://github.com/harilvfs/npltz/commit/91b794748b945ebacfccce348e5621cd04738530)  *(tui)* Update to new looks (#16) by @harilvfs in [#16](https://github.com/harilvfs/npltz/pull/16)



## [1.0.3](https://github.com/harilvfs/npltz/compare/v1.0.2...v1.0.3) - 2026-06-24


### 🚀 Features


- [58cef01](https://github.com/harilvfs/npltz/commit/58cef018c8731ae7c89cc72f3734cc3e596284b5)  *(tui)* Add mark when using 'goto' specific year day by @harilvfs



### 🐛 Bug Fixes


- [a79aad5](https://github.com/harilvfs/npltz/commit/a79aad5e2339360933722a51948b240e9226bfb9)  *(install)* Check version tag before download to prevent 404 by @harilvfs



### ⚙️ Miscellaneous Tasks


- [3ec4339](https://github.com/harilvfs/npltz/commit/3ec4339ccc09ce3355c2b627f3d53f32f231b562)  *(tui)* Increase popup size by @harilvfs

- [58b30d8](https://github.com/harilvfs/npltz/commit/58b30d8e57b934d97a069349d3dfbc900e0c7ee2)  *(lint)* Correct shfmt fix by @harilvfs



## [1.0.2](https://github.com/harilvfs/npltz/compare/v1.0.1...v1.0.2) - 2026-06-24


### 🚜 Refactor


- [9478f8a](https://github.com/harilvfs/npltz/commit/9478f8a3767b7b06dd6937d000335befb818dacd)  *(tui)* Make full-screen tui and remove status widget (#11) by @harilvfs in [#11](https://github.com/harilvfs/npltz/pull/11)



### ⚙️ Miscellaneous Tasks


- [1707342](https://github.com/harilvfs/npltz/commit/17073421ad84b18f7224c9ce424b4329a61ff811)  *(cli)* Simply to use BS/AD by @harilvfs

- [ba8c715](https://github.com/harilvfs/npltz/commit/ba8c715d4673a9c857d39855c10890fbbf715ba9)  *(tui)* Add dot separator by @harilvfs



## [1.0.1](https://github.com/harilvfs/npltz/compare/v1.0.0...v1.0.1) - 2026-06-23


### 🚀 Features


- [3c95e4a](https://github.com/harilvfs/npltz/commit/3c95e4ace3dde5d3ecab2c58c942d9bb5394ba8f)  *(tui)* Add warning if terminal size width is less than 80 by @harilvfs

- [5b1a100](https://github.com/harilvfs/npltz/commit/5b1a1002f36119fadda5c07f8e34b75c63923459)  *(tui)* Add help menu popup by @harilvfs



### 🚜 Refactor


- [7485459](https://github.com/harilvfs/npltz/commit/74854591c71c26bec37b7c8b85104c23145f3a2b)  *(script)* Making install script minimal by @harilvfs

- [ed8a576](https://github.com/harilvfs/npltz/commit/ed8a576f6ce3d941c95e9b1785dc377813cf53c4) Split large modules (#8) by @harilvfs in [#8](https://github.com/harilvfs/npltz/pull/8)



### 🎨 Styling


- [2c7ad40](https://github.com/harilvfs/npltz/commit/2c7ad401988f3f8ff7e5058be8b4616012f55e88)  *(tui)* Make border text alignment center by @harilvfs



### ⚙️ Miscellaneous Tasks


- [7e4be3e](https://github.com/harilvfs/npltz/commit/7e4be3e760fadccf9ba55ca157bc63a1cbc1d38d)  *(cargo)* Include calendar_data.json by @harilvfs

- [18add81](https://github.com/harilvfs/npltz/commit/18add81618b7fd5c6dcd7f05642a1986079661fb)  *(topic)* Add matching keywords as project by @harilvfs

- [014b090](https://github.com/harilvfs/npltz/commit/014b0902b1490279e343f6396fe8ffa3920320ab)  *(lints)* Correct toml formatting by @harilvfs

- [ce23784](https://github.com/harilvfs/npltz/commit/ce23784ea6b8ec65f76ae9ce9110f2704891e707)  *(metadata)* Correct mail and description by @harilvfs

- [a2f8e4c](https://github.com/harilvfs/npltz/commit/a2f8e4c794e98ec8a50add7c9ed38e851dbe93cb)  *(typo)* Correct project about heading typo by @harilvfs



## [1.0.0] - 2026-06-22


### 🚀 Features


- [8c082e3](https://github.com/harilvfs/npltz/commit/8c082e3f0239916148e92da8d88f2fcbcd2fc591) Initial of npltz by @harilvfs

- [cc7d4e1](https://github.com/harilvfs/npltz/commit/cc7d4e149d88899a4ae73b0137deb0c7d8c0b50b) Remove battery status & change weekend to regular by @harilvfs

- [bc230a4](https://github.com/harilvfs/npltz/commit/bc230a47f8c5875fbb83663c31d9e0585cceb684)  *(ci)* Add rust lint workflow by @harilvfs

- [94eb87a](https://github.com/harilvfs/npltz/commit/94eb87a5f72ec0cd838382bc623eb5b351b19102) Increase calander height width by @harilvfs

- [6c2158a](https://github.com/harilvfs/npltz/commit/6c2158a0b11e7e8bca1198ab6375fcf38adb6457)  *(rust)* Using rust nightly by @harilvfs

- [456fbc8](https://github.com/harilvfs/npltz/commit/456fbc84bed29db9c234973f049c48225301f7ef)  *(cli)* Add `npltz show` for getting the date on cli by @harilvfs

- [6238ab1](https://github.com/harilvfs/npltz/commit/6238ab135c966cec29e04cc796a8f6dbde4a1066) Add man-pages by @harilvfs

- [31c5bd8](https://github.com/harilvfs/npltz/commit/31c5bd8ae47f23d5102e1f5c664e5c8e7ec1dade)  *(script)* Add initial install script by @harilvfs

- [a658840](https://github.com/harilvfs/npltz/commit/a65884059434a2c9a484d5310e3e8a9e6988505c) Add numeric date format (YYYY/MM/DD) by @harilvfs



### 🚜 Refactor


- [63596f4](https://github.com/harilvfs/npltz/commit/63596f457b520aa7076ef572731da43509e9e88a) Add themes, config, logging, AD dates, and CLI command (#2) by @harilvfs in [#2](https://github.com/harilvfs/npltz/pull/2)



### ⚙️ Miscellaneous Tasks


- [e968f24](https://github.com/harilvfs/npltz/commit/e968f24011ebccebb1ee470eb031432001808802) Add license by @harilvfs

- [e9a60d0](https://github.com/harilvfs/npltz/commit/e9a60d0384ac083c540e7069c55a82b6960e80b9) Add profile release by @harilvfs

- [4fe03a0](https://github.com/harilvfs/npltz/commit/4fe03a0e9525793b7aa3a6e6aaf762a3a27603f8) Add editorconfig by @harilvfs

- [6edf31a](https://github.com/harilvfs/npltz/commit/6edf31a1fd48e00203653e786ec2536c04953179) Add typos checker by @harilvfs

- [d9b9fe4](https://github.com/harilvfs/npltz/commit/d9b9fe40f9ffe41668c0ace419716b2e92d7b760)  *(typo)* Ignore nd text by @harilvfs

- [2f16fd5](https://github.com/harilvfs/npltz/commit/2f16fd5fc42eb615ad61e2920ee738083c007e73) Remove clock from status widget by @harilvfs

- [8bcc872](https://github.com/harilvfs/npltz/commit/8bcc8725aae142e9cf7f378e91e9551db06a8647)  *(ci)* Add dependabot by @harilvfs

- [8abd039](https://github.com/harilvfs/npltz/commit/8abd039e92aa91ed14da8e63efd26ff237f19fe3) Make default feature to false by @harilvfs

- [ea7024e](https://github.com/harilvfs/npltz/commit/ea7024ee5ff6f0a2ddb3989ab2aeb06d37466043)  *(ci)* Add release & changelog workflow by @harilvfs

- [1f7a60e](https://github.com/harilvfs/npltz/commit/1f7a60e156f5158bb3755ce23d501c218a3ea2d9)  *(ci)* Add cargo crate publish workflow by @harilvfs

- [180e560](https://github.com/harilvfs/npltz/commit/180e560d2004787fbbf2ab1254c8624847144dee) Add security policy by @harilvfs

- [2e4c403](https://github.com/harilvfs/npltz/commit/2e4c4033dae320476653814f57d70a10ac222b08) Add coc ( code of conduct ) by @harilvfs

- [ff02bc0](https://github.com/harilvfs/npltz/commit/ff02bc0ab5ef7b2232f5a9786573e1ba6007a175)  *(docs)* Add issue/pr templates by @harilvfs

- [678358c](https://github.com/harilvfs/npltz/commit/678358c1ddaf90154b081c5c8819914ae30d03d6)  *(security)* Correct repo url by @harilvfs

- [946a2d1](https://github.com/harilvfs/npltz/commit/946a2d1a5c8f4e028748aca3511fb7fa94506c18)  *(template)* Correct some typos by @harilvfs

- [bab7f71](https://github.com/harilvfs/npltz/commit/bab7f71857901d25c485648034330f943ade03ef)  *(docs)* Add contributing guidelines by @harilvfs

- [e4d4942](https://github.com/harilvfs/npltz/commit/e4d4942d3f05a464f16fdfca74549132748ec0ac)  *(docs)* Add initial readme by @harilvfs

- [e22a501](https://github.com/harilvfs/npltz/commit/e22a501f9a3e80e0b2188d9ff52e2ea733366cc3)  *(source)* Add some metadata by @harilvfs

- [d1a5b44](https://github.com/harilvfs/npltz/commit/d1a5b44a030b157821dba7268faa222fc2c7b81c)  *(docs)* Correct use case by @harilvfs

- [4d9b74b](https://github.com/harilvfs/npltz/commit/4d9b74bbc674577758b45bcdab0fd225e2ed9ca8)  *(cli)* Add configuration for build and lints by @harilvfs

- [88856ff](https://github.com/harilvfs/npltz/commit/88856ff278695a2c4a14156c03836999aca4a27b)  *(ci)* Add release header for changelog by @harilvfs

- [83aad6b](https://github.com/harilvfs/npltz/commit/83aad6bfe76f4fe2665a06850a1acf25d4e4fb02) Add desktop file by @harilvfs

- [af7b11e](https://github.com/harilvfs/npltz/commit/af7b11e9080caffcb175d1b8e655f679734b3c0f)  *(config)* Update cliff config for consise on full changelog by @harilvfs

- [46cf207](https://github.com/harilvfs/npltz/commit/46cf207b3a4b1815e09969878c16bb984428d804)  *(ci)* Add install for taplo-cli and cargo-deny by @harilvfs

- [2bd3921](https://github.com/harilvfs/npltz/commit/2bd39216dcfafb3ed7538b3b59c469d6de8621af)  *(preview)* Add preview gen tape by @harilvfs

- [06ec510](https://github.com/harilvfs/npltz/commit/06ec5108dc44158faa65e99ca1d9a1092e381fd2)  *(docs)* Add credit for vhs by @harilvfs

- [1e13fff](https://github.com/harilvfs/npltz/commit/1e13fff3ad771a67479d9fc89e04d97a961db618)  *(docs)* Correct some typos by @harilvfs

- [251a60d](https://github.com/harilvfs/npltz/commit/251a60d8cb04fca21e9f771486e941fe29b220c6)  *(docs)* Using redirect install script url by @harilvfs

- [0c54624](https://github.com/harilvfs/npltz/commit/0c5462431a85fa0a45fb87dce3ecdd91e0b9605e)  *(ci)* Add missing setup for go by @harilvfs

- [57dbe66](https://github.com/harilvfs/npltz/commit/57dbe66c19ca14662d26db4646fd21ad6ea97dc0)  *(ci)* Add generating preview workflow by @harilvfs

- [ab59d5d](https://github.com/harilvfs/npltz/commit/ab59d5d36a93e0cd0e717ce7a30ef2c1d756e664) Remove man-pages dir by @harilvfs

- [515ac06](https://github.com/harilvfs/npltz/commit/515ac06f15dff557725c6a66e02b9e4c1c77901a)  *(ci)* Add github token by @harilvfs

- [32ed7b7](https://github.com/harilvfs/npltz/commit/32ed7b76bf0da374341a0b9e9732fdc2ad6cff1a)  *(typo)* Exclude changelog.md for typos check by @harilvfs

- [1602333](https://github.com/harilvfs/npltz/commit/1602333d7d3cb360943ec41aa208113dae604803)  *(preview)* Correct binary name by @harilvfs

- [d6149ab](https://github.com/harilvfs/npltz/commit/d6149abffad5a9283e932a1e05e5ebab7f5677b8)  *(docs)* Add preview raw url by @harilvfs

- [fb56531](https://github.com/harilvfs/npltz/commit/fb56531b57e713dd0d3ca46344d5fbdebaefc00e)  *(preview)* Increase preview work time by @harilvfs

- [1964c2b](https://github.com/harilvfs/npltz/commit/1964c2bd1adb37218ff18d277515353c8e6cd3f8)  *(docs)* Restructure notes and work by @harilvfs

- [df214a6](https://github.com/harilvfs/npltz/commit/df214a63411289d93f96e3732b6fb565281e4105)  *(preview)* Remove command line task by @harilvfs

- [d171abe](https://github.com/harilvfs/npltz/commit/d171abec6332b2315e5aca6321bb1d53284275ca)  *(docs)* Remove some note shenanigance by @harilvfs

- [abfdae5](https://github.com/harilvfs/npltz/commit/abfdae55e384a76a16aec6ce8474a5a26be7b539)  *(docs)* Add build from source info by @harilvfs

- [4f7d548](https://github.com/harilvfs/npltz/commit/4f7d548afa6f694c30f23ddc931e3dd6548604bd)  *(docs)* Don't pass blank issue report by @harilvfs

- [e16e83b](https://github.com/harilvfs/npltz/commit/e16e83bfe1a8e2a9555ab530792cc876971b779d)  *(docs)* Add guide for uninstallation by @harilvfs

- [d6fb5c3](https://github.com/harilvfs/npltz/commit/d6fb5c3430dc00adafeab18f947029fb1fdc6e7b)  *(ci)* Make prerelease to false by @harilvfs

- [b3cc333](https://github.com/harilvfs/npltz/commit/b3cc3335b10bc554e98b4e226bb39f22cfb10ab8)  *(docs)* Add discord link by @harilvfs

- [c9e6040](https://github.com/harilvfs/npltz/commit/c9e604023bce0eb8cef2bd14338ffcbe6908e579)  *(preview)* Increase ms to 200 by @harilvfs

- [a1a243a](https://github.com/harilvfs/npltz/commit/a1a243a6f13bd16ca5360bd8eb2896c9f2e0506f)  *(cargo)* Do not include manpages by @harilvfs

- [978024c](https://github.com/harilvfs/npltz/commit/978024c4556e76b8e094d232a59c14aa926987f9)  *(version)* Ready for initial release by @harilvfs

- [4f9330f](https://github.com/harilvfs/npltz/commit/4f9330fd29d62dbf952d4bbbdc6627f14eed6eda)  *(ci)* Fix manpage not found issue by @harilvfs



<!-- generated by git-cliff -->
