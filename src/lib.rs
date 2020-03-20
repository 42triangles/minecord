use std::cell::RefCell;

use structopt::StructOpt;

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
pub struct Conf {
    pub width: usize,
    pub height: usize,
    pub minecount: usize,
    #[structopt(default_value = ":bomb:")]
    pub mine: String,
    #[structopt(short = "o", long = "open-first")]
    pub open_first: bool,
}

#[derive(Debug)]
pub struct MinefieldGenerator<R: rand::Rng> {
    pub conf: Conf,
    pub rng: RefCell<R>,
}

impl<R: rand::Rng> MinefieldGenerator<R> {
    pub fn new(conf: Conf, rng: R) -> Self {
        MinefieldGenerator {
            conf,
            rng: std::cell::RefCell::new(rng),
        }
    }

    pub fn from_args(rng: R) -> Self {
        MinefieldGenerator::new(Conf::from_args(), rng)
    }
}

impl MinefieldGenerator<rand::rngs::ThreadRng> {
    pub fn with_thread_rng(conf: Conf) -> Self {
        MinefieldGenerator::new(conf, rand::thread_rng())
    }

    pub fn from_args_with_thread_rng() -> Self {
        MinefieldGenerator::with_thread_rng(Conf::from_args())
    }
}

impl<R: rand::Rng> std::fmt::Display for MinefieldGenerator<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let MinefieldGenerator { ref conf, ref rng } = self;
        let mut rng = rng.borrow_mut();

        let mut minefield = Minefield::new(conf.width, conf.height);
        minefield.mine(&mut *rng, conf.minecount);

        let mine = &conf.mine;

        let (safe_pos, safe_count) = if conf.open_first {
            minefield.safest_field(&mut *rng)
        } else {
            ([0, 0], 0)  // This isn't used, it's just to please RAII. Also, not calculating that is just faster here.
        };

        macro_rules! field {
            (uncovered $x:expr) => {
                write!(f, "{}", $x)?;
            };
            ($x:expr) => {
                write!(f, "||{}||", $x)?;
            };
        }

        writeln!(f, "***{}x{}, {} mines***", conf.width, conf.height, conf.minecount)?;
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

            if y != minefield.height() - 1 {
                writeln!(f, "")?;
            }
        }

        Ok(())
    }
}

pub fn run() {
    println!("{}", MinefieldGenerator::from_args_with_thread_rng());
}
