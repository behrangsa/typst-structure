# Basic Typst AST Printer

## Table of Contents

- [Build](#build)
- [Run](#run)
  - [Against Valid Table](#against-valid-table)
  - [Against Invalid Table](#against-invalid-table)
  - [JSON output](#json-output)

## Build

```bash
nix develop
nix build      # or cargo build
```

## Run

### Against Valid Table

Command:

```bash
./result/bin/typst-structure --source /path/to/valid-table.typ
```

<details>
<summary>Output:</summary>

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

</details>

### Against Invalid Table

```bash
./result/bin/typst-structure --source /path/to/invalid-table.typ
```

<details>
<summary>Output:</summary>

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

</details>

### JSON output

```bash
./result/bin/typst-structure --source /path/to/invalid-table.typ --json
```

<details>
<summary>Output:</summary>

```json
{
  "kind": "Markup",
  "len": 147,
  "children": [
    {
      "kind": "Hash",
      "len": 1,
      "text": "#"
    },
    {
      "kind": "FuncCall",
      "len": 145,
      "children": [
        {
          "kind": "Ident",
          "len": 5,
          "text": "table"
        },
        {
          "kind": "Args",
          "len": 140,
          "children": [
            {
              "kind": "LeftParen",
              "len": 1,
              "text": "("
            },
            {
              "kind": "Space",
              "len": 3,
              "text": "\n  "
            },
            {
              "kind": "Named",
              "len": 10,
              "children": [
                {
                  "kind": "Ident",
                  "len": 7,
                  "text": "columns"
                },
                {
                  "kind": "Colon",
                  "len": 1,
                  "text": ":"
                },
                {
                  "kind": "Space",
                  "len": 1,
                  "text": " "
                },
                {
                  "kind": "Int",
                  "len": 1,
                  "text": "2"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 3,
              "text": "\n  "
            },
            {
              "kind": "FuncCall",
              "len": 55,
              "children": [
                {
                  "kind": "FieldAccess",
                  "len": 12,
                  "children": [
                    {
                      "kind": "Ident",
                      "len": 5,
                      "text": "table"
                    },
                    {
                      "kind": "Dot",
                      "len": 1,
                      "text": "."
                    },
                    {
                      "kind": "Ident",
                      "len": 6,
                      "text": "header"
                    }
                  ]
                },
                {
                  "kind": "Args",
                  "len": 43,
                  "children": [
                    {
                      "kind": "LeftParen",
                      "len": 1,
                      "text": "("
                    },
                    {
                      "kind": "Space",
                      "len": 5,
                      "text": "\n    "
                    },
                    {
                      "kind": "Named",
                      "len": 8,
                      "children": [
                        {
                          "kind": "Ident",
                          "len": 5,
                          "text": "level"
                        },
                        {
                          "kind": "Colon",
                          "len": 1,
                          "text": ":"
                        },
                        {
                          "kind": "Space",
                          "len": 1,
                          "text": " "
                        },
                        {
                          "kind": "Int",
                          "len": 1,
                          "text": "1"
                        }
                      ]
                    },
                    {
                      "kind": "Comma",
                      "len": 1,
                      "text": ","
                    },
                    {
                      "kind": "Space",
                      "len": 5,
                      "text": "\n    "
                    },
                    {
                      "kind": "ContentBlock",
                      "len": 7,
                      "children": [
                        {
                          "kind": "LeftBracket",
                          "len": 1,
                          "text": "["
                        },
                        {
                          "kind": "Markup",
                          "len": 5,
                          "children": [
                            {
                              "kind": "Text",
                              "len": 5,
                              "text": "Month"
                            }
                          ]
                        },
                        {
                          "kind": "RightBracket",
                          "len": 1,
                          "text": "]"
                        }
                      ]
                    },
                    {
                      "kind": "Comma",
                      "len": 1,
                      "text": ","
                    },
                    {
                      "kind": "Space",
                      "len": 1,
                      "text": " "
                    },
                    {
                      "kind": "ContentBlock",
                      "len": 9,
                      "children": [
                        {
                          "kind": "LeftBracket",
                          "len": 1,
                          "text": "["
                        },
                        {
                          "kind": "Markup",
                          "len": 7,
                          "children": [
                            {
                              "kind": "Text",
                              "len": 7,
                              "text": "Savings"
                            }
                          ]
                        },
                        {
                          "kind": "RightBracket",
                          "len": 1,
                          "text": "]"
                        }
                      ]
                    },
                    {
                      "kind": "Comma",
                      "len": 1,
                      "text": ","
                    },
                    {
                      "kind": "Space",
                      "len": 3,
                      "text": "\n  "
                    },
                    {
                      "kind": "RightParen",
                      "len": 1,
                      "text": ")"
                    }
                  ]
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 3,
              "text": "\n  "
            },
            {
              "kind": "ContentBlock",
              "len": 9,
              "children": [
                {
                  "kind": "LeftBracket",
                  "len": 1,
                  "text": "["
                },
                {
                  "kind": "Markup",
                  "len": 7,
                  "children": [
                    {
                      "kind": "Text",
                      "len": 7,
                      "text": "January"
                    }
                  ]
                },
                {
                  "kind": "RightBracket",
                  "len": 1,
                  "text": "]"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 1,
              "text": " "
            },
            {
              "kind": "ContentBlock",
              "len": 7,
              "children": [
                {
                  "kind": "LeftBracket",
                  "len": 1,
                  "text": "["
                },
                {
                  "kind": "Markup",
                  "len": 5,
                  "children": [
                    {
                      "kind": "Escape",
                      "len": 2,
                      "text": "\\$"
                    },
                    {
                      "kind": "Text",
                      "len": 3,
                      "text": "250"
                    }
                  ]
                },
                {
                  "kind": "RightBracket",
                  "len": 1,
                  "text": "]"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 3,
              "text": "\n  "
            },
            {
              "kind": "ContentBlock",
              "len": 10,
              "children": [
                {
                  "kind": "LeftBracket",
                  "len": 1,
                  "text": "["
                },
                {
                  "kind": "Markup",
                  "len": 8,
                  "children": [
                    {
                      "kind": "Text",
                      "len": 8,
                      "text": "February"
                    }
                  ]
                },
                {
                  "kind": "RightBracket",
                  "len": 1,
                  "text": "]"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 1,
              "text": " "
            },
            {
              "kind": "ContentBlock",
              "len": 6,
              "children": [
                {
                  "kind": "LeftBracket",
                  "len": 1,
                  "text": "["
                },
                {
                  "kind": "Markup",
                  "len": 4,
                  "children": [
                    {
                      "kind": "Escape",
                      "len": 2,
                      "text": "\\$"
                    },
                    {
                      "kind": "Text",
                      "len": 2,
                      "text": "80"
                    }
                  ]
                },
                {
                  "kind": "RightBracket",
                  "len": 1,
                  "text": "]"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 3,
              "text": "\n  "
            },
            {
              "kind": "ContentBlock",
              "len": 7,
              "children": [
                {
                  "kind": "LeftBracket",
                  "len": 1,
                  "text": "["
                },
                {
                  "kind": "Markup",
                  "len": 5,
                  "children": [
                    {
                      "kind": "Text",
                      "len": 5,
                      "text": "March"
                    }
                  ]
                },
                {
                  "kind": "RightBracket",
                  "len": 1,
                  "text": "]"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 1,
              "text": " "
            },
            {
              "kind": "ContentBlock",
              "len": 7,
              "children": [
                {
                  "kind": "LeftBracket",
                  "len": 1,
                  "text": "["
                },
                {
                  "kind": "Markup",
                  "len": 5,
                  "children": [
                    {
                      "kind": "Escape",
                      "len": 2,
                      "text": "\\$"
                    },
                    {
                      "kind": "Text",
                      "len": 3,
                      "text": "420"
                    }
                  ]
                },
                {
                  "kind": "RightBracket",
                  "len": 1,
                  "text": "]"
                }
              ]
            },
            {
              "kind": "Comma",
              "len": 1,
              "text": ","
            },
            {
              "kind": "Space",
              "len": 1,
              "text": "\n"
            },
            {
              "kind": "RightParen",
              "len": 1,
              "text": ")"
            }
          ]
        }
      ]
    },
    {
      "kind": "Space",
      "len": 1,
      "text": "\n"
    }
  ]
}
```

</details>
