## Task Description

占卜籤筒有
m
支籤，每一支籤為一個由英文小寫字母組成的字串。從籤筒內抽出兩支籤，若將這兩支籤上的字串
S
和
T
連接起來形成的字串可以將該字串拆成左右兩半並且內容一樣，則抽到聖筊代表神明同意，否則神明不同意或是沒回答。

例如抽出的兩支籤上的字串分別為 piep 和 ie，則相連接起來的字串為 piepie 可以拆分左右兩半為相同的字串 pie 和 pie，但抽出的兩支籤為 foo 和 bar 時則不滿足條件。

神奇的是，若抽到的兩支籤
S
和
T
為聖筊，則不管是將
T
接在
S
後面或是順序反過來接起來，都可以是聖筊，再次說明了這兩支籤有著某種神秘力量在祝福著抽到的幸運人。例如 piep 和 ie 不管是使用 piepie 或是 iepiep 都可以拆分成兩個一樣的字串。

詢問籤筒內這
m
支籤，有幾個 pair 可以形成聖筊。相同的兩支籤組合計算一次即可。

## Input Format

<p>輸入一個正整數 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="8"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>m</mi></math></mjx-assistive-mml></mjx-container>，接下來有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="9"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>m</mi></math></mjx-assistive-mml></mjx-container> 的字串，每個字串長度最長為 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="10"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>100</mn></math></mjx-assistive-mml></mjx-container>。</p>
<p>&nbsp;</p>
<p>數字範圍</p>
<ul><li><mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="11"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c35"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>1</mn><mo>≤</mo><mi>m</mi><mo>≤</mo><mn>50000</mn></math></mjx-assistive-mml></mjx-container></li><li>籤筒內所有字串均相異</li></ul>
<p>&nbsp;</p>
<p>子題配分</p>
<ul><li>(20%): <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="12"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c32"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45A TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>2</mn><mo>≤</mo><mi>m</mi><mo>≤</mo><mn>100</mn></math></mjx-assistive-mml></mjx-container>, 字串長度只會是 10 或是 20</li><li>(20%): 字串長度只會是 10 或是 20</li><li>(60%): 無額外限制</li></ul>

## Output Format

<p>輸出一個正整數，代表有幾個 pair 滿足條件。</p>

## Sample Input 1

    3
    a
    aba
    aaa

## Sample Output 1

    1

## Sample Input 2

    5
    abyyyab
    y
    yy
    yyy
    yyyy

## Sample Output 2

    3