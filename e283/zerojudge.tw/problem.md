# Task Description
小崴要來玩編碼了\~\~!!! ![](jscripts/tinymce/plugins/emoticons/img/smiley-smile.gif)
----------

這次，他打算跟你講很多字串! 

這些字串均經過特殊編碼的\~\~ 

祝你全部解讀成功!

P.S. 編碼方式: 每個字串均只由 A\~F 組成，

並由以下對照表將每一個字元轉換成長度為4的二元序列

A -\> 0 1 0 1

B -\> 0 1 1 1

C -\> 0 0 1 0

D -\> 1 1 0 1

E -\> 1 0 0 0

F -\> 1 1 0 0
# Input Format
**多筆輸入!!!**

**每筆輸入 第一行有一個 正整數N 代表此字串的長度，**

**接下來有n行，每一行給一個字元經編碼後的序列 !**

**以EOF結束\~\~**
# Output Format
**多筆輸出!!!**

**輸出原始字串**

**記得空行歐\~\~**
# Hint
第1組測資 即為範例測資  
第2組測資 N均 \<=4  
第3組測資以後 N 是 unsigned int

測資點 #05很大\~\~\~ 可能要優化 IO 小心 TLE ![](jscripts/tinymce/plugins/emoticons/img/smiley-smile.gif)

\~感謝 David 建議 測資點 #05 延長時限 至 0.3s 已重測

 \~感謝 icube 建議 測資點 #05 延長時限 至 1.0s 已重測
# Sample Input
```
1
0 1 0 1
1
0 0 1 0
2
1 0 0 0
1 1 0 0
4
1 1 0 1
1 0 0 0
0 1 1 1
1 1 0 1
```
# Sample Output
```
A
C
EF
DEBD
```

