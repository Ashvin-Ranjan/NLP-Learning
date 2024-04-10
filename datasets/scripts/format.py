import re

formatted_file = "nineteen-eighty-four.txt"

text = ""
with open(formatted_file, "r", encoding="utf-8") as f:
    text = f.read()

text = text.replace("“", '"').replace("”", '"').replace("’", "'").replace("—", "-")

text = re.sub(r"[^\n]\n[^\n]", "", text)

while "\n\n" in text:
    text = text.replace("\n\n", "\n")

with open(formatted_file, "w") as f:
    f.write(text)