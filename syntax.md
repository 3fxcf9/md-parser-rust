# Heading

## Basic syntax

paragraph OK

**bold** OK
_italic_ or _italic_ OK
..underline.. OK
~~strikethrough~~ OK
||highlighted|| OK

~ nbsp OK
~: nnbsp

[link](url)
((footnote))
{{sidenote}}

`inline code` OK and

```lang
code block OK
```

$inline math$ OK
\[
display math OK
\]

## Lists

- dash OK

* dot

- star

> vartriangleright

-> rightarrow

=> (implies) / <= (impliedby)

~ auto (> then \* then + then -)

=== filled hline OK
--- dashed hline OK
... dotted hline OK
^^^ sawtooth hline OK

## Environments

### Syntax

%name params

%

### Example

%thm Caractérisation du rang par extraction de matrice inversible
...
%

### Standard environments

- thm [name]
- cor [name]
- lemma [name]
- rem
- eg
- exo (trouver equivalent en anglais)
- fold
- conceal / block <level/categ>

%env
text

    %env
    text

        %env
        text
            %env
            text
            %
        %
    %

%

%env
text
%%env
text
%%%env
text
%%%%env
text
%%%%
%%%
%%
%
