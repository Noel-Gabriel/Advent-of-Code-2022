use std::fs::File;
use std::io::Read;
use regex::Regex;

struct Game {
    id:    u32,
    cubes: Vec<(u32, u32, u32)>,
}

fn main() {
    let games = read_input();

    const RED_MAX:   u32 = 12;
    const GREEN_MAX: u32 = 13;
    const BLUE_MAX:  u32 = 14;
    
    let possible_sum = games
        .iter()
        .fold(0, |mut sum, game| {
            if game.cubes.iter().all(|&(r, g, b)| r <= RED_MAX && g <= GREEN_MAX && b <= BLUE_MAX) {
                sum += game.id;
            }
            sum
        });

    let min_power = games
        .iter()
        .fold(0, |power, game| {
            let (r, g, b) = game.cubes
                .iter()
                .fold((0u32, 0u32, 0u32), |(rmax, gmax, bmax), &(r, g, b)| {
                    (std::cmp::max(rmax, r), std::cmp::max(gmax, g), std::cmp::max(bmax, b))
            });
            power + (r * g * b)
        });

    println!("Sum IDs of possible games: {}", possible_sum);
    println!("Sum of powers of min colors: {}", min_power);
    
}

fn read_input() -> Vec<Game> {
    let mut s = String::new();
    let _ = File::open("input02.txt")
        .expect("Could not open file.")
        .read_to_string(&mut s);

    let numerics = Regex::new(r"(\d+)").unwrap();

    s
        .trim()
        .lines()
        .fold(Vec::new(), |mut games, line| {
            let (id, cubes) = line
                .trim()
                .split_once(":")
                .expect("Could not parse game");

            let mut game = Game {
                id: 0,
                cubes: Vec::new(),
            };
            
            if let Some(i) = numerics.find(&id) {
                game.id = i.as_str().parse::<u32>().unwrap();
            } else { panic!("Game ID not found."); }

            for draw in cubes.split(";").collect::<Vec<&str>>() {
                let mut draws: (u32, u32, u32) = (0, 0, 0); // r, g, b
                draw
                    .split(",")
                    .into_iter()
                    .for_each(|d| {
                        let color: &mut u32;
                        if d.contains("d") { color = &mut draws.0; // re*d*
                        } else if d.contains("n") { color = &mut draws.1; // gree*n*
                        } else { color = &mut draws.2; }
                        *color = numerics.find(&d).unwrap().as_str().parse::<u32>().unwrap();
                    });
                game.cubes.push(draws);
            }
            games.push(game);
            games
        })
}
