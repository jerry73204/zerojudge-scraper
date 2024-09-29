## Task Description

幼稚園的絲絲很喜歡跟哥哥玩猜拳，因為這是他少數有機會贏哥哥的遊戲。每天只要一回家，絲絲就要哥哥陪他猜拳。
為了戰勝哥哥，絲絲每天在幼稚園時都會花好多時間研究出拳的策略，並將預計要出的拳寫在紙上
但是哥哥上了國中以後功課越來越多，沒有空先思考要出什麼拳，於是哥哥決定根據絲絲出的拳來決定該如何出拳。

每天哥哥只要決定第一次猜拳的狀況 F，接下來他的猜拳策略如下：

* 如果絲絲連續兩輪出了一樣的拳，下一輪他就會出打敗絲絲前兩輪的拳。
* 否則，他下一輪會出跟絲絲前一輪一樣的拳。

請你寫一個程式模擬兩人遊戲過程與結果。

 

## Input Format

<p>第一行輸入哥哥第一輪要出的拳 F。<br>第二行輸入妹妹準備的數量 N。<br>第三行依序輸入妹妹準備出的拳 y<sub>1</sub>, y<sub>2</sub>&nbsp;...&nbsp;y<sub>N</sub>，以空格隔開。<br><br></p>
<p>測資範圍如下：</p>
<ul><li>所有的出拳皆為 0, 2, 5（0指石頭，2指剪刀，5指布）</li><li>N≤10</li></ul>
<p>本題包含三個子題組，每個子題組配分如下：</p>
<ul><li>第 1 子題組共 20 分： N=1</li><li>第 2 子題組共 20 分： N=2，y<sub>1</sub>≠y<sub>2</sub>。</li><li>第 3 子題組共 60 分： 無額外限制。</li></ul>

## Output Format

<p>輸出有一行，依序輸出哥哥每一回合猜的拳，以空格隔開。並在冒號後輸出第幾回合分出勝負。</p>
<ul><li>若在第 k 輪時哥哥贏了，輸出 <code>: Won at round k</code></li><li>若在第 k 輪時哥哥輸了，輸出 <code>: Lost at round k</code></li><li>若比完 N 輪仍然平手，輸出 <code>: Drew at round N</code></li></ul>

## Sample Input 1

    0
    4
    2 5 0 2

## Sample Output 1

    0 : Won at round 1

## Sample Input 2

    2
    2
    2 0

## Sample Output 2

    2 2 : Lost at round 2

## Sample Input 3

    5
    4
    5 5 0 0

## Sample Output 3

    5 5 2 : Lost at round 3

## Sample Input 4

    5
    6
    5 5 2 2 0 0

## Sample Output 4

    5 5 2 2 0 0 : Drew at round 6