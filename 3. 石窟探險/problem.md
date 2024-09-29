## Task Description

有一組探險隊要去一個樹狀結構的石窟內探險，該石窟內有
n
個石室，第
i
個石室有一個編號
ai
，若
ai
為偶數則會有
2
條分支(左分支和右分支)，若
ai
為奇數則有
3
條分支(左分支、中分支和右分支)。

探險隊想要紀錄這個石窟的結構，每次只要第一次走到一個新的石室，就會將該石室的編號記錄在紙上，並由左到右依序走訪該石室的分支們，若走到一條死路則會在紙上紀錄一個數字
0
，若該石室已經走完所有分支則退回到上一個石室，走訪完整個石窟後在紙上得到上一個數字序列。

探險隊回到基地後忘記計算了這個石窟內所有相鄰的石室編號相差取絕對值的總和，請幫助探險隊從紙上的序列推算出該數值。

## Input Format

<p>輸入一個整數個數不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="8"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-mn class="mjx-n" size="s"><mjx-c class="mjx-c36"></mjx-c></mjx-mn></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>10</mn><mn>6</mn></msup></math></mjx-assistive-mml></mjx-container> 的整數序列，石室的編號不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="9"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-mn class="mjx-n" size="s"><mjx-c class="mjx-c35"></mjx-c></mjx-mn></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>10</mn><mn>5</mn></msup></math></mjx-assistive-mml></mjx-container>，並保證造出來的樹深度不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="10"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c34"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>40</mn></math></mjx-assistive-mml></mjx-container>。<br><br>子題配分<br>(20%): 石室數量最多不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="11"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-mn class="mjx-n"><mjx-c class="mjx-c32"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><mn>20</mn></math></mjx-assistive-mml></mjx-container> 個，編號均為偶數<br>(20%): 石室數量最多不超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="12"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-mn class="mjx-n" size="s"><mjx-c class="mjx-c33"></mjx-c></mjx-mn></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>10</mn><mn>3</mn></msup></math></mjx-assistive-mml></mjx-container> 個<br><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">(60%): </span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">石室最大深度不超過</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">40</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">層，而且編號和石室編號不超過</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif"> <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 119.6%; position: relative;" tabindex="0" ctxtmenu_counter="13"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c31"></mjx-c><mjx-c class="mjx-c30"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.393em;"><mjx-mn class="mjx-n" size="s"><mjx-c class="mjx-c35"></mjx-c></mjx-mn></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>10</mn><mn>5</mn></msup></math></mjx-assistive-mml></mjx-container>，答案可能會超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 119.6%; position: relative;" tabindex="0" ctxtmenu_counter="14"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c32"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.363em;"><mjx-texatom size="s" texclass="ORD"><mjx-mn class="mjx-n"><mjx-c class="mjx-c33"></mjx-c><mjx-c class="mjx-c31"></mjx-c></mjx-mn></mjx-texatom></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>2</mn><mrow data-mjx-texclass="ORD"><mn>31</mn></mrow></msup></math></mjx-assistive-mml></mjx-container>。</span><span style="font-family:'-apple-system' , 'blinkmacsystemfont' , 'segoe ui' , 'roboto' , 'oxygen' , 'ubuntu' , 'cantarell' , 'open sans' , 'helvetica neue' , sans-serif">&nbsp;</span></p>

## Output Format

<p>輸出一個整數代表這個石窟內所有相鄰的石室編號相差取絕對值的總和，答案大小有可能會超過 <mjx-container class="MathJax CtxtMenu_Attached_0" jax="CHTML" style="font-size: 116.6%; position: relative;" tabindex="0" ctxtmenu_counter="15"><mjx-math class="MJX-TEX" aria-hidden="true"><mjx-msup><mjx-mn class="mjx-n"><mjx-c class="mjx-c32"></mjx-c></mjx-mn><mjx-script style="vertical-align: 0.363em;"><mjx-texatom size="s" texclass="ORD"><mjx-mn class="mjx-n"><mjx-c class="mjx-c33"></mjx-c><mjx-c class="mjx-c31"></mjx-c></mjx-mn></mjx-texatom></mjx-script></mjx-msup></mjx-math><mjx-assistive-mml unselectable="on" display="inline"><math xmlns="http://www.w3.org/1998/Math/MathML"><msup><mn>2</mn><mrow data-mjx-texclass="ORD"><mn>31</mn></mrow></msup></math></mjx-assistive-mml></mjx-container>。</p>

## Sample Input 1

    2 6 0 8 14 0 0 0 10 0 4 0 0

## Sample Output 1

    26

## Sample Input 2

    5 2 10 0 0 0 8 0 0 17 0 0 0

## Sample Output 2

    26