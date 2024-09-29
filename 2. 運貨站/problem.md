## Task Description

運貨站要管理
n
個五種不同形狀的貨物，下圖標示出貨物的形狀以及對應的英文代碼。


現在這
n
個貨物要按照順序堆放在一個容量大小為
R×C
的倉庫內，第
i
個貨物的形狀為
ti
，並且和倉庫的頂部距離為
yi
(見圖ㄧ)。貨物堆放置倉庫內時必須維持和倉庫頂端的高度由右向左推到不能前進為止，並且過程中不行將貨物的方向做旋轉。若有一個貨物不能完整放入倉庫內，則該貨物會被貨運站丟棄。

請輸出依序放完這
n
個貨物後，倉庫內有多少剩餘空格，以及被丟棄的貨物有幾個。


(圖一: 該貨物類別為 B, 並且離倉庫頂端距離為 2)

保證輸入內貨物距離倉庫頂部的高度不會讓貨物底部低於地面，並且不會有任何貨物卡在倉庫門口的情形。

## Input Format

<p>第一行輸入三個數字 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="7"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c28"></mjx-c></mjx-mo><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c33"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c29"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>R</mi><mo stretchy="false">(</mo><mn>1</mn><mo>≤</mo><mi>R</mi><mo>≤</mo><mn>30</mn><mo stretchy="false">)</mo></math></mjx-assistive-mml></mjx-container>, <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="8"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D436 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c28"></mjx-c></mjx-mo><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D436 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c35"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c29"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>C</mi><mo stretchy="false">(</mo><mn>1</mn><mo>≤</mo><mi>C</mi><mo>≤</mo><mn>50</mn><mo stretchy="false">)</mo></math></mjx-assistive-mml></mjx-container>, <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="9"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n"><mjx-c class="mjx-c28"></mjx-c></mjx-mo><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c></mjx-mn><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="4"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="4"><mjx-c class="mjx-c2264"></mjx-c></mjx-mo><mjx-mn class="mjx-n" space="4"><mjx-c class="mjx-c32"></mjx-c><mjx-c class="mjx-c30"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-mo class="mjx-n"><mjx-c class="mjx-c29"></mjx-c></mjx-mo></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi><mo stretchy="false">(</mo><mn>1</mn><mo>≤</mo><mi>n</mi><mo>≤</mo><mn>200</mn><mo stretchy="false">)</mo></math></mjx-assistive-mml></mjx-container>，代表倉庫大小為 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="10"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D445 TEX-I"></mjx-c></mjx-mi><mjx-mo class="mjx-n" space="3"><mjx-c class="mjx-cD7"></mjx-c></mjx-mo><mjx-mi class="mjx-i" space="3"><mjx-c class="mjx-c1D436 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>R</mi><mo>×</mo><mi>C</mi></math></mjx-assistive-mml></mjx-container> 以及有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="11"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 個貨物。接下來有 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="12"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D45B TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>n</mi></math></mjx-assistive-mml></mjx-container> 行，第 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="13"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mi>i</mi></math></mjx-assistive-mml></mjx-container> 行有一個大寫英文字母 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="14"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msub><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D461 TEX-I"></mjx-c></mjx-mi><mjx-script style="vertical-align: -0.15em;"><mjx-mi class="mjx-i" size="s"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-script></mjx-msub></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mi>t</mi><mi>i</mi></msub></math></mjx-assistive-mml></mjx-container> 和一個數字 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="15"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msub><mjx-mi class="mjx-i"><mjx-c class="mjx-c1D466 TEX-I"></mjx-c></mjx-mi><mjx-script style="vertical-align: -0.15em;"><mjx-mi class="mjx-i" size="s"><mjx-c class="mjx-c1D456 TEX-I"></mjx-c></mjx-mi></mjx-script></mjx-msub></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msub><mi>y</mi><mi>i</mi></msub></math></mjx-assistive-mml></mjx-container> 代表貨物的種類以及和倉庫頂部的距離，貨物種類只會是 A 到 E 的大寫字母。<br><br>子題配分<br>- (20%) : 只會出現Ｂ類型<br><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">- (40%) : </span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">只會出現Ａ</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">,</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">Ｂ</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">,</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">Ｃ類型<br></span>- (40%) : ５種類型都會出現</p>

## Output Format

<p>輸出倉庫剩餘的空格數量，以及被丟棄的貨物數量。</p>

## Sample Input 1

    5 4 6
    B 0
    B 3
    B 1
    B 3
    B 1
    B 2

## Sample Output 1

    8 2

## Sample Input 2

    5 6 6
    C 1
    A 1
    E 0
    E 0
    B 0
    A 0

## Sample Output 2

    13 2