# Brainfuck interpreter

## Install

```
$ cargo install bf
```

## Usage

```
$ cat > hello.bf
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
$ bf hello.bf
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
$ bf --language=abcdefgh hello.abc.bf
Hello World!
```

Even emojis.

```
$ cat > hello.emoji.bf
😀😀😀😀😀😀😀😀😂😄😀😀😀😀😂😄😀😀😄😀😀😀😄😀😀😀😄😀😁😁😁😁😃🤣😄😀😄😀😄😃😄😄😀😂😁🤣😁😃🤣😄😄😅😄😃😃😃😅😀😀😀😀😀😀😀😅😅😀😀😀😅😄😄😅😁😃😅😁😅😀😀😀😅😃😃😃😃😃😃😅😃😃😃😃😃😃😃😃😅😄😄😀😅😄😀😀😅
$ bf --language=😀😃😄😁😆😅😂🤣 "hello.emoji.bg
```

## LICENSE

MIT.
