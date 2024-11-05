# Task Description
<img src="./ShowImage_id_1068" height="857" width="1000" />

<img src="./ShowImage_id_1067" height="224" width="1000" />
# Input Format
第一行有一個正整數n代表樹狀圖的節點個數，節點的編號為1到n。  
接下來有n行，第i行的第一個數字k代表節點i有k個子節點，第i行接下來的k個數字就是這些子節點的編號。  
每一行的相鄰數字間以空白隔開。
# Output Format
輸出兩行各含一個整數，第一行是根節點的編號，第二行是H(T)。
# Hint
輸入包含若干筆測試資料，每一筆測試資料的執行時間限制(time limit)均為1秒，依正確通過測資筆數給分。  
測資範圍如下，其中k是每個節點的子節點數量上限：  
 第1子題組10分，1 ≤ n ≤ 4, k ≤ 3, 除了根節點之外都是葉節點。  
 第2子題組30分，1 ≤ n ≤ 1,000, k ≤ 3。  
 第3子題組30分，1 ≤ n ≤ 100,000, k ≤ 3。  
 第4子題組30分，1 ≤ n ≤ 100,000, k無限制。

非官方測資 有錯請站內信通知 感恩 0u0

p.s. 感謝 boook 幫忙生測資

原題 pdf : [https://apcs.csie.ntnu.edu.tw/files/1061028APCSImplementation.pdf](../apcs.csie.ntnu.edu.tw/files/1061028APCSImplementation.pdf)

2017.12.31 01:16am rejudge 測資加強
# Sample Input
```
範例一：
7 
0 
2 6 7 
2 1 4 
0 
2 3 2 
0 
0 

範例二：
9 
1 6 
3 5 3 8 
0 
2 1 7 
1 9 
0 
1 2 
0 
0 
```
# Sample Output
```
範例一：
5 
4 

範例二：
4 
11 
```

