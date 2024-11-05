# Task Description
<img src="./ShowImage_id_1064" height="486" width="1000" />
# Input Format
輸入只有一行，共三個整數值，整數間以一個空白隔開。  
第一個整數代表 a，第二個整數代表 b，這兩數均為非負的整數。  
第三個整數代表邏輯運算的結果，只會是 0 或 1。
# Output Format
輸出可能得到指定結果的運算，若有多個，輸出順序為AND、OR、XOR，每個可能的運算單獨輸出一行，每行結尾皆有換行。

若不可能得到指定結果，輸出IMPOSSIBLE。

（注意輸出時所有英文字母均為大寫字母。）
# Hint
輸入包含若干筆測試資料，每一筆測試資料的執行時間限制(time limit)均為1秒，依正確通過測資筆數給分。  
其中：  
 第1子題組80分，a 和 b 的值只會是0或1。  
 第2子題組20分，0 ≤ a, b \< 10,000。

非官方測資 有錯請站內信通知 感恩 0u0

原題 pdf : [https://apcs.csie.ntnu.edu.tw/files/1061028APCSImplementation.pdf](../apcs.csie.ntnu.edu.tw/files/1061028APCSImplementation.pdf)
# Sample Input
```
範例一：
0 0 0 

範例二：
1 1 1 

範例三：
3 0 1

範例四：
0 0 1  
```
# Sample Output
```
範例一：
AND 
OR 
XOR

範例二：
AND 
OR 

範例三：
OR 
XOR 

範例四：
IMPOSSIBLE 
```

