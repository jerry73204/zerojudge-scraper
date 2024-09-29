## Task Description

三角形除了是最基本的多邊形外，亦可進一步細分為鈍角三形、直角三角形及銳角三角形。若給定三個線段的長度，透過下列公式運算，即可得知此三線段能否構成三角形，亦可判斷是直角、銳角和鈍角三角形。

提示：若a、b、c為三個線段的邊長，且c為最大值，則

若 a+b ≦ c　　　　　，三線段無法構成三角形

若 a×a+b×b ＜ c×c　　，三線段構成鈍角三角形(Obtuse triangle)

若 a×a+b×b ＝ c×c　　，,三線段構成直角三角形(Right triangle)

若 a×a+b×b ＞ c×c　　，三線段構成銳角三角形(Acute triangle)

請設計程式以讀入三個線段的長度判斷並輸出此三線段可否構成三角形？若可，判斷 並輸出其所屬三角形類型。

 

原題pdf檔

 

## Input Format

<p>輸入僅一行包含三正整數，三正整數皆小於 30,001，兩數之間有一空白。</p>

## Output Format

<p>輸出共有兩行，第一行由小而大印出此三正整數，兩字之間以一個空白格間格，最後 一個數字後不應有空白；第二行輸出三角形的類型：</p>
<p>　　若無法構成三角形時輸出「No」；</p>
<p>　　若構成鈍角三形時輸出「Obtuse」；</p>
<p>　　若直角三形時輸出「Right」；</p>
<p>　　若銳角三形時輸出「Acute」。</p>
<p>&nbsp;</p>

## Sample Input 1

    3 4 5

## Sample Output 1

    3 4 5
    Right

## Sample Input 2

    101 100 99

## Sample Output 2

    99 100 101
    Acute

## Sample Input 3

    10 10 100

## Sample Output 3

    10 10 100
    No