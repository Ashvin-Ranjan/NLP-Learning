# This is similar to proper n-gram LMs except since it uses letters instead of tokens I can perhaps be a little less memory-efficient in my code
import random

INITIAL_N_VALUE = 12

test_text = """This is a test. And this is another, I really do love writing tests, something about doing so is so enjoyable to me.
I could never understand why though, the reason consistently eludes me. People tell me that it's a strange thing to love, but I do it anyways. I hope that one day someone else will share my love of writing these tests."""

def generate_ngram_map(text, n=INITIAL_N_VALUE):
    letters = list(text)
    ngrams = list(zip(*[letters[i:] for i in range(n+1)]))
    ngram_map = {}
    for ngram in ngrams:
        key = "".join(ngram[0:-1])
        if key in ngram_map:
            if ngram[-1] in ngram_map[key]:
                ngram_map[key][ngram[-1]] += 1
            else:
                ngram_map[key][ngram[-1]] = 1
        else:
            ngram_map[key] = {ngram[-1]: 1}
    return ngram_map

# Reduces N of ngram map by 1
def flatten_ngram_map(ngram_map):
    flattened_map = {}
    for ngram, values in ngram_map.items():
        if ngram[1:] in flattened_map:
            for unigram, value in values.items():
                if unigram in flattened_map[ngram[1:]]:
                    flattened_map[ngram[1:]][unigram] += value
                else:
                    flattened_map[ngram[1:]][unigram] = value
        else:
            flattened_map[ngram[1:]] = values
    return flattened_map

def gen_next_character(text, ngram_map, n=INITIAL_N_VALUE):
    if text[-n:] in ngram_map:
        values = ngram_map[text[-n:]]
        total = sum(values.values())
        index = random.randint(0,total-1)
        for k, v in values.items():
            index -= v
            if index <= 0:
                return k
    else:
        return gen_next_character(text, flatten_ngram_map(ngram_map), n=n-1)

def gen_text(text, ngram_map):
    print(text)
    while text[-1] != "\n":
        text += gen_next_character(text, ngram_map)
        print(text)
    return text

if __name__ == "__main__":
    text = ""
    # Get all of the data needed
    with open("../../datasets/bin/concat.txt", 'r') as f:
        text = f.read()

    ngram_map = generate_ngram_map(text)

    print("Map Generated")

    print(gen_text("t", ngram_map))