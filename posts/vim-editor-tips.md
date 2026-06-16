+++
title = "Vim Editor Tips That Will 10x Your Editing Speed"
date = "2026-02-10"
tags = ["vim", "productivity", "editors"]
excerpt = "Move beyond basic navigation with these intermediate and advanced Vim techniques that shave seconds off every edit."
+++

I have been using Vim as my primary editor for over eight years. The difference between someone who just knows the basics and someone who truly wields Vim is not about memorizing more keybindings. It is about understanding the grammar of the editor.

## The Grammar of Vim

Vim operations follow a predictable pattern: `verb + modifier + motion`. Once this clicks, you stop thinking about individual keys.

| Verb (Operator) | Modifier | Motion | Result |
|---|---|---|---|
| `d` (delete) | `i` (inside) | `w` (word) | Delete inside word |
| `c` (change) | `a` (around) | `"` (quotes) | Change inside quotes |
| `y` (yank) | `t` (till) | `x` (char) | Yank till char `x` |

Combining these gives you `ci"` 鈥?change inside quotes 鈥?which replaces everything between double quotes in one keystroke.

## Relative Line Numbers

Absolute line numbers are useful for `42G`, but relative line numbers let you use `7j` or `5k` without mental arithmetic. The combination that works best is:

```vim
set number
set relativenumber
```

Vim will show your current line as an absolute number and everything else relative to it.

## Macros Are a Superpower

Recording a macro with `q` followed by a register letter transforms repetitive edits into a single command.

```vim
qa                    " Start recording into register a
^f,ldf,               " Jump to first comma, delete it
j                     " Move to next line
q                     " Stop recording
10@a                  " Replay macro 10 times
```

Use `@@` to repeat the last macro, and `100@a` to run it a hundred times.

## The Dot Command

The `.` command repeats the last change. If you delete a word with `daw`, then move the cursor and press `.`, it deletes that word too.

The trick is to make your changes composable. Instead of `dd` (delete whole line), use `d$` (delete to end of line) if you only want to repeat deleting to the end. Plan your operations so the dot command does what you need.

## Quick List and Completion Navigation

- `]q` / `[q` 鈥?next/previous quickfix item
- `]l` / `[l` 鈥?next/previous location list item
- `]m` / `[m` 鈥?next/previous method start

These are invaluable when working through compiler errors or linting warnings.

```vim
" Map them for easier access
nmap <silent> ]q :cnext<CR>
nmap <silent> [q :cprev<CR>
```

## Searching Within a File

Beyond basic `/pattern`, there are subtle variants you should know:

- `*` 鈥?search for the word under cursor forward
- `#` 鈥?search for the word under cursor backward
- `g*` 鈥?search for partial word under cursor
- `<C-r><C-w>` in command mode 鈥?paste word under cursor into search

Combine these with `n` and `N` for rapid navigation.

## The Expression Register

The `=` register evaluates Vim script expressions. In insert mode, press `<C-r>=` followed by an expression.

```
<C-r>= 3 + 5 鈫?8
<C-r>= line(".") 鈫?42
```

This is incredibly useful for generating sequences, computing values, or even calling custom functions on the fly.

## Windows and Tabs

Vim's window model tripped me up for years. Here is the distinction:

- **Buffers** 鈥?files loaded into memory
- **Windows** 鈥?viewports into buffers
- **Tabs** 鈥?collections of windows

Use `:ls` to list buffers, `<C-w>v` to split vertically, and `gt`/`gT` to cycle tabs. The key insight: you can have multiple windows showing the same buffer, which is perfect for scrolling different parts of a file simultaneously.

## Use Help Effectively

`:help` is the most underused feature. Navigate help with `j`/`k` and follow tags with `<C-]>`. Press `<C-o>` to jump back.

Start with `:help user-manual` if you have never read the built-in documentation.

## Conclusion

Vim mastery is not about learning every keybinding. It is about internalizing the composable grammar of operators, motions, and text objects. Start with five new techniques, practice them until they become automatic, then add five more. Over a year, the compound effect is dramatic.
