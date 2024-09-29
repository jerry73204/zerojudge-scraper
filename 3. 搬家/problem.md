## Task Description

忍者龜住在下水道中，他們正在準備搬家。下水道由
n
x
m
的矩陣表示，其中不同的字元代表著水管的開口方向。如果兩個水管可以互相連接，它們屬於同一個連通塊。你需要找出最大的連通塊的大小。

其中，X 代表十字架，而 H、I、F、7、L 分別代表其他不同形狀的水管。0 字元代表沒有水管連接的地方。

請注意，在某個連通塊內的水管可以連接，而不同連通塊的水管不會相互連接。

 

下面是一些可能的水管形狀：

水管的開口方向與字元對應關係
F: 右和下
H: 左和右
7: 左和下
I: 上和下
X: 上、下、左和右
L: 右和上
J: 左和上
0: 沒有水管

## Input Format

<p>第一行包含兩個整數：<mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="2"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 和 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="3"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>m</mi></math></mjx-assistive-mml></mjx-container>，以空格分隔。它們分別代表下水道矩陣的行數和列數。</p>
<p>接下來的 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="4"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 行，每行包含 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="5"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>m</mi></math></mjx-assistive-mml></mjx-container> 個字元，用於表示下水道的樣子。這些字元描述了各種水管的不同形狀，以及沒有水管的地方。不同的字元代表不同的水管形狀，如 H、I、F、7、L 和 0。水管形狀的解釋在題目敘述中有詳細說明。</p>
<p>請注意，連接在一起的相同形狀的水管屬於同一個連通塊，但不同連通塊之間的水管是不會相互連接的。</p>
<p>所有測試資料皆滿足 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="6"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c35"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>1</mn><mo>≤</mo><mi>n</mi><mo>,</mo><mi>m</mi><mo>≤</mo><mn>500</mn></math></mjx-assistive-mml></mjx-container></p>
<p><strong>子題分數：</strong></p>
<ul><li>60%：不會有 X 出現在下水道。</li><li>40%：一般情況。</li></ul>

## Output Format

<p>輸出出最大連通塊的大小。</p>

## Sample Input 1

    3 4
    FHH7
    IIII
    LHHJ

## Sample Output 1

    10

## Sample Input 2

    4 7
    0F70000
    FXJ0000
    II700X7
    LJ0HHLJ

## Sample Output 2

    9