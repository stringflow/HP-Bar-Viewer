mod gfx;

use crate::gfx::*;
use lazy_static::*;
use std::collections::*;
use std::env;
use std::io::*;
use termcolor::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || args[1] == "--help" {
        print_help(&args);
        return;
    }

    if args[1] == "--species" {
        for name in SPECIES_MAP.keys() {
            println!("{}", name);
        }
        return;
    } else if args.len() < 4 {
        print_help(&args);
        return;
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    macro_rules! cprint {
        ($color: expr, $message: literal, $($element:expr),*) => {
            stdout.set_color(ColorSpec::new().set_fg(Some($color))).unwrap();
            writeln!(&mut stdout, $message, $(
                $element,
            )*);
        };
    }

    let name = &args[1].to_lowercase();

    if let Some(base_hp) = SPECIES_MAP.get(&name[..]) {
        if let Ok(level) = args[2].parse::<u32>() {
            if let Ok(current_hp) = args[3].parse::<u32>() {
                let max_hp = get_maxhp(level, *base_hp);
                let pixels = get_pixels(current_hp, max_hp);
                cprint!(Color::White, "lv{} {}", level, name);
                cprint!(Color::White, "{} max hp", max_hp);
                cprint!(Color::White, "{}/48 pixels", pixels);

                let font = Font {
                    bitmap: png_load(include_bytes!("font.png")),
                    character_width: 8,
                    character_height: 8,
                    charmap: "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z ( ) : ; [ ] a b c d e f g h i j k l m n o p q r s t u v w x y z 'e 'd 'l 's 't 'v  _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ ' PK MN 'r 'm ? ! . _ _ _ _ _ _ _M $ * . / , _F 0 1 2 3 4 5 6 7 8 9"
                        .split(' ')
                        .collect(),
                };

                let mut bitmap = png_load(include_bytes!("base_bar.png"));
                bitmap_text(&font, &mut bitmap, 8, 0, &name.to_uppercase());
                bitmap_text(&font, &mut bitmap, 40, 8, &level.to_string());
                bitmap_fill(&mut bitmap, 32, 19, pixels, 2, 127, 56, 72);

                let file_name = format!("hpbar-{}-lv{}-{}.png", name, level, current_hp);
                png_write(&file_name, &bitmap);
                cprint!(Color::Green, "written to {}", file_name);

                if current_hp != max_hp && pixels == get_pixels(current_hp + 1, max_hp) {
                    cprint!(Color::Yellow, "Warning: the hp bars for {} hp and {} hp look identical", current_hp, current_hp + 1);
                }
            } else {
                cprint!(Color::Red, "{} is not a numeric current hp.", args[3]);
            }
        } else {
            cprint!(Color::Red, "{} is not a numeric level.", args[2]);
        }
    } else {
        cprint!(Color::Red, "{} is not a valid species name. Use '{} --species' to list the supported names.", args[1], args[0]);
    }

    stdout.reset().unwrap();
}

fn print_help(args: &Vec<String>) {
    println!("{} --species                       |  lists the supported species names", args[0]);
    println!("{} (pokemon) (level) (current hp)  |  writes a screenshot of the hp bar to a png", args[0]);
}

fn get_pixels(hp: u32, mut maxhp: u32) -> u32 {
    let mut n = hp * 48;

    if maxhp > 0xff {
        maxhp = maxhp / 4;
        n = (n & 0xff0000) | ((n & 0x00ffff) / 4);
    }

    return (n / maxhp) & 0xff;
}

fn get_maxhp(level: u32, base: u32) -> u32 {
    return ((2 * (8 + base)) * level / 100 + level + 10) & 0xffff;
}

lazy_static! {
    static ref SPECIES_MAP: HashMap<&'static str, u32> = [
        ("rhydon", 105),
        ("kangaskhan", 105),
        ("nidoran_m", 46),
        ("clefairy", 70),
        ("spearow", 40),
        ("voltorb", 40),
        ("nidoking", 81),
        ("slowbro", 95),
        ("ivysaur", 60),
        ("exeggutor", 95),
        ("lickitung", 90),
        ("exeggcute", 60),
        ("grimer", 80),
        ("gengar", 60),
        ("nidoran_f", 55),
        ("nidoqueen", 90),
        ("cubone", 50),
        ("rhyhorn", 80),
        ("lapras", 130),
        ("arcanine", 90),
        ("mew", 100),
        ("gyarados", 95),
        ("shellder", 30),
        ("tentacool", 40),
        ("gastly", 30),
        ("scyther", 70),
        ("staryu", 30),
        ("blastoise", 79),
        ("pinsir", 65),
        ("tangela", 65),
        ("growlithe", 55),
        ("onix", 35),
        ("fearow", 65),
        ("pidgey", 40),
        ("slowpoke", 90),
        ("kadabra", 40),
        ("graveler", 55),
        ("chansey", 250),
        ("machoke", 80),
        ("mr.mime", 40),
        ("hitmonlee", 50),
        ("hitmonchan", 50),
        ("arbok", 60),
        ("parasect", 60),
        ("psyduck", 50),
        ("drowzee", 60),
        ("golem", 80),
        ("magmar", 65),
        ("electabuzz", 65),
        ("magneton", 50),
        ("koffing", 40),
        ("mankey", 40),
        ("seel", 65),
        ("diglett", 10),
        ("tauros", 75),
        ("farfetch'd", 52),
        ("venonat", 60),
        ("dragonite", 91),
        ("doduo", 35),
        ("poliwag", 40),
        ("jynx", 65),
        ("moltres", 90),
        ("articuno", 90),
        ("zapdos", 90),
        ("ditto", 48),
        ("meowth", 40),
        ("krabby", 30),
        ("vulpix", 38),
        ("ninetales", 73),
        ("pikachu", 35),
        ("raichu", 60),
        ("dratini", 41),
        ("dragonair", 61),
        ("kabuto", 30),
        ("kabutops", 60),
        ("horsea", 30),
        ("seadra", 55),
        ("sandshrew", 50),
        ("sandslash", 75),
        ("omanyte", 35),
        ("omastar", 70),
        ("jigglypuff", 115),
        ("wigglytuff", 140),
        ("eevee", 55),
        ("flareon", 65),
        ("jolteon", 65),
        ("vaporeon", 130),
        ("machop", 70),
        ("zubat", 40),
        ("ekans", 35),
        ("paras", 35),
        ("poliwhirl", 65),
        ("poliwrath", 90),
        ("weedle", 40),
        ("kakuna", 45),
        ("beedrill", 65),
        ("dodrio", 60),
        ("primeape", 65),
        ("dugtrio", 35),
        ("venomoth", 70),
        ("dewgong", 90),
        ("caterpie", 45),
        ("metapod", 50),
        ("butterfree", 60),
        ("machamp", 90),
        ("golduck", 80),
        ("hypno", 85),
        ("golbat", 75),
        ("mewtwo", 106),
        ("snorlax", 160),
        ("magikarp", 20),
        ("muk", 105),
        ("kingler", 55),
        ("cloyster", 50),
        ("electrode", 60),
        ("clefable", 95),
        ("weezing", 65),
        ("persian", 65),
        ("marowak", 60),
        ("haunter", 45),
        ("abra", 25),
        ("alakazam", 55),
        ("pidgeotto", 63),
        ("pidgeot", 83),
        ("starmie", 60),
        ("bulbasaur", 45),
        ("venusaur", 80),
        ("tentacruel", 80),
        ("goldeen", 45),
        ("seaking", 80),
        ("ponyta", 50),
        ("rapidash", 65),
        ("rattata", 30),
        ("raticate", 55),
        ("nidorino", 61),
        ("nidorina", 70),
        ("geodude", 40),
        ("porygon", 65),
        ("aerodactyl", 80),
        ("magnemite", 25),
        ("charmander", 39),
        ("squirtle", 44),
        ("charmeleon", 58),
        ("wartortle", 59),
        ("charizard", 78),
        ("oddish", 45),
        ("gloom", 60),
        ("vileplume", 75),
        ("bellsprout", 50),
        ("weepinbell", 65),
        ("victreebel", 80),
    ]
    .iter()
    .copied()
    .collect();
}
