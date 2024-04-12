# This is similar to proper n-gram LMs except since it uses letters instead of tokens I can perhaps be a little less memory-efficient in my code
import random, json, time
from reverse_summation_tree import ReverseSummationTree
import treelib

INITIAL_N_VALUE = 12

test_text = """This is a test. And this is another, I really do love writing tests, something about doing so is so enjoyable to me.
I could never understand why though, the reason consistently eludes me. People tell me that it's a strange thing to love, but I do it anyways. I hope that one day someone else will share my love of writing these tests.\n"""

def generate_reverse_ngram_tree(text, n=INITIAL_N_VALUE):
    letters = list(text)
    ngrams = list(zip(*[letters[i:] for i in range(n+1)]))
    for i in range(1,n+1):
        ngrams += [letters[:i]] + [letters[-i:]]
    tree = ReverseSummationTree("<start>")
    for ngram in ngrams:
        tree.add_reverse_ngram(ngram[::-1])
    return tree

def gen_next_character(text, tree, n=INITIAL_N_VALUE):
    node = tree.query_reverse_ngram(list(text[-n:])[::-1])
    if node != None:
        index = random.randint(1,node.value)
        for v in tree.children.values():
            potential_node = v.query_reverse_ngram(list(text[-n:])[::-1])
            index -= 0 if potential_node == None else potential_node.value
            if index <= 0:
                return v.key
    else:
        return gen_next_character(text, tree, n=n-1)

def gen_text(text, tree):
    print(text, end='')
    while text[-1] != "\n":
        text += gen_next_character(text, tree)
        print(text[-1], end='')
    return text

if __name__ == "__main__":
    text = ""
    # Get all of the data needed
    with open("../datasets/bin/concat.txt", 'r') as f:
        text = f.read().lower()

    tree = generate_reverse_ngram_tree(text)

    print("Map Generated")
        
    generated_text = gen_text("t", tree)
    print(generated_text)