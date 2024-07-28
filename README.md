![Banner for Axe](.github/img/banner.webp)

# Axe

Axe is a CLI tool written in Rust that enables users to split files into multiple sections and subsequently reassemble them into a single file. It provides two primary commands: `CUT` and `MELD`.

## Features

### CUT

Splits a file into a specified number of pieces.

|Argument|Description|Default|Mandatory|
|:-:|:-:|:-:|:-:|
|-f/--file|Path of the file to be cut|N/A|Yes|
|-c/--count|Number of files to create|2|No|
|-s/--segments|Number of segments to cut|N/A|No|
**Note:** Only can select _count_ or _segments_ at the same time.

### MELD

Reassembles the pieces back into the original file.

|Argument|Description|Default|Mandatory|
|:-:|:-:|:-:|:-:|
|-f/--files|Paths of the files to be melded|N/A|Yes|
|-o/--output|Output path for the melded file|axe_output|No|

## Examples

```Bash
ls;
echo 'This is a test file' > test.txt;
cat test.txt;
./axe CUT -f test.txt -c 5;
ls -lh --block-size=1;
./axe MELD -o test_axe.txt -f $(ls test.txt.*);
cat test_axe.txt;
sha256sum test.txt;
sha256sum test_axe.txt;
```

[![asciicast](https://asciinema.org/a/O0KR9SpOWMAlFoHmse2kY7ANU.svg)](https://asciinema.org/a/O0KR9SpOWMAlFoHmse2kY7ANU)

```Bash
ls;
echo $(python -c "print('a' + '0'*1000 + 'z')") > test.txt;
cat test.txt;
./axe CUT -f test.txt -s 10 -s 20 -s 30,40;
ls -lh --block-size=1;
./axe MELD -o test_axe.txt -f $(ls test.txt.*);
cat test_axe.txt;
sha256sum test.txt;
sha256sum test_axe.txt;
```

[![asciicast](https://asciinema.org/a/1HLnuZkNAiJ05HAnikzxquqFC.svg)](https://asciinema.org/a/1HLnuZkNAiJ05HAnikzxquqFC) <- TODO

### License

This project is licensed under the **Apache 2.0 License** - see the [LICENSE](https://raw.githubusercontent.com/RobertoRojas/axe/main/LICENSE) file for details.

### Authors

- [Roberto Rojas](https://github.com/RobertoRojas)
