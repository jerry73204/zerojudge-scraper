# Task Description
給定一個大小為 $K$ 個字母的集合和字串 $S$，求出在使用該集合所組出長度為 $L$ 字串中，不為字串 $S$ 子字串的最小字典序字串為何。

例如字母集合 $\\{\\text{a}, \\text{c}, \\text{m}\\}$，其能組出長度為 $2$ 的字串字典序由小到大排序為 $\\text{aa} \< \\text{ac} \< \\text{am} \< \\text{ca} \< \\text{cc} \< \\text{cm} \< \\text{ma} \< \\text{mc} \< \\text{mm}$。字串 $S$ 為 $\\text{accaamcm}$，因此 $\\text{ma}$ 為不在 $S$ 內的最小字典序字串。
# Input Format
第一行為一個長度為 $K (1 \\le K \\le 10)$ 的小寫字母字串代表字母集合，保證字元不重複且照字元由小到大排序。

第二行為一個正整數 $L (1 \\le L \\le 8, 1 \\le K^L \\le 6 \\times 10^5)$。

第三行為小寫英文字串 $S (L \\le |S| \\le 5 \\times 10^5)$。

(20 分): $|S| = 1000$  
(80 分): 無限制
# Output Format
輸出滿足題目要求的最小字典序字串
# Hint
範測 1: 

字母集合 $\\{\\text{a}, \\text{c}, \\text{m}\\}$，其能組出長度為 $2$ 的字串字典序由小到大為 $\\text{aa} \< \\text{ac} \< \\text{am} \< \\text{ca} \< \\text{cc} \< \\text{cm} \< \\text{ma} \< \\text{mc} \< \\text{mm}$。字串 $S$ 為 $\\text{accaamcm}$，$\\text{ma}$ 為不在 $S$ 內的最小字典序字串。

範測 2:

字母集合 $\\{\\text{d}, \\text{p}\\}$，其能組出長度為 $3$ 的字串字典序由小到大為 $\\text{ddd} \< \\text{ddp} \< \\text{dpd} \< \\text{dpp} \< \\text{pdd} \< \\text{pdp} \< \\text{ppd} \< \\text{ppp}$。字串 $S$ 為 $\\text{dddppdpd}$，$\\text{pdd}$ 為不在 $S$ 內的最小字典序字串。
# Sample Input 1
```
acm
2
accaamcm
```
# Sample Output 1
```
ma

                     ```
# Sample Input 2
```
dp
3
dddppdpd
```
# Sample Output 2
```
pdd

                     ```

