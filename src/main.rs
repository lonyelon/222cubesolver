use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Movement {
    BACK,
    DOWN,
    RIGHT,
}

struct Cube {
    hash: u64,
    prev_hash: u64,
    prev_move: Movement,
    depth: i32,
}

fn get_piece_in_position(cube: &Cube, piece_number: usize) -> (u8, u8) {
    let piece_mask = 0b111 << piece_number*5;
    let rotation_mask = 0b11 << piece_number*5 + 3;
    let piece_id = (cube.hash & piece_mask) >> piece_number*5;
    let piece_rotation = (cube.hash & rotation_mask) >> piece_number*5 + 3;
    
    (piece_id as u8, piece_rotation as u8)
}

fn print_move(movement: Movement) {
    match movement {
        Movement::RIGHT => print!("R"),
        Movement::DOWN => print!("D"),
        Movement::BACK => print!("B"),
    };
}

fn get_color(id: u8, rotation: u8) -> char {
    let faces = match id {
        0 => "WGO",
        1 => "WOB",
        2 => "WRG",
        3 => "WBR",
        4 => "YOG",
        5 => "YBO",
        6 => "YGR",
        7 => "YRB",
        _ => "_"
    };
    faces.chars().nth(usize::from(rotation % 3)).unwrap()
}

fn print_cube(cube: &Cube) {
    let vec: [(u8, u8); 8] = [
        get_piece_in_position(&cube, 0),
        get_piece_in_position(&cube, 1),
        get_piece_in_position(&cube, 2),
        get_piece_in_position(&cube, 3),
        get_piece_in_position(&cube, 4),
        get_piece_in_position(&cube, 5),
        get_piece_in_position(&cube, 6),
        get_piece_in_position(&cube, 7),
    ];

    println!("   {}{}\n   {}{}",
             get_color(vec[0].0, vec[0].1 + 0),
             get_color(vec[1].0, vec[1].1 + 0),
             get_color(vec[2].0, vec[2].1 + 0),
             get_color(vec[3].0, vec[3].1 + 0));

    println!("{}{} {}{} {}{} {}{}",
           get_color(vec[0].0, vec[0].1 + 1),
           get_color(vec[2].0, vec[2].1 + 2),
           get_color(vec[2].0, vec[2].1 + 1),
           get_color(vec[3].0, vec[3].1 + 2),
           get_color(vec[3].0, vec[3].1 + 1),
           get_color(vec[1].0, vec[1].1 + 2),
           get_color(vec[1].0, vec[1].1 + 1),
           get_color(vec[0].0, vec[0].1 + 2));

    println!("{}{} {}{} {}{} {}{}",
           get_color(vec[4].0, vec[4].1 + 2),
           get_color(vec[6].0, vec[6].1 + 1),
           get_color(vec[6].0, vec[6].1 + 2),
           get_color(vec[7].0, vec[7].1 + 1),
           get_color(vec[7].0, vec[7].1 + 2),
           get_color(vec[5].0, vec[5].1 + 1),
           get_color(vec[5].0, vec[5].1 + 2),
           get_color(vec[4].0, vec[4].1 + 1));

    println!("   {}{}",
           get_color(vec[6].0, vec[6].1 + 0),
           get_color(vec[7].0, vec[7].1 + 0));

    println!("   {}{}",
           get_color(vec[4].0, vec[4].1 + 0),
           get_color(vec[5].0, vec[5].1 + 0));
}

fn move_cube(cube: &Cube, movement: Movement) -> Cube {
    let mut vec = vec![
        get_piece_in_position(cube, 0),
        get_piece_in_position(cube, 1),
        get_piece_in_position(cube, 2),
        get_piece_in_position(cube, 3),
        get_piece_in_position(cube, 4),
        get_piece_in_position(cube, 5),
        get_piece_in_position(cube, 6),
        get_piece_in_position(cube, 7),
    ];
    let vec = match movement {
        Movement::RIGHT => vec![
            vec[0],
            (vec[3].0, (vec[3].1 + 2) % 3),
            vec[2],
            (vec[7].0, (vec[7].1 + 1) % 3),
            vec[4],
            (vec[1].0, (vec[1].1 + 1) % 3),
            vec[6],
            (vec[5].0, (vec[5].1 + 2) % 3),
        ],
        Movement::DOWN => vec![
            vec[0], vec[1], vec[2], vec[3],
            vec[5], vec[7], vec[4], vec[6],
        ],
        Movement::BACK => vec![
            (vec[1].0, (vec[1].1 + 2) % 3),
            (vec[5].0, (vec[5].1 + 1) % 3),
            vec[2], vec[3],
            (vec[0].0, (vec[0].1 + 1) % 3),
            (vec[4].0, (vec[4].1 + 2) % 3),
            vec[6], vec[7],
        ],
    };
    let mut hash = 0;
    for i in 0..8 {
        hash += (vec[i].0 as u64) << i*5;
        hash += (vec[i].1 as u64) << i*5 + 3;
    }
    Cube {
        hash: hash,
        prev_hash: cube.hash,
        prev_move: movement,
        depth: cube.depth + 1,
    }
}

fn solve_cube(cube: Cube) -> Vec<Movement> {
    let solved_cube_hash = get_solved_cube().hash;
    let mut map: HashMap<u64, Cube> = HashMap::new();
    let mut vec = VecDeque::new();
    let mut return_vec = vec![];

    vec.push_back(cube);

    while vec.len() > 0 {
        let mut current_cube = vec.pop_front().unwrap();
        if current_cube.hash == solved_cube_hash {
            return_vec = vec![];
            loop {
                if current_cube.depth == 0 {
                    break;
                }
                return_vec.push(current_cube.prev_move);
                let new_cube = map.get(&current_cube.prev_hash).unwrap();
                current_cube.hash = new_cube.hash;
                current_cube.prev_hash = new_cube.prev_hash;
                current_cube.prev_move = new_cube.prev_move;
                current_cube.depth = new_cube.depth;
            }
            break;
        } else if !map.contains_key(&current_cube.hash) {
            vec.push_back(move_cube(&current_cube, Movement::DOWN));
            vec.push_back(move_cube(&current_cube, Movement::BACK));
            vec.push_back(move_cube(&current_cube, Movement::RIGHT));
            map.insert(current_cube.hash, current_cube);
        } else {
            let present_cube = map.get(&current_cube.hash).unwrap();
            if present_cube.depth > current_cube.depth {
                map.insert(current_cube.hash, current_cube);
            }
        }
    }
    return_vec
}

fn get_solved_cube() -> Cube {
    Cube {
        hash: 0b00_111__00_110__00_101__00_100__00_011__00_010__00_001__00_000,
        prev_hash: 0,
        prev_move: Movement::RIGHT,
        depth: 0,
    }
}

fn main() {
    let mut cube = get_solved_cube();
    cube = move_cube(&cube, Movement::RIGHT);
    cube = move_cube(&cube, Movement::DOWN);
    cube = move_cube(&cube, Movement::BACK);
    cube = move_cube(&cube, Movement::RIGHT);
    cube = move_cube(&cube, Movement::RIGHT);
    cube = move_cube(&cube, Movement::BACK);
    cube = move_cube(&cube, Movement::BACK);
    cube = move_cube(&cube, Movement::DOWN);
    cube = move_cube(&cube, Movement::BACK);
    cube = move_cube(&cube, Movement::RIGHT);
    cube = move_cube(&cube, Movement::RIGHT);
    cube = move_cube(&cube, Movement::RIGHT);
    cube = move_cube(&cube, Movement::BACK);
    cube = move_cube(&cube, Movement::DOWN);
    cube.depth = 0;

    println!("Input cube is:");
    print_cube(&cube);

    let solution = solve_cube(cube);
    let mut i = solution.len();
    println!("Found solution at depth {} (counting only R, D and B moves).", i);
    println!("Solution is:");
    while i > 0 {
        if i > 1 && (solution[i - 1] == solution[i - 2]) {
            if i > 2 && (solution[i - 2] == solution[i - 3]) {
                print_move(solution[i - 1]);
                print!("'");
                i -= 3;
            } else {
                print!("2");
                print_move(solution[i - 1]);
                i -= 2;
            }
        } else {
            print_move(solution[i - 1]);
            i -= 1;
        }
        print!(" ");
    }
    println!();
}
