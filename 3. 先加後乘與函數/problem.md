## Task Description

給一個運算式，運算式的內容由數字、
+
、
∗
和 某個函式
f()
所組成，除了函式
f()
以外不會有額外的括號。請將此運算式依照 先加後乘 的方式運算。

函式
f(x1,x2,x3,x4,...)
定義為從這個不定長度的參數
x1,x2,x3,x4,...
中的最大值扣掉最小值。例如
f(3,6,2)=6−2=4
、
f(3)=0
。

## Input Format

<p>輸入一個運算式，保證長度不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="8"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c35"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>500</mn></math></mjx-assistive-mml></mjx-container>，出現在運算式內的數字介於 0 到 200 之間，除了函式 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="9"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D453 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c28"></mjx-c></mjx-mo><mjx-mo class="mjx-n"><mjx-c class="mjx-c29"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>f</mi><mo stretchy="false">(</mo><mo stretchy="false">)</mo></math></mjx-assistive-mml></mjx-container> 之外不會出現多餘的括號，並且運算式一定合法。</p>
<p>(30 分): 運算式只包含數字、<mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="10"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mo class="mjx-n"><mjx-c class="mjx-c2B"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo>+</mo></math></mjx-assistive-mml></mjx-container> 和 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="11"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mo class="mjx-n"><mjx-c class="mjx-c2217"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mo>∗</mo></math></mjx-assistive-mml></mjx-container><br>(70 分): 無其他限制</p>

## Output Format

<p>輸出運算式的計算結果，此題運算過程和答案可能超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="12"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c32"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.363em;"><mjx-texatom size="s" texclass="ORD"><mjx-mn class="mjx-n"><mjx-c class="mjx-c33"></mjx-c><mjx-c class="mjx-c31"></mjx-c></mjx-mn></mjx-texatom></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>2</mn><mrow data-mjx-texclass="ORD"><mn>31</mn></mrow></msup></math></mjx-assistive-mml></mjx-container> 但不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="13"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-texatom size="s" texclass="ORD"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c37"></mjx-c></mjx-mn></mjx-texatom></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>10</mn><mrow data-mjx-texclass="ORD"><mn>17</mn></mrow></msup></math></mjx-assistive-mml></mjx-container>。</p>

## Sample Input 1

    2+3*1+2+1

## Sample Output 1

    20

## Sample Input 2

    12+f(13,2+f(8,1+2*3),1+1*f(20,4)*f(2))*2

## Sample Output 2

    50

## Sample Input 3

    f(0)

## Sample Output 3

    0