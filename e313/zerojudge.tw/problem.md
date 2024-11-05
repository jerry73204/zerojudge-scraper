# Task Description
『字串裡面有太多不同的字是不和諧的』，喵喵妮維森這麼說著。

因此她想要在 N 個字串中，找出含有最少相異字母的字串，  
當有多個都是最少相異字母時，則選出字典排序最小者。

舉例來說，對於三個字串  
"ABBCAAB"、"AABBACC"、"AAPPCCSS"，  
可以看出各個字串所含有的相異字母數量分別是  
3、3、4。

其中按照字典間互相比較可以得到"ABBCAAB" \> "AABBACC"，  
所以最後會得到字串"AABBACC"。

請幫助喵喵維森找出最和諧的字串。
# Input Format
第一行有一個正整數 N (N ≤ 1000)，代表接下來有 N 個字串。

接下來有 N 行，  
每一行有一個字串，字串內只會含有大寫字母A \~ Z。
# Output Format
找出 N 個字串內最少相異字母的字串，  
當有多個字串相異字母都是最少時，則選出字典排序最小者。
# Hint

# Sample Input
```
3
ABBCAAB
AABBACC
AAPPCCSS
```
# Sample Output
```
AABBACC
```

