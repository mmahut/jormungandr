# Change Log

## [v0.3.3](https://github.com/input-output-hk/jormungandr/tree/v0.3.3) (2019-08-22)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.3.2...v0.3.3)

**Implemented enhancements:**

- CORS not enabled [\#708](https://github.com/input-output-hk/jormungandr/issues/708)
- add some logging on the leader event created block [\#717](https://github.com/input-output-hk/jormungandr/pull/717)
- Add CORS to REST endpoint [\#711](https://github.com/input-output-hk/jormungandr/pull/711)
- Add leadership logs fetching over REST API and JCLI [\#707](https://github.com/input-output-hk/jormungandr/pull/707)
- Finalize the loop for the new leadership code/event [\#701](https://github.com/input-output-hk/jormungandr/pull/701)
- allow setting the address prefix manually when creating the address [\#695](https://github.com/input-output-hk/jormungandr/pull/695)
- blockchain task: Process block announcements and blocks from network [\#693](https://github.com/input-output-hk/jormungandr/pull/693)
- plumbing changes for the leadership scheduling and blockchain validation [\#688](https://github.com/input-output-hk/jormungandr/pull/688)
- Update Leadership module to handle new blockchain API [\#685](https://github.com/input-output-hk/jormungandr/pull/685)
- Added block processing for new blockchain [\#684](https://github.com/input-output-hk/jormungandr/pull/684)

**Fixed bugs:**

- \[Jormungandr\] - \[mempool\] : Node "stops" producing blocks if garbage\_collection\_interval \< fragment\_ttl [\#705](https://github.com/input-output-hk/jormungandr/issues/705)
- Database error after abrupt node restart [\#676](https://github.com/input-output-hk/jormungandr/issues/676)
- make sure we don't  block the poll in the fragment pool too [\#706](https://github.com/input-output-hk/jormungandr/pull/706)
- Mempool and Leadership logs GC setting and fixes [\#703](https://github.com/input-output-hk/jormungandr/pull/703)
- Fix tests aborting on invalid logs [\#689](https://github.com/input-output-hk/jormungandr/pull/689)
- Added block processing for new blockchain [\#684](https://github.com/input-output-hk/jormungandr/pull/684)

**Closed issues:**

- serve the leader logs through the Rest API [\#698](https://github.com/input-output-hk/jormungandr/issues/698)

**Merged pull requests:**

- Testing scenario managing test flow [\#716](https://github.com/input-output-hk/jormungandr/pull/716)
- Testing scenario managing test flow [\#715](https://github.com/input-output-hk/jormungandr/pull/715)
- Rest async [\#714](https://github.com/input-output-hk/jormungandr/pull/714)
- \[Tests\] Fixed test\_genesis\_stake\_pool\_with\_utxo\_faucet\_starts\_successfully [\#713](https://github.com/input-output-hk/jormungandr/pull/713)
- Minor improvements on scenario testing [\#712](https://github.com/input-output-hk/jormungandr/pull/712)
- experiment with new interface for multi nodes testing [\#710](https://github.com/input-output-hk/jormungandr/pull/710)
- Futures rest [\#700](https://github.com/input-output-hk/jormungandr/pull/700)
- Deps cleanup [\#699](https://github.com/input-output-hk/jormungandr/pull/699)
- add link to the Node REST Api documentation [\#697](https://github.com/input-output-hk/jormungandr/pull/697)
- blockchain: Restore block propagation [\#696](https://github.com/input-output-hk/jormungandr/pull/696)
- Openapi validator 1st level [\#692](https://github.com/input-output-hk/jormungandr/pull/692)
- Use mainstream implementation for hex [\#691](https://github.com/input-output-hk/jormungandr/pull/691)
- Create OpenAPI docs [\#690](https://github.com/input-output-hk/jormungandr/pull/690)
- cosmetic-summing fixes [\#686](https://github.com/input-output-hk/jormungandr/pull/686)
- Update ROADMAP.md [\#669](https://github.com/input-output-hk/jormungandr/pull/669)

## [v0.3.2](https://github.com/input-output-hk/jormungandr/tree/v0.3.2) (2019-08-07)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.3.1...v0.3.2)

**Implemented enhancements:**

- version info in startup messages [\#606](https://github.com/input-output-hk/jormungandr/issues/606)
- Improve naming in config YAML [\#575](https://github.com/input-output-hk/jormungandr/issues/575)
- Extend logs with app version, epoch and slot\_id [\#679](https://github.com/input-output-hk/jormungandr/pull/679)
- Graceful handling of block0 in the future [\#661](https://github.com/input-output-hk/jormungandr/pull/661)
- Add stake pool getter to REST API [\#660](https://github.com/input-output-hk/jormungandr/pull/660)
- network: Perform protocol handshake [\#657](https://github.com/input-output-hk/jormungandr/pull/657)
- Add leadership management REST API [\#654](https://github.com/input-output-hk/jormungandr/pull/654)

**Fixed bugs:**

- Upgrade custom\_error to 1.7.1 [\#678](https://github.com/input-output-hk/jormungandr/pull/678)
- it seems that debug\_assertions feature was not doing what I expected [\#677](https://github.com/input-output-hk/jormungandr/pull/677)
- Graceful handling of block0 in the future [\#661](https://github.com/input-output-hk/jormungandr/pull/661)
- Poll gRPC client ready before sending any requests [\#656](https://github.com/input-output-hk/jormungandr/pull/656)
- Don't let one client connection terminate task [\#650](https://github.com/input-output-hk/jormungandr/pull/650)

**Closed issues:**

- Jcli: address info - wrong subcommand description [\#670](https://github.com/input-output-hk/jormungandr/issues/670)
- jormungandr install error [\#665](https://github.com/input-output-hk/jormungandr/issues/665)
- Jcli: cargo install failure due to custom\_error/1.7.1 crate [\#664](https://github.com/input-output-hk/jormungandr/issues/664)
- v0.3.1 Cannot Compile [\#648](https://github.com/input-output-hk/jormungandr/issues/648)
- Add fields useful for logs [\#645](https://github.com/input-output-hk/jormungandr/issues/645)

**Merged pull requests:**

- Remove Mjolnir [\#683](https://github.com/input-output-hk/jormungandr/pull/683)
- Remove unused deps from Jormungandr [\#682](https://github.com/input-output-hk/jormungandr/pull/682)
- internal code design: simple case state machine [\#681](https://github.com/input-output-hk/jormungandr/pull/681)
- Implement bootstrap for new blockchain API [\#675](https://github.com/input-output-hk/jormungandr/pull/675)
- New blockchain data representation [\#673](https://github.com/input-output-hk/jormungandr/pull/673)
- Jcli: Fix - address info, wrong subcommand description [\#671](https://github.com/input-output-hk/jormungandr/pull/671)
- \[Tests\] Set rust backtrace in e2e tests [\#668](https://github.com/input-output-hk/jormungandr/pull/668)
- add ROADMAP [\#666](https://github.com/input-output-hk/jormungandr/pull/666)
- Boxing problem with custom\_error [\#662](https://github.com/input-output-hk/jormungandr/pull/662)
- Network fixes [\#655](https://github.com/input-output-hk/jormungandr/pull/655)
- Protocol doc update [\#653](https://github.com/input-output-hk/jormungandr/pull/653)
- Fixed test\_correct\_utxo\_transaction\_replaces\_old\_utxo\_by\_node test [\#651](https://github.com/input-output-hk/jormungandr/pull/651)
- \[Tests\] Use fragment Id to track transaction status after post [\#647](https://github.com/input-output-hk/jormungandr/pull/647)
- update to latest chain-deps [\#646](https://github.com/input-output-hk/jormungandr/pull/646)
- Push/pull chain as complete blocks in one go [\#639](https://github.com/input-output-hk/jormungandr/pull/639)
- CircleCI: Streamline the workflow [\#625](https://github.com/input-output-hk/jormungandr/pull/625)

## [v0.3.1](https://github.com/input-output-hk/jormungandr/tree/v0.3.1) (2019-07-19)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.3.0...v0.3.1)

**Implemented enhancements:**

- Add node shutdown over REST [\#643](https://github.com/input-output-hk/jormungandr/pull/643)
- Add lastBlock info to node state REST [\#642](https://github.com/input-output-hk/jormungandr/pull/642)
- Add REST endpoint for getting node settings [\#634](https://github.com/input-output-hk/jormungandr/pull/634)

**Closed issues:**

- Node crash with Crit error when sending multiple transactions from the same Account, with the same Counter, in 2 consecutive slots [\#641](https://github.com/input-output-hk/jormungandr/issues/641)
- Transaction rejected because "Account with invalid signature" when sending multiple transactions from the same Account in the same slot \(with different Counter values\) [\#640](https://github.com/input-output-hk/jormungandr/issues/640)

**Merged pull requests:**

- Upgrade Slog to 2.5.1 [\#637](https://github.com/input-output-hk/jormungandr/pull/637)
- Simplify slot\_start\_time storage to seconds [\#636](https://github.com/input-output-hk/jormungandr/pull/636)
- Remove unused JCLI deps [\#635](https://github.com/input-output-hk/jormungandr/pull/635)
- Rename more fields in p2p config [\#632](https://github.com/input-output-hk/jormungandr/pull/632)

## [v0.3.0](https://github.com/input-output-hk/jormungandr/tree/v0.3.0) (2019-07-12)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.2.4...v0.3.0)

**Implemented enhancements:**

- Log Level consistency between CLI and config file [\#622](https://github.com/input-output-hk/jormungandr/issues/622)
- breaking change: move to fragment id to refer to utxo [\#633](https://github.com/input-output-hk/jormungandr/pull/633)
- Rework handling of inbound blocks and headers from the network [\#626](https://github.com/input-output-hk/jormungandr/pull/626)

**Fixed bugs:**

- Node crash if sending multiple transactions in the same slot [\#586](https://github.com/input-output-hk/jormungandr/issues/586)
- broken link, registering stake key guide [\#565](https://github.com/input-output-hk/jormungandr/issues/565)

**Closed issues:**

- add-output example missing value [\#628](https://github.com/input-output-hk/jormungandr/issues/628)
- gelf logging broken [\#621](https://github.com/input-output-hk/jormungandr/issues/621)
- Transactions are rejected when genesis file is re-encoded manually [\#610](https://github.com/input-output-hk/jormungandr/issues/610)

**Merged pull requests:**

- Rename Message to Fragment [\#631](https://github.com/input-output-hk/jormungandr/pull/631)
- Clean up unnecessary lifetimes in configuration\_builder test tools [\#630](https://github.com/input-output-hk/jormungandr/pull/630)
- Small doc updates [\#629](https://github.com/input-output-hk/jormungandr/pull/629)
- Clean up and extend log configuration [\#627](https://github.com/input-output-hk/jormungandr/pull/627)
- Process events in the client connection [\#620](https://github.com/input-output-hk/jormungandr/pull/620)
- Fix node crashing when multiple TXs for same account are in slot [\#619](https://github.com/input-output-hk/jormungandr/pull/619)
- make sure we use the latest stable available [\#616](https://github.com/input-output-hk/jormungandr/pull/616)
- move the address prefix to -lib and jcli [\#614](https://github.com/input-output-hk/jormungandr/pull/614)
- Fix localhost for BSD & OSX [\#613](https://github.com/input-output-hk/jormungandr/pull/613)
- scripts: bootstrap small fixes [\#611](https://github.com/input-output-hk/jormungandr/pull/611)
- \[Test\] node stops producing blocks test case [\#609](https://github.com/input-output-hk/jormungandr/pull/609)
- scripts: create-account-and-delegate POSIX-syntax [\#608](https://github.com/input-output-hk/jormungandr/pull/608)
- jormungandr: fix binary version release 0.2.3 -\> 0.2.4 [\#607](https://github.com/input-output-hk/jormungandr/pull/607)
- Add proper error reporting to JCLI REST commands [\#605](https://github.com/input-output-hk/jormungandr/pull/605)
- Pulling missing chain blocks from the network [\#601](https://github.com/input-output-hk/jormungandr/pull/601)
- use binaries by default and support building source too [\#594](https://github.com/input-output-hk/jormungandr/pull/594)
- Docker: use alpine base image and versioned releases [\#567](https://github.com/input-output-hk/jormungandr/pull/567)

## [v0.2.4](https://github.com/input-output-hk/jormungandr/tree/v0.2.4) (2019-07-04)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.2.3...v0.2.4)

**Implemented enhancements:**

- Improve bootstrap script to prevent a non-stake case to appear [\#604](https://github.com/input-output-hk/jormungandr/pull/604)
- Rest stake distribution [\#603](https://github.com/input-output-hk/jormungandr/pull/603)
- More graceful error handling in blockchain task [\#588](https://github.com/input-output-hk/jormungandr/pull/588)
- More logging improvements; add output to stdout [\#587](https://github.com/input-output-hk/jormungandr/pull/587)

**Fixed bugs:**

- block0 initial funds should accept multiple entries [\#579](https://github.com/input-output-hk/jormungandr/issues/579)
- jcli add-certificate does not take fees into account [\#499](https://github.com/input-output-hk/jormungandr/issues/499)

**Closed issues:**

- bootstrap script error [\#602](https://github.com/input-output-hk/jormungandr/issues/602)
- v0.2.3 Cannot Compile \(Experimental Alpine Docker\) [\#590](https://github.com/input-output-hk/jormungandr/issues/590)
- cargo install compile fail [\#581](https://github.com/input-output-hk/jormungandr/issues/581)
- add-output results in invalid internal encoding error [\#577](https://github.com/input-output-hk/jormungandr/issues/577)
- Documentation : empty faucet warning \(?\) [\#564](https://github.com/input-output-hk/jormungandr/issues/564)
- bootstrap error in genesis\_praos, genesis file corrupted [\#562](https://github.com/input-output-hk/jormungandr/issues/562)
- Documentation: Improve the documentation related to Staking&Delegation  [\#530](https://github.com/input-output-hk/jormungandr/issues/530)
- documentation: Add a consolidated/consistent/easier way for starting the node [\#515](https://github.com/input-output-hk/jormungandr/issues/515)
- documentation: improve the documentation for 'jcli rest v0 account get' [\#484](https://github.com/input-output-hk/jormungandr/issues/484)

**Merged pull requests:**

- Finalize Divide and Reuse [\#600](https://github.com/input-output-hk/jormungandr/pull/600)
- More changes in the jormungandr-lib API [\#599](https://github.com/input-output-hk/jormungandr/pull/599)
- take into account the certificate when computing the fees [\#598](https://github.com/input-output-hk/jormungandr/pull/598)
- Use certificate from jormungandr lib [\#597](https://github.com/input-output-hk/jormungandr/pull/597)
- Improve testing of the Block0Configuration [\#596](https://github.com/input-output-hk/jormungandr/pull/596)
- updated delegation script  [\#595](https://github.com/input-output-hk/jormungandr/pull/595)
- REST refactoring and simplification [\#593](https://github.com/input-output-hk/jormungandr/pull/593)
- Test Improvement. Implement dumping logs on console on jormungandr error [\#592](https://github.com/input-output-hk/jormungandr/pull/592)
- Update chain-deps [\#585](https://github.com/input-output-hk/jormungandr/pull/585)
- experiment with changelog generation [\#584](https://github.com/input-output-hk/jormungandr/pull/584)
- Fix multi output funds support in genesis yaml file [\#583](https://github.com/input-output-hk/jormungandr/pull/583)
- Add notes on protobuf and C compilers to the install steps [\#582](https://github.com/input-output-hk/jormungandr/pull/582)
- Improve GELF logging backend configuration [\#574](https://github.com/input-output-hk/jormungandr/pull/574)
- bootstrap script ported to Windows Powershell [\#572](https://github.com/input-output-hk/jormungandr/pull/572)
- Update blockchain.md [\#568](https://github.com/input-output-hk/jormungandr/pull/568)
- Pull missing blocks [\#554](https://github.com/input-output-hk/jormungandr/pull/554)
- Add example for account input and links to scripts [\#487](https://github.com/input-output-hk/jormungandr/pull/487)

## [v0.2.3](https://github.com/input-output-hk/jormungandr/tree/v0.2.3) (2019-06-23)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.2.2...v0.2.3)

**Merged pull requests:**

- Move JCLI's genesis into jormungandr-lib [\#560](https://github.com/input-output-hk/jormungandr/pull/560)
- Proposal to replace ENTRYPOINT with CMD in Dockerfile [\#559](https://github.com/input-output-hk/jormungandr/pull/559)

## [v0.2.2](https://github.com/input-output-hk/jormungandr/tree/v0.2.2) (2019-06-21)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.2.1...v0.2.2)

**Closed issues:**

- jcli 0.2.1 \[jcli key generate --type\]  [\#501](https://github.com/input-output-hk/jormungandr/issues/501)
- REST account API: The delegation field format should be improved [\#491](https://github.com/input-output-hk/jormungandr/issues/491)
- gelf logging support for slog [\#447](https://github.com/input-output-hk/jormungandr/issues/447)

**Merged pull requests:**

- mark gelf as optional feature [\#557](https://github.com/input-output-hk/jormungandr/pull/557)
- Fix incorrect PATH setting [\#555](https://github.com/input-output-hk/jormungandr/pull/555)
- Update introduction.md [\#552](https://github.com/input-output-hk/jormungandr/pull/552)
- Update delegating\_stake.md [\#550](https://github.com/input-output-hk/jormungandr/pull/550)
- add more documentation [\#547](https://github.com/input-output-hk/jormungandr/pull/547)
- UTxO Info as a common interface between jcli, jormungandr and the tests [\#543](https://github.com/input-output-hk/jormungandr/pull/543)
- move to chain-deps [\#542](https://github.com/input-output-hk/jormungandr/pull/542)
- ignore unstable test [\#539](https://github.com/input-output-hk/jormungandr/pull/539)
- remove non needed reference [\#538](https://github.com/input-output-hk/jormungandr/pull/538)
- fix the path to the default genesis block in the documentation [\#537](https://github.com/input-output-hk/jormungandr/pull/537)
- add hex-crate to replace cardano::util::hex [\#534](https://github.com/input-output-hk/jormungandr/pull/534)
- account state for both jormungandr, jcli and tests [\#532](https://github.com/input-output-hk/jormungandr/pull/532)
- Revert "New corner cases for transaction module" [\#531](https://github.com/input-output-hk/jormungandr/pull/531)
- Add script for create account and delegating with it [\#529](https://github.com/input-output-hk/jormungandr/pull/529)
- provide more details on the error if available [\#528](https://github.com/input-output-hk/jormungandr/pull/528)
- Move genesis.yaml initial state to single list [\#527](https://github.com/input-output-hk/jormungandr/pull/527)
- Fix for soak test after \#522 [\#526](https://github.com/input-output-hk/jormungandr/pull/526)
- Documentation update [\#524](https://github.com/input-output-hk/jormungandr/pull/524)
- Unify jormungandr API Types to allow better reusability [\#522](https://github.com/input-output-hk/jormungandr/pull/522)
- Update introduction.md [\#521](https://github.com/input-output-hk/jormungandr/pull/521)
- bootstrap: fix printed example command [\#519](https://github.com/input-output-hk/jormungandr/pull/519)
- Changing function declaration to POSIX-syntax [\#518](https://github.com/input-output-hk/jormungandr/pull/518)
- Increase logger async buffer from 128 to 1024 entries [\#509](https://github.com/input-output-hk/jormungandr/pull/509)
- removing dup getopts d [\#504](https://github.com/input-output-hk/jormungandr/pull/504)
- jcli: fix key type in help message ed25510bip32 -\> ed25519bip32 [\#502](https://github.com/input-output-hk/jormungandr/pull/502)
- adding a few more flags/options to the bootstrap script [\#498](https://github.com/input-output-hk/jormungandr/pull/498)
- Support GELF logging output. [\#497](https://github.com/input-output-hk/jormungandr/pull/497)
- Remove todo in quickstart section about P2P [\#486](https://github.com/input-output-hk/jormungandr/pull/486)
- Test stability fix for transaction test cases [\#483](https://github.com/input-output-hk/jormungandr/pull/483)

## [v0.2.1](https://github.com/input-output-hk/jormungandr/tree/v0.2.1) (2019-06-15)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.2.0...v0.2.1)

**Fixed bugs:**

- output and format of logger defined in config yaml is ignored [\#494](https://github.com/input-output-hk/jormungandr/issues/494)
- jcli transaction id not changing when adding certificate [\#475](https://github.com/input-output-hk/jormungandr/issues/475)

**Merged pull requests:**

- JCLI fixes and Ed25519 sk unification [\#500](https://github.com/input-output-hk/jormungandr/pull/500)
- Stop ignoring config.yaml logger settings [\#495](https://github.com/input-output-hk/jormungandr/pull/495)
- Extend faucet script [\#492](https://github.com/input-output-hk/jormungandr/pull/492)
- Poll the gRPC client for readiness [\#489](https://github.com/input-output-hk/jormungandr/pull/489)
- replace invalid TransactionId [\#488](https://github.com/input-output-hk/jormungandr/pull/488)
- Fix README typo: public\_access-\>public\_address [\#482](https://github.com/input-output-hk/jormungandr/pull/482)
- add option to disable colours, fix find for deleting tmp files [\#480](https://github.com/input-output-hk/jormungandr/pull/480)
- Stake key certificate does not exist anymore [\#461](https://github.com/input-output-hk/jormungandr/pull/461)

## [v0.2.0](https://github.com/input-output-hk/jormungandr/tree/v0.2.0) (2019-06-13)
[Full Changelog](https://github.com/input-output-hk/jormungandr/compare/v0.1.0...v0.2.0)

**Fixed bugs:**

- Error when verifying transaction with fee [\#449](https://github.com/input-output-hk/jormungandr/issues/449)
- Can't read secret key for creating witness with jcli [\#448](https://github.com/input-output-hk/jormungandr/issues/448)

**Closed issues:**

- jcli: remove 'allow\_account\_creation' from the config generated with 'jcli genesis init \> genesis.yaml'  [\#471](https://github.com/input-output-hk/jormungandr/issues/471)
- Invalid Node secret file: bft.signing\_key: Invalid prefix: expected ed25519e\_sk but was ed25519\_sk at line 6 column 16 [\#460](https://github.com/input-output-hk/jormungandr/issues/460)
- remove the shell ansi colours from scripts/stakepool-single-node-test [\#441](https://github.com/input-output-hk/jormungandr/issues/441)

**Merged pull requests:**

- jcli: 'remove allow\_account\_creation' from 'jcli genesis init' [\#477](https://github.com/input-output-hk/jormungandr/pull/477)
- Mention add-certificate in stake delegation [\#476](https://github.com/input-output-hk/jormungandr/pull/476)
- Last minute updates [\#474](https://github.com/input-output-hk/jormungandr/pull/474)
- Update to API changes in network-grpc [\#468](https://github.com/input-output-hk/jormungandr/pull/468)
- enable fixing the builds under nix, by making the jormungandr path configurable [\#464](https://github.com/input-output-hk/jormungandr/pull/464)
- Bft secretkey cleanup [\#462](https://github.com/input-output-hk/jormungandr/pull/462)
- Add a full transaction creation and sending example to the docs [\#459](https://github.com/input-output-hk/jormungandr/pull/459)
- Fix error when the current epoch is nearly finished and no block have been created [\#458](https://github.com/input-output-hk/jormungandr/pull/458)
- update cardano-deps and fix issue with fee check [\#455](https://github.com/input-output-hk/jormungandr/pull/455)
- Trim strings read with JCLI read\_line [\#454](https://github.com/input-output-hk/jormungandr/pull/454)
- Adding a utility that'll convert a between different addresses [\#453](https://github.com/input-output-hk/jormungandr/pull/453)
- Added scripts for bft node and send transaction [\#445](https://github.com/input-output-hk/jormungandr/pull/445)
- Update network-grpc, ported to tower-hyper [\#444](https://github.com/input-output-hk/jormungandr/pull/444)
- new test case for genesis utxo stake pool [\#443](https://github.com/input-output-hk/jormungandr/pull/443)
- improve jcli account-id parsing  [\#442](https://github.com/input-output-hk/jormungandr/pull/442)
- remove stake key and related certificate, fix network compilation [\#440](https://github.com/input-output-hk/jormungandr/pull/440)



\* *This Change Log was automatically generated by [github_changelog_generator](https://github.com/skywinder/Github-Changelog-Generator)*
