pub mod undirected_graph ;
pub mod predictor ;
pub mod trie ; 

use std::fs::File; 
use std::io::{ Read , stdin };
use std::path::Path;

use self::predictor::Predictor;
use self::undirected_graph::UndirectedGraph ; 
use self::trie::Trie ; 

fn main() {

    println!( "Enter filepath for unigrams" ) ; 
    let file_path_str = &mut String::new() ; 
    let file_path = match stdin().read_line( file_path_str ) {
        Err( e ) => panic!( "Could not read from stdin {}" , e ) , 
        Ok( _ ) => file_path_str.strip_suffix( "\n" ).unwrap() 
    } ; 
    println!( "{}" , file_path ) ; 

    let unigrams_file_path = Path::new( file_path ) ; 
    let mut unigrams_file = match File::open( unigrams_file_path ) {
        Err( e ) => panic!( "Could not open file {}" , e ) ,
        Ok( file ) => file
    } ; 

    let mut file_contents = String::new() ; 
    let lines = match unigrams_file.read_to_string( &mut file_contents ) {
        Err( e ) => panic!( "Could not read file {}" , e ) , 
        Ok( buffer_size ) => (&file_contents[ 0..buffer_size ]).lines()
    } ; 
    
    let mut graph = UndirectedGraph::new() ; 
    let mut count: u32 = 0 ;
    for line in lines {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect() ; 
        graph.add( &parts[0].to_string() , &parts[1].to_string() ) ; 
        count += 1 ; 
        println!( "Unigrams read: {}" , count ) ; 
    }
    
    graph.save( &("output.txt".to_string()) ) ; 

    // instantiate_predictor() ; 
    instantiate_trie() ; 

    
    
}

fn instantiate_trie() {
    let mut trie = Trie::new() ; 
    trie.load( &("vocab.txt".to_string())) ; 
    
    
    loop {
        let input_phrase = &mut String::new() ; 
        let _ = stdin().read_line( input_phrase ).expect( "Could not read line from stdin" ) ;
        let input_word = input_phrase.strip_suffix( '\n' ).expect( "Could not strip suffix" ) ; 
        let output = trie.predict( &(input_word.to_string()) ) ;
        println!( "{:?}" , output ) ; 
    }

}

fn instantiate_predictor() {
    let mut predictor = Predictor::new() ; 
    predictor.load( &("corpus.txt".to_string()) ) ;

    loop {
        let input_word_str = &mut String::new() ; 
        let input_word = match stdin().read_line( input_word_str ) {
            Err( e ) => panic!( "Could not read from stdin {}" , e ) , 
            Ok( _ ) => input_word_str.strip_suffix( "\n" ).unwrap() 
        } ; 
        let output = predictor.predict( &(input_word.to_string()) ) ;
        println!( "{}" , output ) ; 
    }
}
