## Task Description



## Input Format

<p>輸入只有一行，共三個整數值，整數間以一個空白隔開。<br>第一個整數代表 a，第二個整數代表 b，這兩數均為非負的整數。<br>第三個整數代表邏輯運算的結果，只會是 0 或 1。</p>

## Output Format

<p>輸出可能得到指定結果的運算，若有多個，輸出順序為AND、OR、XOR，每個可能的運算單獨輸出一行，每行結尾皆有換行。</p>
<p>若不可能得到指定結果，輸出IMPOSSIBLE。</p>
<p>（注意輸出時所有英文字母均為大寫字母。）</p>

## Sample Input

    範例一：
    0 0 0 
    
    範例二：
    1 1 1 
    
    範例三：
    3 0 1
    
    範例四：
    0 0 1  

## Sample Output

    範例一：
    AND 
    OR 
    XOR
    
    範例二：
    AND 
    OR 
    
    範例三：
    OR 
    XOR 
    
    範例四：
    IMPOSSIBLE 