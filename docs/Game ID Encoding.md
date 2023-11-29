# Game ID Encoding
## Premise
Create an encoding scheme for game session IDs ("codes") that:
1. Is "word-safe"; does not contain digits that could be mistaken for vowels.
2. Does not contain digits that may be confused with one another.
3. Can encode a large range of values in minimal digits.

## Details
|                |                      |
|----------------|----------------------|
| Digit set      | 23456789CFGHJMPQRVWX |
| Code length    | 6                    |
| Possible codes | 20^6 (64,000,000)    |

**Example code:** `7FX 99R`

## Encoding table
| Value         | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 |
|---------------|---|---|---|---|---|---|---|---|---|---|----|----|----|----|----|----|----|----|----|----|
| Base 20 digit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | A  | B  | C  | D  | E  | F  | G  | H  | I  | J  |
| Code digit    | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | C | F | G  | H  | J  | M  | P  | Q  | R  | V  | W  | X  |
