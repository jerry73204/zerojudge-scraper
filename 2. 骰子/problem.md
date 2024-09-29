## Task Description

給定
n
個骰子排成一列，一開始都是點數
1
朝上，點數
4
朝前，點數
2
朝右 (如下左圖所示)，另外骰子的展開圖如下右圖所示。



接下來有
m
次修改操作，每個操作包含兩個整數
a
,
b

* 若
a
,
b
都是正整數，交換編號
a
與編號
b
的骰子的位置。
* 若
b
為
−1
，將編號
a
的骰子向前旋轉。
* 若
b
為
−2
，將編號
a
的骰子向右旋轉。

在
m
次操作結束之後，依序輸出編號
1
到編號
n
的骰子朝上的點數。

## Input Format

<p>第一行包含兩個正整數 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="20"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c28"></mjx-c></mjx-mo><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c32"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="2"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c29"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi><mo>,</mo><mi>m</mi><mo stretchy="false">(</mo><mn>1</mn><mo>≤</mo><mi>n</mi><mo>≤</mo><mn>20</mn><mo>,</mo><mn>1</mn><mo>≤</mo><mi>m</mi><mo>≤</mo><mn>100</mn><mo stretchy="false">)</mo></math></mjx-assistive-mml></mjx-container>。</p>
<p>接下來 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="21"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>m</mi></math></mjx-assistive-mml></mjx-container> 行每行有兩個整數，第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="22"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 行的兩個正整數表示第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="23"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 次操作。</p>

## Output Format

<p>在一行輸出 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="24"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 個數字以空格分隔，第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="25"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 個數字表示編號 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="26"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 的骰子最後朝上的點數。</p>

## Sample Input 1

    1 2
    1 -2
    1 -1

## Sample Output 1

    3

## Sample Input 2

    3 3
    2 -1
    3 -2
    3 1

## Sample Output 2

    5 3 1