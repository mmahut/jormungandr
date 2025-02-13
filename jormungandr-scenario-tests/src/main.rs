#[macro_use]
extern crate jormungandr_scenario_tests;

use jormungandr_scenario_tests::{prepare_command, style, Context, Seed};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct CommandArgs {
    /// path or name of the jormungandr node to test
    #[structopt(long = "jormungandr", default_value = "jormungandr")]
    jormungandr: PathBuf,

    /// path or name of the jcli to test
    #[structopt(long = "jcli", default_value = "jcli")]
    jcli: PathBuf,

    /// set a directory in which the tests will be run, allowing every details
    /// to be save persistently. By default it will create temporary directories
    /// and will delete the files and documents
    #[structopt(long = "root-dir")]
    testing_directory: Option<PathBuf>,

    /// document the different scenario, creating markdown and dot (graphviz) files
    /// describing the tests initial setup
    ///
    /// The files are created within the `--root-dir`  directory.
    #[structopt(long = "document")]
    generate_documentation: bool,

    /// to set if to reproduce an existing test
    #[structopt(long = "seed")]
    seed: Option<Seed>,
}

fn main() {
    let command_args = CommandArgs::from_args();

    let jormungandr = prepare_command(command_args.jormungandr);
    let jcli = prepare_command(command_args.jcli);
    let seed = command_args
        .seed
        .unwrap_or_else(|| Seed::generate(rand::rngs::OsRng::new().unwrap()));
    let testing_directory = command_args.testing_directory;
    let generate_documentation = command_args.generate_documentation;

    let mut context = Context::new(
        seed,
        jormungandr,
        jcli,
        testing_directory,
        generate_documentation,
    );

    introduction(&context);

    scenario_1(context.derive());
}

fn introduction<R: rand_core::RngCore>(context: &Context<R>) {
    println!(
        r###"
        ---_ ......._-_--.
       (|\ /      / /| \  \               _  ___  ____  __  __ _   _ _   _  ____    _    _   _ ____  ____
       /  /     .'  -=-'   `.            | |/ _ \|  _ \|  \/  | | | | \ | |/ ___|  / \  | \ | |  _ \|  _ \
      /  /    .'             )        _  | | | | | |_) | |\/| | | | |  \| | |  _  / _ \ |  \| | | | | |_) |
    _/  /   .'        _.)   /        | |_| | |_| |  _ <| |  | | |_| | |\  | |_| |/ ___ \| |\  | |_| |  _ <
   /   o  o       _.-' /  .'          \___/ \___/|_| \_\_|  |_|\___/|_| \_|\____/_/   \_\_| \_|____/|_| \_\
   \          _.-'    / .'#|
    \______.-'//    .'.' \#|         SCENARIO TEST SUITE
     \|  \ | //   .'.' _ |#|
      `   \|//  .'.'_._._|#|
       .  .// .'.' | _._ \#|
       \`-|\_/ /    \ _._ \#\
        `/'\__/      \ _._ \#\
       /^|            \ _-_ \#
      '  `             \ _-_ \
                        \_

 {}jormungandr: {}
 {}jcli:        {}
 {}seed:        {}

###############################################################################
    "###,
        *style::icons::jormungandr,
        style::binary.apply_to(context.jormungandr().to_string()),
        *style::icons::jcli,
        style::binary.apply_to(context.jcli().to_string()),
        *style::icons::seed,
        style::seed.apply_to(context.seed()),
    )
}

use rand_chacha::ChaChaRng;

pub fn scenario_1(mut context: Context<ChaChaRng>) {
    let scenario_settings = prepare_scenario! {
        "simple network example",
        &mut context,
        topology [
            "node1",
            "node2" -> "node1",
        ]
        blockchain {
            consensus = Bft,
            number_of_slots_per_epoch = 10,
            slot_duration = 1,
            leaders = [ "node1" ],
            initials = [
                account "faucet1" with 1_000_000_000,
                account "faucet2" with 2_000_000_000 delegates to "node2",
            ],
        }
    };

    let mut controller = scenario_settings.build(context).unwrap();

    let node1 = controller.spawn_node("node1", true).unwrap();
    let node2 = controller.spawn_node("node2", false).unwrap();

    controller.monitor_nodes();
    std::thread::sleep(std::time::Duration::from_secs(10));
    let tip1 = node1.get_tip().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    node1.shutdown().unwrap();
    let _block = node2.get_block(&tip1).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    node2.shutdown().unwrap();

    controller.finalize();
}
