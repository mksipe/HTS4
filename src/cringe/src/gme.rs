use colored::*;

pub fn main() {
    println!("{}","Games".green());
    let list = vec![
        "0ad",
        "2048",
        "3dchess",
        "7kaa",
        "a7xpg",
        "abe",
        "acm",
        "adonthell",
        "airstrike",
        "aisleriot",
        "gnome-games",
        "alex4",
        "alienblaster",
        "allure",
        "amoebax",
        "amphetamine",
        "an",
        "angband",
        "angrydd",
        "animals",
        "antigravitaattori",
        "armagetronad",
        "asc",
        "asciijump",
        "asylum",
        "atanks",
        "atom4",
        "atomix",
        "auralquiz",
        "ballerburg",
        "ballz",
        "bambam",
        "barrage",
        "bastet",
        "bb",
        "beneath-a-steel-sky",
        "berusky",
        "berusky2",
        "between",
        "billard-gl",
        "biloba",
        "biniax2",
        "black-box",
        "blastem",
        "blinken",
        "blobandconquer",
        "blobby",
        "bloboats",
        "blobwars",
        "blockattack",
        "blockout2",
        "blocks-of-the-undead",
        "bombardier",
        "bomber",
        "bomberclone",
        "boohu",
        "boswars",
        "bouncy",
        "bovo",
        "braillefont",
        "brainparty",
        "briquolo",
        "brutalchess",
        "bsdgames",
        "btanks",
        "bucklespring",
        "bugsquish",
        "bumprace",
        "burgerspace",
        "bygfoot",
        "bzflag",
        "cappuccino",
        "cataclysm-dda-curses",
        "caveexpress",
        "cavepicker",
        "caferino",
        "cgoban",
        "chessx",
        "chocolate-doom",
        "chromium-bsu",
        "circuslinux",
        "clobot",
        "colorcode",
        "colossal-cave-adventure",
        "connectagram",
        "cookietool",
        "cowsay",
        "crack-attack",
        "crawl",
        "crazywa",
        "crimson",
        "crispy-doom",
        "criticalmass",
        "crossfire-client",
        "crrcsim",
        "csmash",
        "cube2",
        "cultivation",
        "curseofwar",
        "cutemaze",
        "dungeon",
        "darkplaces",
        "ddnet",
        "deal",
        "dealer",
        "desmume",
        "deutex",
        "dizzy",
        "dmagnetic",
        "dodgindiamond2",
        "dolphin-emu",
        "doomsday",
        "dopewars",
        "dossizola",
        "drascula",
        "dreamchess",
        "dustracing2d",
        "dvorak7min",
        "eboard",
        "efp",
        "einstein",
        "el-ixir",
        "ember-media",
        "empire",
        "endless-sky",
        "enemylines3",
        "enemylines7",
        "enigma",
        "epipany",
        "etw",
        "excellent-bifurcation",
        "extremetuxracer",
        "fairy-stockfish",
        "fairymax",
        "fathom",
        "fb-music-high",
        "filler",
        "filters",
        "five-or-more",
        "fzmo-console",
        "flare",
        "fight-of-the-amazon-queen",
        "fligtgear",
        "flobopuyo",
        "fltk1.1-games",
        "fltk1.3-games",
        "foobillardplus",
        "fortunate.app",
        "fortune-anarchism",
        "fortunes",
        "four-in-a-row",
        "freealchemist",
        "freecell-solver-bin",
        "freeciv",
        "freecol",
        "freedink",
        "freedm",
        "freedoom",
        "freedroid",
        "freedroidrpg",
        "freegish",
        "freeonrion",
        "freesweep",
        "freetennis",
        "frotz",
        "fruit",
        "funguloids",
        "funnyboat",
        "galois",
        "gamazons",
        "gamine",
        "garden-of-coloured-lights",
        "gargoyle-free",
        "gav",
        "gbatnav",
        "gbrainy",
        "gearhead",
        "gearhead2",
        "geki2",
        "geki3",
        "gemdropx",
        "gigalomania",
        "gl-117",
        "glaurung",
        "glhack",
        "glob2",
        "glpeces",
        "gltron",
        "glulxe",
        "gm-assistant",
        "gmult",
        "gnome-2048",
        "gnome-breakout",
        "gnome-chess",
        "gnome-games-app",
        "gnome-klotski",
        "gnome-mahjongg",
        "gnome-masterming",
        "gnome-mines",
        "gnome-nibbles",
        "gnome-robots",
        "gnome-tetravex",
        "gnubg",
        "gnubik",
        "gnuchess",
        "gnugo",
        "gnujump",
        "gnuminishogi",
        "gnurobbo",
        "gnushogi",
        "golly",
        "goverlay",
        "gplanetarity",
        "granatier",
        "granule",
        "gravitation",
        "gravitywars",
        "greed",
        "grhino",
        "groundhog",
        "gsalliere",
        "gtans",
        "gtetrinet",
        "gtans",
        "gtkatlantic",
        "gtlballs",
        "gtkpool",
        "gunroar",
        "hachu",
        "hannah",
        "hedgewars",
        "heroes",
        "hex-a-hop",
        "hexalate",
        "hexxagon",
        "hitori",
        "hoichess",
        "hollywood",
        "holotz-castle",
        "hyperrogue",
        "iagno",
        "icebreaker",
        "ii-esu",
        "instead",
        "ioquake3",
        "jag",
        "jerry",
        "jester",
        "jigzo",
        "jumpnbump",
        "junior-toys",
        "jzip",
        "kajongg",
        "kanagram",
        "kapman",
        "katomic",
        "kawari8",
        "kball",
        "kblackbox",
        "kblocks",
        "kbounce",
        "kbreakout",
        "kcheckers",
        "kdegames-card-data-kf5",
        "kdiamond",
        "ketm",
        "kfourinlne",
        "kgoldrunner",
        "khangman",
        "kigo",
        "kildclient",
        "killbots",
        "kirki",
        "kjumpingcube",
        "klickety",
        "klines",
        "kmahjongg",
        "kmines",
        "knavalbase",
        "knetwalk",
        "knights",
        "kobodeluxe",
        "kolf",
        "kollision",
        "koml",
        "kollision",
        "komi",
        "konquest",
        "koules",
        "kpat",
        "krank",
        "kraptor",
        "kreversi",
        "kshsen",
        "ksirk",
        "ksnakeduel",
        "kspaceduel",
        "ksquares",
        "ksudoku",
        "ktuberling",
        "kubrick",
        "laby",
        "lambdahack",
        "late",
        "lbreakout2",
        "leela-zero",
        "lgc-pg",
        "lgeneral",
    ];
    for obj in list.iter() {
        let out = which::which(obj.clone());
        let _ret = match out {
            Ok(ref _ret) => println!("[{}] {:?}", "+".yellow(), out.unwrap()),
            Err(_error) => print!("{}", ""),
        };
    }
}