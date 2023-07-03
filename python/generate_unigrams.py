import re
import contractions
import pandas as pd

sent_regex = re.compile( r"(?:\.|\?|!)(?: \n?)?" )
punc_regex = re.compile( r""";|:|,|"|\{|\}|\[|\]|\'|\(|\)|“|”|’|‘|\/|-|…|@|™|—|_|\\|\*""" )
non_ascii_regex = re.compile( r"[^\x00-\x7F]+" )
number_regex = re.compile( r"(?:- ?)?\d+\.\d+|(?:- ?)?\d+" )

def filter_text(text : str):
    text = non_ascii_regex.sub( " " , text )
    text = contractions.fix( text , slang=False )
    text = punc_regex.sub( " " , text )
    text = sent_regex.sub( " [SEP] " , text )
    text = number_regex.sub( " " , text )
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

output_file = open( "generated/unigrams.txt" , "w" )

ds = pd.read_csv( "dataset/conversations.csv" )
questions = list( ds[ "question" ] )
answers = list( ds[ "answer" ] )
ds = [ " ".join( [questions[i] , answers[i]] ) for i in range( len( questions ) ) ]

for line in ds:
    text = filter_text( line.lower() )
    samples = [ get_unigrams( tokens ) for tokens in get_tokens( text ) ]
    samples = [ y for x in samples for y in x ]
    for s in samples:
        output_file.write( "{} {}\n".format( s[0] , s[1] ) )
output_file.close()


