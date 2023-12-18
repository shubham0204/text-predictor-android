pub mod directed_graph {

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    #[derive(Debug)]
    struct Node {
        word: String,
        frequency: u32,
    }

    impl Node {
        fn new(word: String, frequency: u32) -> Node {
            Node { word, frequency }
        }
    }

    /// An directed-graph represented with the adjacency-list format
    /// This is a slightly modified version of the original adjacency-list, which is
    /// 2D linkedlist data structure.
    /// Consider the following graph:
    /// ```text
    /// A------> B
    /// ^
    /// |
    /// |       E
    /// |       ^
    /// |       |
    /// C------>D---> F
    /// ```
    /// Its representation in the [adjacency-list](https://en.wikipedia.org/wiki/Adjacency_list) format will be given by:
    /// ```text
    /// A -> B
    /// B -> NULL
    /// C -> A -> D
    /// D -> E -> F
    /// E -> NULL
    /// F -> NULL
    ///
    /// ```
    /// In our implementation, the pointers to head-nodes are stored in a `HashMap` instead of a linked list,
    /// to enable faster access. Also, NULL pointers are not included, meaning, every node must have a successor
    /// in the graph.
    pub struct DirectedGraph {
        words: HashMap<String, Vec<Node>>,
    }

    impl Default for DirectedGraph {
        fn default() -> Self {
            Self::new()
        }
    }

    impl DirectedGraph {
        /// Returns a new instance of `DirectedGraph` with an
        /// empty adjacency list
        #[must_use]
        pub fn new() -> DirectedGraph {
            DirectedGraph {
                words: HashMap::new(),
            }
        }

        /// Adds a directed edge in the graph, from node represented by `word1` to
        /// the node represented by `word2`.
        pub fn add_edge(&mut self, word1: &String, word2: &String) {
            if self.words.contains_key(word1) {
                // Get the vector with key = `word1`
                self.words.entry(word1.to_string()).and_modify(|list| {
                    let mut is_found = false;
                    // Search for `word2` in the list, if found,
                    // increment its `frequency`
                    for node in list.iter_mut() {
                        if node.word == *word2 {
                            node.frequency += 1;
                            is_found = true;
                            break;
                        }
                    }

                    // If not found, append a new `Node` to the vector
                    if !is_found {
                        list.push(Node::new(word2.to_string(), 1));
                    }
                });
            } 
            else {
                // If `words` does not contain `word1`,
                // Create a new vector `list`. Also, create a new `Node` with,
                // word = word2 and frequency = 1.
                // Assign the new vector to `word1` in the `words`
                let list: Vec<Node> = vec![Node::new(word2.to_string(), 1)];
                self.words.insert(word1.to_string(), list);
            }
        }

        /// Print the adjacency list of the directed-graph
        pub fn print(&self) {
            for (word, list) in &self.words {
                print!("{word} -> ");
                for list_node in list.iter() {
                    print!("{list_node:?} ");
                }
                println!();
            }
        }

        /// Save the adjacency list as a text file to the given
        /// `filepath`
        /// # Panics
        /// If file at `filepath` cannot be opened
        pub fn save(&self, filepath: &str) {
            let path = Path::new(filepath);
            let mut file = match File::create(path) {
                Err(e) => panic!("Could not open file {e}"),
                Ok(opened_file) => opened_file,
            };
            for (word, list) in &self.words {
                let _ = file.write(format!("{word} ").as_bytes());
                for list_node in list.iter() {
                    let _ = file
                        .write(format!("{} {} ", list_node.word, list_node.frequency).as_bytes());
                }
                let _ = file.write("\n".as_bytes());
            }
        }
    }
}

pub mod predictor {

    use std::io::Read;
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::Path;

    pub struct Predictor {
        words: HashMap<String, Vec<String>>,
    }

    impl Default for Predictor {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Predictor {

        #[must_use]
        pub fn new() -> Predictor {
            Predictor {
                words: HashMap::new(),
            }
        }

        pub fn load(&mut self, corpus_filepath: &String) {
            let mut file = match File::open(Path::new(corpus_filepath)) {
                Err(e) => panic!("Could not open file {e}"),
                Ok(file) => file,
            };

            let mut buffer = String::new();
            let _ = file.read_to_string(&mut buffer).unwrap();
            let lines: Vec<&str> = buffer.lines().collect();
            for line in lines {
                let parts: Vec<&str> = line.split_ascii_whitespace().collect();
                let mut pred = Vec::new();
                for part in parts[2..].iter() {
                    pred.push((*part).to_string());
                }
                self.words.insert(parts[0].to_string(), pred);
            }
        }

        pub fn predict(&self, word: &String) -> String {
            if self.words.contains_key(word) {
                let mut output = String::new();
                for word in self.words.get(word).unwrap().iter() {
                    let mut pred_word = word.to_string();
                    pred_word.push(' ');
                    output.push_str(pred_word.as_str());
                }
                output
            } else {
                String::new() 
            }
        }
    }
}
