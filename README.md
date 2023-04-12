g w w w r
g g y w o

0   1   2
g w w w r  0
g   ?   ?
y ? ? ? ?  1
w   ?   ?
o ? ? ? ?  2

x x x x x
x   x   x
x x x x x
x   x   x
x x x x x


Go through the row index that matches our current index first.
Remove any words that can be ruled out based on the results.

Go through the col index that matches the current index next (add 3 to index)
Remove any words that can be ruled out based on the results.

Go to the next index in the loop and rule out any words based on the position of
results from the other columns and rows.    


Result row 0
Row 0 *
Row 1
Row 2
Column 0 *
Column 1
Column 2

Result column 1
Row 0 *
Row 1
Row 2
Column 0 *
Column 1
Column 2


x x  w  x x
x    w    x
x x  w  x x
x    w    x
r r  wr r y


word_list_index   = 1
result_list_index = 2
result_index = 4

if word_list_index * 2 == result_index (result is IN the word)
    if word_list[result_list_index * 2] has the yellow letter

x x  w  x x
x    w    x
x x  w  x x
x    w    x
r r  wy r r

word_list_index   = 1
result_list_index = 2
result_index = 2

if word_list_index * 2 == result_index (result is IN the word)
    if word_list[result_list_index * 2] has the yellow letter

x x  w  x x
x    w    x
x x  w  x x
x    w    x
y r  w  r r

word_list_index   = 1
result_list_index = 2
result_index = 0

if word_list_index * 2 == result_index (result is IN the word)
    if word_list[result_list_index * 2] has the yellow letter


w  x x x x
w    x   x
wr y r r r
w    x   x
w  x x x x

word_list_index   = 0
result_list_index = 1
result_index      = 1

if word_list_index * 2 == result_index (result is IN the word)
    if word_list[result_list_index * 2] has the yellow letter