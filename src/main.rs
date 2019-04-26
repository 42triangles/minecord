use structopt::StructOpt;

use rand;

mod minefield;
use minefield::Minefield;

const NUMERALS: [&'static str; 9] = [
    ":zero:",
    ":one:",
    ":two:",
    ":three:",
    ":four:",
    ":five:",
    ":six:",
    ":seven:",
    ":eight:",
];

#[derive(Clone, StructOpt, Debug)]
#[structopt(name = "minecord")]
struct Conf {
    width: usize,
    height: usize,
    minecount: usize,
    #[structopt(default_value = ":bomb:")]
    mine: String,
    #[structopt(short = "o", long = "open-first")]
    open_first: bool,
}

fn main() {
    let conf = Conf::from_args();

    let mut rng = rand::thread_rng();

    let mut minefield = Minefield::new(conf.width, conf.height);
    minefield.mine(&mut rng, conf.minecount);

    let mine = conf.mine;

    let (safe_pos, safe_count) = if conf.open_first {
        minefield.safest_field(&mut rng)
    } else {
        ([0, 0], 0)  // This isn't used, it's just to please RAII. Also, not calculating that is just faster here.
    };

    macro_rules! field {
        (uncovered $x:expr) => {
            print!("{}", $x);
        };
        ($x:expr) => {
            print!("||{}||", $x);
        };
    }

    println!("***{}x{}, {} mines***", conf.width, conf.height, conf.minecount);
    for y in 0..(minefield.height()) {
        for x in 0..(minefield.width()) {
            if conf.open_first && [x, y] == safe_pos {
                field!(uncovered NUMERALS[safe_count]);
            } else if let Some(number) = minefield.number([x, y]) {
                field!(NUMERALS[number]);
            } else {
                field!(mine);
            }
        }

        println!("");
    }
}
