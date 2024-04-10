from collections import Counter
import re
import json

test_text = """This is a test. And this is another, I really do love writing tests, something about doing so is so enjoyable to me.
I could never understand why though, the reason consistently eludes me. People tell me that it's a strange thing to love, but I do it anyways. I hope that one day someone else will share my love of writing these tests."""

# Finds the most common two sequences
def comm_subtk_sq(token_frequencies, n=2):
    counter = Counter({})

    for frequency, tokens in token_frequencies:
        # Generate list of all of the bigram subtoken sequences
        sequences = list(zip(*[tokens[i:] for i in range(n)]))
        #print(sequences)
        counter.update(sequences*frequency)

    # Find the most commmon one
    return list(counter.most_common(1)[0][0])

# Merges tokens together
def merge_tk(tokens, merger):
    joined = "".join(merger)
    merged_tokens = []
    index = 0
    while index < len(tokens):
        if (tokens[index:index+len(merger)] == merger):
            merged_tokens.append(joined)
            index += 1
        else:
            merged_tokens.append(tokens[index])
        index += 1
    return merged_tokens

# Process frequencies txt file into actual data
def preprocess_frequencies(text):
    out = []
    for t in text.split("\n"):
        new_entry = t.strip().split(" ")
        new_entry[0] = int(new_entry[0])
        new_entry[1] = list(new_entry[1] + "\n")
        out.append(new_entry)
    return out

# Apply BPE
def apply_bpe(text, tokens):
    tokenized_list = list(re.sub(r"([\w'])([^\w'])", r"\1\n\2", text))
    for token in tokens:
        tokenized_list = merge_tk(tokenized_list, token)
    
    tokenized_list = [tk.strip() if len(tk.strip()) != 0 else tk for tk in tokenized_list]
    return tokenized_list

# Prep the variables needed for BPE
token_list = []
token_frequencies = []

# Get all of the data needed
with open("../datasets/bin/sorted.txt", 'r') as f:
    token_frequencies = preprocess_frequencies(f.read())

print(token_frequencies)

# Actually run BPE
for i in range(350):
    comm = comm_subtk_sq(token_frequencies)
    token_list.append(comm)
    print(comm)
    for i, j in enumerate(token_frequencies):
        token_frequencies[i][1] = merge_tk(j[1], comm)

with open("bin/output.json", "w+") as f:
    f.write(json.dumps(token_list, indent=1))

# Test it out
print("|".join(apply_bpe(test_text.lower(), token_list)))