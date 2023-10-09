# biff - A CLI tool for comparing directory structures

## Usage

```
biff ./dirone ./dirtwo
```

## Output

```
.hid                            .hid
both                            both
inner/                          inner/
  onefile2                        onefile2
innertwo/                       innertwo/
  innertwofile                    innertwofile
onefile.ts                      onefile.ts
```

(Files and directories that don't exist are dimmed in the respective column output)
