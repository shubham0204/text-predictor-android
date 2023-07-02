use std::io::Read;
use std::path::Path ; 
use std::fs::File ; 

struct TrieNode {
    data: char , 
    is_end_of_word: bool , 
    children: Vec<Option<Box<TrieNode>>>
}

impl TrieNode {
    fn new( data: char ) -> TrieNode {
        let mut default_children = Vec::new() ;
        for _ in 0..26 {
            default_children.push( None ) ; 
        }
        TrieNode { data , is_end_of_word: false, children: default_children }
    }
}

pub struct Trie {
    root: Option<Box<TrieNode>>
} 

impl Trie {

    pub fn new() -> Trie {
        Trie{ root: Some( Box::new( TrieNode::new( '*' ) ) ) }
    }

    pub fn load( &mut self , vocab_filepath: &String ) {
        let mut file = File::open( Path::new( vocab_filepath ) ).expect( "Could not open file" ) ;
        let mut file_contents = String::new() ; 
        let _ = file.read_to_string( &mut file_contents ).expect( "Could not read vocab" ) ; 
        let lines: Vec<&str> = file_contents.lines().collect() ; 
        for line in lines {
            self.insert( &(line.to_string()) ) ;
        }
    }

    pub fn insert( &mut self , word: &String ) {
        let mut current_node = &mut self.root ; 
        let word_chars: Vec<char> = word.chars().collect() ;
        let a: u64 = 97 ; 
        for c in word_chars {
            let index = ((c as u64) - a) as usize ;
            if current_node.as_ref().unwrap().children[ index ].is_none() {
                current_node.as_mut().unwrap().children[ index ] = Some( Box::new( TrieNode::new( c ) ) ) ;
            }
            current_node = &mut current_node.as_mut().unwrap().children[ index ] ; 
        } 
        current_node.as_mut().unwrap().is_end_of_word = true ;
    }

    pub fn predict( &self , phrase: &String ) -> String {
        let mut current_node = &self.root ; 
        let mut phrase = phrase.clone() ;
        let word_chars: Vec<char> = phrase.chars().collect() ;
        for c in word_chars {
            let index = ((c as u32) - 97) as usize ;
            if current_node.as_ref().unwrap().children[ index ].is_none() {
                break ;
            }
            current_node = &current_node.as_ref().unwrap().children[ index ] ; 
        } 
        let mut output: String = String::new() ; 
        let mut stack: Vec<&Option<Box<TrieNode>>> = Vec::new() ; 
        let mut words: Vec<String> = Vec::new() ; 
        stack.push( current_node ) ; 
        words.push( phrase ) ;
        while !stack.is_empty() {
        
            let p = stack.pop().unwrap() ; 
            let seq = words.pop().unwrap() ; 
            if p.as_ref().unwrap().is_end_of_word {
                output.push_str( &seq.clone() ) ; 
                output.push( ' ' ) ; 
            }
            for i in (0..26).rev() {
                if p.as_ref().unwrap().children[ i ].is_some() {
                    stack.push( & p.as_ref().unwrap().children[ i ] ) ;
                    let mut seq = seq.clone() ;  
                    seq.push( p.as_ref().unwrap().children[ i ].as_ref().unwrap().data ) ; 
                    words.push( seq ) ; 
                }
            }
            
        }
        output
    }

}