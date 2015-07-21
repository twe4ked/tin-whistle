# Tin Whistle

Takes ABC notation from STDIN and prints out tin whistle diagrams.

## Example

    $ crystal build tin-whistle.cr && ./tin-whistle < examples/scale.abc
    d  e  f# g  |  a  b  c# D  |  E  F# G  A  |  B
                |           .  |  .  .  .  .  |  .
    ●  ●  ●  ●  |  ●  ●  ○  ●  |  ●  ●  ●  ●  |  ●
    ●  ●  ●  ●  |  ●  ○  ○  ●  |  ●  ●  ●  ●  |  ○
    ●  ●  ●  ●  |  ○  ○  ○  ●  |  ●  ●  ●  ○  |  ○
    ●  ●  ●  ○  |  ○  ○  ○  ●  |  ●  ●  ○  ○  |  ○
    ●  ●  ○  ○  |  ○  ○  ○  ●  |  ●  ○  ○  ○  |  ○
    ●  ○  ○  ○  |  ○  ○  ○  ●  |  ○  ○  ○  ○  |  ○
