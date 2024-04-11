# This class will hold n-grams in a reverse older in a tree structure
# Each node in the tree will hold the sum total of all the values beneath it
# This means that it is easy to query (n-x)-grams.

class ReverseSummationTree:
    def __init__(self, key, value=0, children=None):
        self.key = key
        self.value = value
        # This is because default parameters are really janky in python and often cause memory linking issues
        self.children = {} if children is None else children
    
    def add_reverse_ngram(self, ngram):
        self.value += 1
        if len(ngram) == 0:
            return
        if not ngram[0] in self.children:
            self.children[ngram[0]] = ReverseSummationTree(ngram[0])
        self.children[ngram[0]].add_reverse_ngram(ngram[1:][:])

    def query_reverse_ngram(self, ngram):
        if len(ngram) == 0:
            return self
        if ngram[0] in self.children:
            return self.children[ngram[0]].query_reverse_ngram(ngram[1:])
        return None

    def dump(self):
        return {
            "key": self.key,
            "value": self.value,
            "children": [child.dump() for child in self.children.values()]
        }
    
    @staticmethod
    def load(data):
        children = {}
        for c in data["children"]:
            child = ReverseSummationTree.load(c)
            children[child.key] = child
        return ReverseSummationTree(data["key"], value=data["value"], children=children)