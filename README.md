# Basic Typst AST Printer

## Build

```bash
nix develop
nix build
```

## Run

### Against Valid Table

Command:

```bash
./result/bin/typst-structure --source /path/to/valid-table.typ
```

Output:

```
Markup: 147 [
    Hash: "#",
    FuncCall: 145 [
        Ident: "table",
        Args: 140 [
            LeftParen: "(",
            Space: "\n  ",
            Named: 10 [
                Ident: "columns",
                Colon: ":",
                Space: " ",
                Int: "2",
            ],
            Comma: ",",
            Space: "\n  ",
            FuncCall: 55 [
                FieldAccess: 12 [
                    Ident: "table",
                    Dot: ".",
                    Ident: "header",
                ],
                Args: 43 [
                    LeftParen: "(",
                    Space: "\n    ",
                    Named: 8 [
                        Ident: "level",
                        Colon: ":",
                        Space: " ",
                        Int: "1",
                    ],
                    Comma: ",",
                    Space: "\n    ",
                    ContentBlock: 7 [
                        LeftBracket: "[",
                        Markup: 5 [
                            Text: "Month",
                        ],
                        RightBracket: "]",
                    ],
                    Comma: ",",
                    Space: " ",
                    ContentBlock: 9 [
                        LeftBracket: "[",
                        Markup: 7 [
                            Text: "Savings",
                        ],
                        RightBracket: "]",
                    ],
                    Comma: ",",
                    Space: "\n  ",
                    RightParen: ")",
                ],
            ],
            Comma: ",",
            Space: "\n  ",
            ContentBlock: 9 [
                LeftBracket: "[",
                Markup: 7 [
                    Text: "January",
                ],
                RightBracket: "]",
            ],
            Comma: ",",
            Space: " ",
            ContentBlock: 7 [
                LeftBracket: "[",
                Markup: 5 [
                    Escape: "\\$",
                    Text: "250",
                ],
                RightBracket: "]",
            ],
            Comma: ",",
            Space: "\n  ",
            ContentBlock: 10 [
                LeftBracket: "[",
                Markup: 8 [
                    Text: "February",
                ],
                RightBracket: "]",
            ],
            Comma: ",",
            Space: " ",
            ContentBlock: 6 [
                LeftBracket: "[",
                Markup: 4 [
                    Escape: "\\$",
                    Text: "80",
                ],
                RightBracket: "]",
            ],
            Comma: ",",
            Space: "\n  ",
            ContentBlock: 7 [
                LeftBracket: "[",
                Markup: 5 [
                    Text: "March",
                ],
                RightBracket: "]",
            ],
            Comma: ",",
            Space: " ",
            ContentBlock: 7 [
                LeftBracket: "[",
                Markup: 5 [
                    Escape: "\\$",
                    Text: "420",
                ],
                RightBracket: "]",
            ],
            Comma: ",",
            Space: "\n",
            RightParen: ")",
        ],
    ],
    Space: "\n",
]
```

### Against Invalid Table

```bash
./result/bin/typst-structure --source /path/to/invalid-table.typ
```

Output:

```
Markup: 120 [
    Text: "| Month",
    Space: "    ",
    Text: "| Savings",
    Space: "  ",
    Text: "|",
    Space: "\n",
    Text: "|",
    Space: " ",
    Shorthand: "---",
    Shorthand: "---",
    Shorthand: "--",
    Space: " ",
    Text: "|",
    Space: " ",
    Shorthand: "---",
    Shorthand: "---",
    Shorthand: "--",
    Space: " ",
    Text: "|",
    Space: "\n",
    Text: "| January",
    Space: "  ",
    Text: "|",
    Space: " ",
    Escape: "\\$",
    Text: "250",
    Space: "    ",
    Text: "|",
    Space: "\n",
    Text: "| February",
    Space: " ",
    Text: "|",
    Space: " ",
    Escape: "\\$",
    Text: "80",
    Space: "     ",
    Text: "|",
    Space: "\n",
    Text: "| March",
    Space: "    ",
    Text: "|",
    Space: " ",
    Escape: "\\$",
    Text: "420",
    Space: "    ",
    Text: "|",
    Space: "\n",
]
```
