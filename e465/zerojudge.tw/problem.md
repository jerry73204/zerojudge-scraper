# Task Description
你是個櫃子租借商，總共擁有 M 個櫃子。  
現在這 M 個櫃子分別被 N 個人借用，借用量分別為 (x<sub>0</sub>, x<sub>1</sub>, x<sub>2</sub>, ...x<sub>N-1</sub>) 個櫃子，  
其中 x<sub>0 +</sub> x<sub>1&nbsp;</sub>+ x<sub>2 +&nbsp;</sub>... + x<sub>N-1</sub> ≤ M  

現在你想要使用 S 個空櫃子，  
在被借用的櫃子只能夠 全退 或 全不退之下，最少需要請這 N 個人退還多少櫃子？  
也就是當有一個人借用 10 個櫃子時，不能夠只請他退還 5 個櫃子。  

舉例來說，對於 M = 20 個櫃子，  
現在分別被 5 個人借用 (1, 7, 2, 8, 2) 個櫃子，  

在需要使用 S = 14 個櫃子之下，  
最少需要請他們退還 7 + 8 = 15 個櫃子。
# Input Format
第一行有三個正整數 M、S、N，  
分別代表櫃子總數 M 、 想要使用的櫃子數 S 、 借用人數 N。  
M ≤ 100,000  
S ≤ M  
N ≤ 100,000

第二行有 N 個非負整數 x<sub>0</sub>, x<sub>1</sub>, x<sub>2</sub>, ...x<sub>N-1</sub>，  
代表每個人所借用的櫃子數量。  
其中 x<sub>0 +</sub> x<sub>1&nbsp;</sub>+ x<sub>2 +&nbsp;</sub>... + x<sub>N-1</sub> ≤ M
# Output Format
最少需要請這 N 個人退還的櫃子總數
# Hint

# Sample Input
```
20 14 5
1 7 2 8 2
```
# Sample Output
```
15
```

