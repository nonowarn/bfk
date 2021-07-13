# Brainfuck interpreter

## Install

```
$ cargo install bfk
```

## Usage

```
$ cat > hello.bf
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
$ bfk hello.bf
Hello, World!
```

## Create your own fuck

For example, Replace `+-><,.[]` with `abcdefgh`.
This is replaced Hello World program in Brainfuck.

```
aaaaaaaagcaaaagcaacaaacaaacaddddbhcacacbccagdhdbhccfcbbbfaaaaaaaffaaafccfdbfdfaaafbbbbbbfbbbbbbbbfccafcaaf
```

By providing replacing character to `language` option,
This interpreter runs it as a transformed Brainfuck program.

```
$ cat > hello.abc.bf
aaaaaaaagcaaaagcaacaaacaaacaddddbhcacacbccagdhdbhccfcbbbfaaaaaaaffaaafccfdbfdfaaafbbbbbbfbbbbbbbbfccafcaaf
$ bfk --language=abcdefgh hello.abc.bf
Hello World!
```

Even emojis.

```
$ cat > hello.emoji.bf
😀😀😀😀😀😀😀😀😂😄😀😀😀😀😂😄😀😀😄😀😀😀😄😀😀😀😄😀😁😁😁😁😃🤣😄😀😄😀😄😃😄😄😀😂😁🤣😁😃🤣😄😄😅😄😃😃😃😅😀😀😀😀😀😀😀😅😅😀😀😀😅😄😄😅😁😃😅😁😅😀😀😀😅😃😃😃😃😃😃😅😃😃😃😃😃😃😃😃😅😄😄😀😅😄😀😀😅
$ bfk --language=😀😃😄😁😆😅😂🤣 hello.emoji.bf
Hello World!
```

## LICENSE

MIT.
