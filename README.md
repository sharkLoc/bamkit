# bamkit

🦀 bamkit: a simple program for bam file manipulation



### install

```bash
cargo install bamkit

# or

git clone https://github.com/sharkLoc/bamkit.git
cd bamkit
cargo b --release
# mv target/release/bamkit to anywhere you want 
```

### usage:
```
bamkit: a simple program for bam file manipulation

Usage: bamkit <COMMAND>

Commands:
  view    sam bam conversion
  region  get target region from bam file
  flags   bam file flag value show
  insert  insert size plot for bam file
  sample  sample sequences by number or fraktion
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information

```
###### view :
```
bamkit view -h
sam bam conversion

Usage: bamkit view [OPTIONS] [INPUT]

Arguments:
  [INPUT]  input bam[sam] file

Options:
  -H, --Header     show sam file header only
  -b, --bam        output is bam
  -o, --out <OUT>  output file name or write to stdout
  -h, --help       Print help information
```

###### region :
```
bamkit region -h
get target region from bam file

Usage: bamkit region [OPTIONS] --bam <BAM> <REG>

Arguments:
  <REG>  bam[sam] file target postion, eg, chr1:100-300

Options:
  -b, --bam <BAM>  input sorted and indexed bam file
  -s, --sam        output is sam
  -o, --out <OUT>  output file name or write to stdout, default bam format
  -h, --help       Print help information
```

###### insert :
```
bamkit insert -h
insert size plot for bam file

Usage: bamkit insert [OPTIONS] --name <NAME> [BAM]

Arguments:
  [BAM]  input bam[sam] file

Options:
  -m, --max <MAX>    max insert szie length [default: 1000]
  -n, --name <NAME>  the html format plot file name
  -h, --help         Print help information
```

<b>eg:</b> `bamkit insert  test.bam -n insertPlot`

<img width="645" alt="insert" src="https://user-images.githubusercontent.com/50580507/203885026-31a8090a-ec47-4831-bc94-68ea604b792d.png">

###### flags :
```
bamkit flags -h
bam file flag value show

Usage: bamkit flags <FLAG>

Arguments:
  <FLAG>  specify bam[sam] flag value

Options:
  -h, --help  Print help information
```

<b>eg:</b> `bamkit flags 163`

<img width="403" alt="flags" src="https://user-images.githubusercontent.com/50580507/203884903-e35d7f3c-548f-4ce7-ba0a-1a908b10e80d.png">

###### sample :

```
bamkit sample -h
sample sequences by number or fraktion

Usage: bamkit sample [OPTIONS] [BAM]

Arguments:
  [BAM]  input bam[sam] file

Options:
  -s, --seed <SEED>  set rand seed [default: 69]
  -n, --num <NUM>    sample by number, use with -r on large bam file
  -f, --frak <FRAK>  sample by fraktion, use with -r on large bam file
  -r, --rdc          reduce much memory but cost more time
  -o, --out <OUT>    output bam file name or write to stdout


```



** any bugs please report issues **💖