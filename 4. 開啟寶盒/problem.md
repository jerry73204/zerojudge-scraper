## Task Description

已知有
n
個寶盒編號
0
到
n−1
以及
m
種鑰匙編號
0
到
m−1
。一開始你有
t
種鑰匙分別為
x1,⋯,xt
。

每一個寶盒要打開都需要同時擁有
k
種鑰匙，第
i
個寶盒分別需要
ri1,ri2,⋯,rik
種類的鑰匙。每個寶盒打開後都會得到
k
種鑰匙，第
i
個寶盒打開後分別會得到
si1,si2,⋯,sik
種類的鑰匙，當拿到新的鑰匙之後可以繼續開啟新的寶盒。保證寶盒內的鑰匙不會重複，並且每種鑰匙可以開啟的寶盒數量不超過
60
。

請輸出最多可以開啟多少個寶盒。

子問題一 (20%)
k=1
,
1≤n,m≤100

子問題二 (20%)
k=1
,
1≤n,m≤105

子問題三 (60%)
1≤k≤5
,
1≤n,m≤105

## Input Format

<p>第一行輸入三個正整數 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="21"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D458 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c28"></mjx-c></mjx-mo><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="2"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-msup space="4"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-mn class="mjx-n" size="s"><mjx-c class="mjx-c35"></mjx-c></mjx-mn></mjx-script></mjx-msup><mjx-mo class="mjx-n"><mjx-c class="mjx-c2C"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="2"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D458 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c35"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c29"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi><mo>,</mo><mi>m</mi><mo>,</mo><mi>k</mi><mo stretchy="false">(</mo><mn>1</mn><mo>≤</mo><mi>n</mi><mo>,</mo><mi>m</mi><mo>≤</mo><msup><mn>10</mn><mn>5</mn></msup><mo>,</mo><mn>1</mn><mo>≤</mo><mi>k</mi><mo>≤</mo><mn>5</mn><mo stretchy="false">)</mo></math></mjx-assistive-mml></mjx-container> 代表有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="22"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 個寶盒，<mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="23"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>m</mi></math></mjx-assistive-mml></mjx-container> 種鑰匙以及每個寶盒需要的鑰匙數量 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="24"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D458 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>k</mi></math></mjx-assistive-mml></mjx-container>。</p>
<p>接下來輸入一行，第一個數字 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="25"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D461 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>t</mi></math></mjx-assistive-mml></mjx-container> 代表一開始有的鑰匙種類數量，後面有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="26"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D461 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>t</mi></math></mjx-assistive-mml></mjx-container> 個數字代表這些鑰匙分別的種類。</p>
<p>接下來有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="27"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 行，每一行有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="28"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D458 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>k</mi></math></mjx-assistive-mml></mjx-container> 個數字，代表要開啟第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="29"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 個寶盒的第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="30"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D457 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>j</mi></math></mjx-assistive-mml></mjx-container> 種鑰匙為 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="31"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msub><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45F TEX-I"></mjx-c></mjx-mi><mjx-script style="vertical-align: -0.15em;"><mjx-texatom size="s" texclass="ORD"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D457 TEX-I"></mjx-c></mjx-mi></mjx-texatom></mjx-script></mjx-msub></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mi>r</mi><mrow data-mjx-texclass="ORD"><mi>i</mi><mi>j</mi></mrow></msub></math></mjx-assistive-mml></mjx-container>。</p>
<p>最後有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="32"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 行，每一行有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="33"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D458 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>k</mi></math></mjx-assistive-mml></mjx-container> 個數字，代表開啟第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="34"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 個寶盒後得到的第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="35"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D457 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>j</mi></math></mjx-assistive-mml></mjx-container> 種鑰匙為 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="36"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msub><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D460 TEX-I"></mjx-c></mjx-mi><mjx-script style="vertical-align: -0.15em;"><mjx-texatom size="s" texclass="ORD"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D457 TEX-I"></mjx-c></mjx-mi></mjx-texatom></mjx-script></mjx-msub></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mi>s</mi><mrow data-mjx-texclass="ORD"><mi>i</mi><mi>j</mi></mrow></msub></math></mjx-assistive-mml></mjx-container>。</p>

## Output Format

<p>輸出一個正整數，代表可以開啟的寶盒數量。</p>

## Sample Input 1

    5 5 1
    2 0 1
    0
    2
    4
    3
    1
    1
    2
    4
    3
    3

## Sample Output 1

    3

## Sample Input 2

    10 8 2
    3 6 5 2
    5 6
    2 7
    2 0
    4 5
    5 1
    3 0
    4 2
    2 4
    5 3
    5 6
    5 0
    0 6
    1 7
    3 2
    2 1
    7 3
    4 7
    4 5
    4 1
    7 5

## Sample Output 2

    5