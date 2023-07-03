import re
import contractions
import pandas as pd

sent_regex = re.compile( r"(?:\.|\?|!)(?: \n?)?" )
punc_regex = re.compile( r""";|:|,|"|\{|\}|\[|\]|\'|\(|\)|“|”|’|‘|\/|-|…|@|™|—|_|\\|\*""" )
non_ascii_regex = re.compile( r"[^a-zA-Z]" )
num_regex = re.compile( r"[\d\.]" )

def filter_text(text : str):
    text = non_ascii_regex.sub( " " , text )
    text = contractions.fix( text , slang=False )
    text = punc_regex.sub( " " , text )
    text = sent_regex.sub( " [SEP] " , text )
    return text

def get_unigrams( tokens ):
    output = []
    for i in range( len( tokens ) - 1 ):
        output.append( (tokens[i] , tokens[i+1]) )
    return output

def get_tokens( text ):
    sents = text.split( "[SEP]" ) 
    seqs = [ sent.split() for sent in sents ]
    seqs = [ seq for seq in seqs if len( seq ) > 2 ]
    return seqs

output_file = open( "generated/vocab.txt" , "w" )

ds = pd.read_csv( "dataset/conversations.csv" )
questions = list( ds[ "question" ] )
answers = list( ds[ "answer" ] )
ds = [ " ".join( [questions[i] , answers[i]] ) for i in range( len( questions ) ) ]

tokens = []
for line in ds:
    text = filter_text( line.lower() )
    tokens += get_tokens( text ) 
tokens = [ y for x in tokens for y in x ]
tokens = set( tokens )
tokens = sorted( tokens )

for token in tokens:
    output_file.write( "{}\n".format( token ) )
output_file.close()
