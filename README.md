# colorify

makes color palettes from vibes

## setup

```
cargo build --release
```

or if you wanna install:
```
cargo install --path .
```

## usage

```
colorify "tropical beach sunset"
colorify "cozy winter cabin" --count 8
colorify --interactive
colorify "cyberpunk night" --bar
```

## what it does

takes mood words and spits out colors that match the vibe

uses ai embeddings to understand what you mean when you say stuff like "peaceful forest morning"

## examples

- "tropical beach sunset" → gold, coral, hot pink vibes
- "cyberpunk city" → neon crimson and cyan energy
- "cozy autumn cabin" → warm browns and oranges

## flags

- `-i` interactive mode
- `-b` show as color bar
- `-n 8` how many colors
- `--examples` see more ideas

thats it
