# Tin Whistle

Takes ABC notation from STDIN and prints out tin whistle diagrams.

## Example

    $ cargo run < examples/scale.abc
    D  E  F# G  |  A  B  C# d  |  e  f# g  a  |  b
                |           .  |  .  .  .  .  |  .
    ●  ●  ●  ●  |  ●  ●  ○  ●  |  ●  ●  ●  ●  |  ●
    ●  ●  ●  ●  |  ●  ○  ○  ●  |  ●  ●  ●  ●  |  ○
    ●  ●  ●  ●  |  ○  ○  ○  ●  |  ●  ●  ●  ○  |  ○
    ●  ●  ●  ○  |  ○  ○  ○  ●  |  ●  ●  ○  ○  |  ○
    ●  ●  ○  ○  |  ○  ○  ○  ●  |  ●  ○  ○  ○  |  ○
    ●  ○  ○  ○  |  ○  ○  ○  ●  |  ○  ○  ○  ○  |  ○

## Sources

- http://damihce726.blogspot.com.au
