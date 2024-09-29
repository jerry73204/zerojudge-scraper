## Task Description

注意:這題放寬了g278時限，題目完全相同。

題敘來自g278:

在一個美食博覽會上，有 n 個攤位在販售美食，已知每個攤位只會販售一種美食，且他們販售的美食依序是 a1,a2,…,an，其中可能會有某些攤位販售相同種類的美食。

國王及大臣們總共 k 人要依序品嚐所有美食，已知每位品嚐員會選擇一段連續的攤位進行試吃，而每個人都不想要試吃到同一種自己曾經吃過的美食，因此一位品嚐員所選到的範圍不能有同一種美食重複出現。另外，品嚐員們都不喜歡被別人打擾用餐，所以任意兩個品嚐員所選到的連續區間必須是沒有重疊的。

給你 n 和 k，以及這 n 個攤位分別販售的美食編號，請計算出這些試吃員們總共最多可以吃到幾攤的美食？

 

測資範圍:

對於所有測資:n,k( 1 ≤ n ≤1e6, 1 ≤ k ≤ 20,1 ≤ n × k ≤ 5e6 )

subtask 1(50%): k = 1，也就是剛好有個試吃員

subtask 2(50%): 無其他限制

## Input Format

<p>第一行輸入兩個正整數<span style="color:#ff9900">&nbsp;</span><span style="color:#ff6600">n</span>,<span style="color:#ff6600">k</span>，代表有&nbsp;<span style="color:#ff6600">n</span>&nbsp;個攤位和&nbsp;<span style="color:#ff6600">k</span>&nbsp;個試吃員。</p>
<p>接下來有&nbsp;<span style="color:#ff6600">n</span>&nbsp;個數字代表每個攤位各別賣哪一種美食，(1 ≤&nbsp;<span style="color:#ff6600">種類編號ai&nbsp;</span>≤ 1e5)</p>
<p>&nbsp;</p>

## Output Format

<p>&nbsp;輸出一個正整數，代表試吃員最後吃到的攤位總和最大值。</p>

## Sample Input 1

    5 1
    1 2 1 3 1

## Sample Output 1

    3

## Sample Input 2

    10 3
    1 7 1 3 1 4 4 2 7 4

## Sample Output 2

    8

## Sample Input 3

    2 2
    1 2

## Sample Output 3

    2