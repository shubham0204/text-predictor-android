use std::collections::HashMap ; 
use std::io::Read;
use std::path::Path ; 
use std::fs::File ; 

pub struct Predictor {
    words: HashMap<String,Vec<String>> 
}

impl Predictor {

    pub fn new() -> Predictor {
        Predictor{ words: HashMap::new() } 
    }

    pub fn load( &mut self , corpus_filepath: &String ) {

        let mut file = match File::open( Path::new( corpus_filepath ) ) {
            Err( e ) => panic!( "Could not open file {}" , e ) ,
            Ok( file ) => file
        } ; 

        let mut buffer = String::new() ; 
        let _ = file.read_to_string( &mut buffer ).unwrap() ; 
        let lines: Vec<&str> = buffer.lines().collect() ; 
        for line in lines {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect() ; 
            let mut pred = Vec::new() ; 
            for part in parts[ 2.. ].iter() {
                pred.push( part.to_string() ) ; 
            }
            self.words.insert( parts[0].to_string() , pred ) ; 
        }

    }

    pub fn predict( &self , word: &String ) -> String {
        if self.words.contains_key( word ) {
            let mut output = String::new() ; 
            for word in self.words.get( word ).unwrap().into_iter() {
                let mut pred_word = word.to_string() ; 
                pred_word.push_str( " " ) ; 
                output.push_str( pred_word.as_str() ) ; 
            }
            return output ; 
        }
        else {
            return String::from( "" ) ; 
        }
    }

}