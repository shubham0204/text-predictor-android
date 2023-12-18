pub mod next_word_predictor;
pub mod trie;

use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;

use self::next_word_predictor::directed_graph::DirectedGraph;
use self::next_word_predictor::predictor::Predictor;
use self::trie::Trie;

fn main() {
    let unigrams_file_path = Path::new("generated/unigrams.txt");
    let mut unigrams_file = match File::open(unigrams_file_path) {
        Err(e) => panic!("Could not open file {e}"),
        Ok(file) => file,
    };

    let mut file_contents = String::new();
    let lines = match unigrams_file.read_to_string(&mut file_contents) {
        Err(e) => panic!("Could not read file {e}"),
        Ok(buffer_size) => file_contents[0..buffer_size].lines(),
    };

    let mut graph = DirectedGraph::new();
    println!( "Reading unigrams..." ) ;
    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        graph.add_edge(&parts[0].to_string(), &parts[1].to_string());
    }
    println!( "Graph saved to generated/corpus.txt" ) ;
    graph.save( "generated/corpus.txt" );

    println!( "Option 0 -> next word predictor\nOption 1 -> word autocomplete" ) ;
    let mut input_option = String::new() ; 
    stdin().read_line( &mut input_option ).expect( "Could not read line" ) ; 
    let option: u32 = input_option.parse().expect( "Could not parse input_option" ) ;
    if option == 0 {
        instantiate_predictor() ;
    }
    else if option == 1 {
        instantiate_trie();
    }

}

fn instantiate_trie() {
    let mut trie = Trie::new();
    trie.load("generated/vocab.txt");

    loop {
        let input_phrase = &mut String::new();
        let _ = stdin()
            .read_line(input_phrase)
            .expect("Could not read line from stdin");
        let input_word = input_phrase
            .strip_suffix('\n')
            .expect("Could not strip suffix");
        let output = trie.predict(input_word);
        println!("{output:?}");
    }
}

fn instantiate_predictor() {
    let mut predictor = Predictor::new();
    predictor.load(&("generated/corpus.txt".to_string()));
    loop {
        let input_word_str = &mut String::new();
        let input_word = match stdin().read_line(input_word_str) {
            Err(e) => panic!("Could not read from stdin {e}"),
            Ok(_) => input_word_str.strip_suffix('\n').unwrap(),
        };
        let output = predictor.predict(&(input_word.to_string()));
        println!("{output}");
    }
}
