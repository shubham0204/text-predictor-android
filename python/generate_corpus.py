def get_top_k_words( words , freqs , k = 3 ):
    t = dict( zip( words , freqs ) )
    p = sorted( t , key=lambda x : t[x] , reverse=True )
    return p[ 0 : k ]

output_file = open( "../generated/corpus.txt" , "w" )

with open( "../generated/graph.txt" , "r" ) as file:
    for line in file:
        parts = line.split() 
        if len( parts ) > 2:
            word1 = parts[0]
            words = []
            freqs = []
            for i in range( 1 , len( parts ) - 1 , 2 ):
                words.append( parts[i] ) 
                freqs.append( int(parts[i+1]) )
            
            pred = get_top_k_words( words , freqs )
            output_file.write( word1 + " " +  str(len(pred)) + " " )
            for p in pred:
                output_file.write( p + " " )
            output_file.write( "\n" ) 
        else:
            continue
output_file.close()
