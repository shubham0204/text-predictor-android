use std::collections::HashMap ;
use std::fs::File ; 
use std::io::Write;
use std::path::Path ; 

#[derive(Debug)]
struct Node {
    word: String , 
    frequency: u32
}

impl Node {
    fn new( word: String , frequency: u32 ) -> Node {
        Node{ word , frequency }
    }
}

pub struct UndirectedGraph {
    words: HashMap<String,Vec<Node>> 
}

impl UndirectedGraph {

    pub fn new() -> UndirectedGraph {
        UndirectedGraph { words: HashMap::new() }
    }

    pub fn add( &mut self , word1: &String , word2: &String ) {
        if !self.words.contains_key( word1 ) {
            let list: Vec<Node> = vec![ Node::new( word2.to_string() , 1 ) ] ;
            self.words.insert( word1.to_string() , list ) ;
        }
        else {
            self.words.entry( word1.to_string() )
            .and_modify( |list| {
                let mut is_found = false ; 
                for node in list.iter_mut() {
                    if node.word == word2.to_string() {
                        node.frequency += 1 ; 
                        is_found = true ; 
                        break ; 
                    }
                }
                if !is_found  {
                    list.push( Node::new( word2.to_string() , 1 ) )
                }
            } ) ; 
        }
    }

    pub fn print( &self ) {
        for ( word , list ) in self.words.iter() {
            print!( "{word} -> " ) ; 
            for list_node in list.iter() {
                print!( "{:?} " , list_node ) ; 
            }
            print!( "\n" ) ;
        }
    }

    pub fn save( &self , filename: &String ) {
        let path = Path::new( filename ) ; 
        let mut file = match File::create( path ) {
            Err( e ) => panic!( "Could not open file {}" , e ) , 
            Ok( opened_file ) => opened_file ,
        } ; 
        for ( word , list ) in self.words.iter() {
            let _ = file.write( format!( "{} " , word ).as_bytes() ) ; 
            for list_node in list.iter() {
                let _ = file.write( format!( "{} {} " , list_node.word , list_node.frequency ).as_bytes() ) ; 
            }
            let _ = file.write( "\n".as_bytes() ) ;
        }
    }

}