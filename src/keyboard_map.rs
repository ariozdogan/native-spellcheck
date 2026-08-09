use std::collections::HashMap;

pub fn build_adjacency_map() -> HashMap<char, Vec<char>> {
    let mut letter_adjacency_map: HashMap<char, Vec<char>> = HashMap::new();

    letter_adjacency_map.insert('q', vec!['w', 'a']);
    letter_adjacency_map.insert('w', vec!['q', 'e', 'a', 's']);
    letter_adjacency_map.insert('e', vec!['w', 'r', 's', 'd']);
    letter_adjacency_map.insert('r', vec!['e', 't', 'd', 'f']);
    letter_adjacency_map.insert('t', vec!['r', 'y', 'f', 'g']);
    letter_adjacency_map.insert('y', vec!['t', 'u', 'g', 'h']);
    letter_adjacency_map.insert('u', vec!['y', 'i', 'h', 'j']);
    letter_adjacency_map.insert('i', vec!['u', 'o', 'j', 'k']);
    letter_adjacency_map.insert('o', vec!['i', 'p', 'k', 'l']);
    letter_adjacency_map.insert('p', vec!['o', 'l']);

    letter_adjacency_map.insert('a', vec!['q', 'w', 's', 'z']);
    letter_adjacency_map.insert('s', vec!['a', 'd', 'w', 'e', 'z', 'x']);
    letter_adjacency_map.insert('d', vec!['s', 'f', 'e', 'r', 'x', 'c']);
    letter_adjacency_map.insert('f', vec!['d', 'g', 'r', 't', 'c', 'v']);
    letter_adjacency_map.insert('g', vec!['f', 'h', 't', 'y', 'v', 'b']);
    letter_adjacency_map.insert('h', vec!['g', 'j', 'y', 'u', 'b', 'n']);
    letter_adjacency_map.insert('j', vec!['h', 'k', 'u', 'i', 'n', 'm']);
    letter_adjacency_map.insert('k', vec!['j', 'l', 'i', 'o', 'm']);
    letter_adjacency_map.insert('l', vec!['k', 'o', 'p']);

    letter_adjacency_map.insert('z', vec!['x', 'a', 's']);
    letter_adjacency_map.insert('x', vec!['z', 'c', 's', 'd']);
    letter_adjacency_map.insert('c', vec!['x', 'v', 'd', 'f']);
    letter_adjacency_map.insert('v', vec!['c', 'b', 'f', 'g']);
    letter_adjacency_map.insert('b', vec!['v', 'n', 'g', 'h']);
    letter_adjacency_map.insert('n', vec!['b', 'm', 'h', 'j']);
    letter_adjacency_map.insert('m', vec!['n', 'j', 'k']);


    letter_adjacency_map
    
}

pub fn is_adjacent(user_input: char, edit: char) -> bool {
    let letters_adjacency_map: HashMap<char, Vec<char>> = build_adjacency_map();

    let letters_adjacent: bool = letters_adjacency_map.get(&user_input)
                                                      .map(|vec: &Vec<char>| vec.contains(&edit))
                                                      .unwrap_or(false);

    letters_adjacent
}