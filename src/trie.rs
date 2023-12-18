use std::fs::File;
use std::io::Read;
use std::path::Path;

/// A structure which represents a node
/// in the trie.
struct TrieNode {
    /// The element held by this node i.e. a letter from the alphabet
    data: char,

    /// Indicates whether this node is a leaf OR the end of a word composed of
    /// all letters from the root node to this node
    is_end_of_word: bool,

    /// An array of 26-children representing 26-letters of the alphabet. Each child is a
    /// optional `Box<>` pointer which points to the child `TrieNode`.
    children: Vec<Option<Box<TrieNode>>>,
}

impl TrieNode {
    /// Creates a new `TrieNode` with given `data`.
    /// The `children` are initialized to a vec![ None ; 26 ] and `is_end_of_word` is set to `false`.
    fn new(data: char) -> TrieNode {
        let mut default_children = Vec::new();
        for _ in 0..26 {
            default_children.push(None);
        }
        TrieNode {
            data,
            is_end_of_word: false,
            children: default_children,
        }
    }
}

/// Creates a new Trie (prefix-tree) for
/// word-level completion
/// Refer this blog for implementation: https://www.geeksforgeeks.org/introduction-to-trie-data-structure-and-algorithm-tutorials
pub struct Trie {
    /// This is an optional pointer to a `TrieNode` which represents the root of the prefix-tree.
    /// Initially, when the tree is empty, it is set to `None`
    root: Option<Box<TrieNode>>,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    /// Creates a new instance of `Trie`.
    /// The `root` is initialized to a new `TrieNode` with `data='*'` which only a placeholder value,
    /// and has no effect on the use of the data-structure
    /// # Examples
    /// ```
    /// let mut trie = Trie::new() ;
    /// ```
    #[must_use]
    pub fn new() -> Trie {
        Trie {
            root: Some(Box::new(TrieNode::new('*'))),
        }
    }

    /// Loads a vocabulary, which is a text file where each line contains a word.
    /// The vocabulary is read line-by-line and the words are inserted in the trie.
    /// # Arguments
    /// * `vocab_filepath`: The path to the vocab text file
    /// # Examples
    /// ```
    /// let mut trie = Trie::new() ;
    /// trie.load( "vocab.txt" ) ;
    /// ```
    pub fn load(&mut self, vocab_filepath: &str) {
        let mut file = File::open(Path::new(vocab_filepath)).expect("Could not open file");
        let mut file_contents = String::new();
        let _ = file
            .read_to_string(&mut file_contents)
            .expect("Could not read vocab");
        let lines: Vec<&str> = file_contents.lines().collect();
        for line in lines {
            self.insert(line );
        }
    }

    /// Given the phrase, predict the words that could be formed using the
    /// `phrase` as a prefix. We perform inorder traversal on the trie, from the node
    /// that represents the last character of the phrase
    /// # Arguments
    /// * `phrase`: A part of the word used as the prefix to predict complete words
    /// # Examples
    /// ```rust
    /// let mut trie = Trie::new() ;
    /// trie.load( "vocab.txt" )
    /// let output = trie.predict( "hell" ) ;
    /// ```
    /// The above examples returns the output, as a sequence of predicted words separated by
    /// a space, for ex. "hell hello help helped helps"
    pub fn predict(&self, phrase: &str) -> String {
        let mut current_node = &self.root;
        let phrase = phrase.to_owned() ; 
        let word_chars: Vec<char> = phrase.chars().collect();
        let mut i = 0 ;
        while current_node.is_some() && i < word_chars.len() {
            let c = word_chars[ i ] ;
            let index = ((c as u32) - 97) as usize;
            current_node = &current_node.as_ref().unwrap().children[index];
            i += 1 ; 
        }
        let mut output: String = String::new();
        if current_node.is_some() {
            let mut stack: Vec<&Option<Box<TrieNode>>> = Vec::new();
            let mut words: Vec<String> = Vec::new();
            stack.push(current_node);
            words.push(phrase);
            while !stack.is_empty() {
                let p = stack.pop().unwrap();
                let seq = words.pop().unwrap();
                if p.as_ref().unwrap().is_end_of_word {
                    output.push_str(&seq.clone());
                    output.push(' ');
                }
                for i in (0..26).rev() {
                    if p.as_ref().unwrap().children[i].is_some() {
                        stack.push(&p.as_ref().unwrap().children[i]);
                        let mut seq = seq.clone();
                        seq.push(p.as_ref().unwrap().children[i].as_ref().unwrap().data);
                        words.push(seq);
                    }
                }
            }
        }
        output
    }

    /// Insert the given `word` in the trie
    fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        let word_chars: Vec<char> = word.chars().collect();
        let a: u64 = 97;
        for c in word_chars {
            let index = ((c as u64) - a) as usize;
            if current_node.as_ref().unwrap().children[index].is_none() {
                current_node.as_mut().unwrap().children[index] = Some(Box::new(TrieNode::new(c)));
            }
            current_node = &mut current_node.as_mut().unwrap().children[index];
        }
        current_node.as_mut().unwrap().is_end_of_word = true;
    }
    
}
