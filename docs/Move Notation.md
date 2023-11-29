# Move Notation
## Board layout
```
   a  b  c  d  .. cols
  .-----------.
1 |__|__|__|__|
2 |__|__|__|__|
3 |__|__|__|__|
4 ;__|__|__|__;
:
rows
```

## Syntax
```bnf
column      := <letter: a...cols>
row         := <number: 1...rows>
coordinate  := <column> <row>
player      := 'B' | 'R'
take        := <coordinate> ['<' <coodinate>]
slide       := (<column> ['\'']) | (<row> ['\''])
normal-move := <slide> [':' <take>]
win-move    := <normal-move> '#'
move        := <normal-move> | <win-move>
```

*Not in proper BNF format.*
*`'` = Direction modifier. Default direction is* ***right/down.***

### Examples
- `B3':d2<c2`: Black slides row 3 up, taking d2, replacing with c2.
- `Ra`: Red slides column a down.
- `Bc:b3#`: Black slides column c down, taking b3, winning the game.
