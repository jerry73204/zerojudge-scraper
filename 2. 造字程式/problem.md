## Task Description

有一個長度
K
的的初始字串
S
，每個字元都是小寫的英文字母。

接下來有
Q
次的修改，每次的修改會把舊的字串重新排列成一個新的字串。
更具體來講，每次修改時會給一個
1
~
K
的排列
P=[P1,P2,...,PK]
，要將舊字串的第
i
的字元複製到新字串的第
Pi
個字元。
例子：
    - 若舊字串是 "abac"，且
P=[4,1,3,2]
，可以得到新字串 "bcaa"。
    - 若舊字串是 "bcaa"，且
P=[1,2,3,4]
，可以得到新字串 "bcaa"。
    - 若舊字串是 "bcaa"，且
P=[2,3,4,1]
，可以得到新字串 "abca"。

在
Q
的修改中，每次修改出來的新字串會被當成下一次修改中的舊字串，而第一次修改時使用的舊字串就是初始字串。

題目另外會給一個數字
R
，請依照下面定義的順序輸出
R
行，每行
Q
個字元
    - 輸出操作 1 ~
Q
的新字串的第
1
個字元
    - 輸出操作 1 ~
Q
的新字串的第
2
個字元
    - ...
    - 輸出操作 1 ~
Q
的新字串的第
R
個字元

## Input Format

<p>第一行有三個整數 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="21"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D43E TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D444 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>K</mi><mo>,</mo><mi>Q</mi><mo>,</mo><mi>R</mi></math></mjx-assistive-mml></mjx-container><br>第二行是長度 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="22"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D43E TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>K</mi></math></mjx-assistive-mml></mjx-container> 的初始字串<br>接下來有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="23"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D444 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>Q</mi></math></mjx-assistive-mml></mjx-container> 行，每行有是一個 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="24"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>1</mn></math></mjx-assistive-mml></mjx-container> ~ <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="25"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D43E TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>K</mi></math></mjx-assistive-mml></mjx-container> 的排列<br><br>(60%) <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="26"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c3D"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="2"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D43E TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D444 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c32"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>R</mi><mo>=</mo><mn>1</mn><mo>,</mo><mn>1</mn><mo>≤</mo><mi>K</mi><mo>,</mo><mi>Q</mi><mo>≤</mo><mn>20</mn></math></mjx-assistive-mml></mjx-container><br>(40%) <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="27"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D43E TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c32"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="2"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D444 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c32"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>1</mn><mo>≤</mo><mi>R</mi><mo>≤</mo><mi>K</mi><mo>≤</mo><mn>20</mn><mo>,</mo><mn>1</mn><mo>≤</mo><mi>Q</mi><mo>≤</mo><mn>20</mn></math></mjx-assistive-mml></mjx-container></p>

## Output Format

<p>請依照題目敘述輸出 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="28"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="3"><mjx-c class="mjx-cD7"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="3"><mjx-c class="mjx-c1D444 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>R</mi><mo>×</mo><mi>Q</mi></math></mjx-assistive-mml></mjx-container> 個字元</p>

## Sample Input 1

    5 4 1
    abcde
    2 1 3 5 4
    5 1 2 4 3
    4 1 2 3 5
    3 1 4 5 2

## Sample Output 1

    bacd

## Sample Input 2

    4 3 4
    abac
    4 1 3 2
    1 2 3 4
    2 3 4 1

## Sample Output 2

    bba
    ccb
    aac
    aaa