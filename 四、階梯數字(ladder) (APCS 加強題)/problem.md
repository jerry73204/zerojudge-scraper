## Task Description

阿明最近在學習程式語言，他對一些特別的正整數很有興趣，他發現一些十進數的每一位數已排好順序，從高位數往低位數看過去，每一位數字只會相等或變大，例如：9、234、777、11222233 等數字都有這性質，他稱這些數字為階梯數字。給定一正整數
N
，阿明想知道不大於N的階梯數字總共有幾個，請注意本題只算正整數，所以 0 不算階梯數字，而且階梯數字不會以 0 開始。請幫阿明寫計算階梯數字的個數。

## Input Format

<p>輸入是多行以 EOF 結尾，每行一個正整數 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="1"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mstyle style="color: black;"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D441 TEX-I"></mjx-c></mjx-mi></mjx-mstyle></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mstyle mathcolor="black"><mi>N</mi></mstyle></math></mjx-assistive-mml></mjx-container>。</p>

## Output Format

<p>對於每行輸入，輸出不大於 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="2"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mstyle style="color: black;"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D441 TEX-I"></mjx-c></mjx-mi></mjx-mstyle></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mstyle mathcolor="black"><mi>N</mi></mstyle></math></mjx-assistive-mml></mjx-container> 的階梯數字總個數於一行，因為答案可能很大，請輸出模 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="3"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mstyle style="color: black;"><mjx-texatom texclass="ORD"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-mn class="mjx-n" size="s"><mjx-c class="mjx-c39"></mjx-c></mjx-mn></mjx-script></mjx-msup><mjx-mo class="mjx-n" space="3"><mjx-c class="mjx-c2B"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="3"><mjx-c class="mjx-c37"></mjx-c></mjx-mn></mjx-texatom></mjx-mstyle></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mstyle mathcolor="black"><mrow data-mjx-texclass="ORD"><msup><mn>10</mn><mn>9</mn></msup><mo>+</mo><mn>7</mn></mrow></mstyle></math></mjx-assistive-mml></mjx-container> 之後的結果。</p>

## Sample Input

    25
    23456
    54321
    88888888

## Sample Output

    22
    1365
    1875
    24301