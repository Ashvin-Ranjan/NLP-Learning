cat ../text/*.txt > ../bin/concat.txt;
tr -sc "A-Za-z'" '\n' < ../bin/concat.txt | tr A-Z a-z | grep -xP "[^']+('[^']+)?(\r)?" | sort | uniq -c | sort -n -r > ../bin/sorted.txt