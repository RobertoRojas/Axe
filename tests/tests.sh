#/usr/bin/env bash
set -e;
cd "$(dirname "$0")";
rm -f file.txt.*;
rm -f file_out.txt;
echo '-----CUT COMMAND-----';
./axe CUT -f file.txt -c 3;
files=($(ls file.txt.*.axe));
echo '-----MELD COMMAND-----';
./axe MELD -o file_out.txt -f $(ls file.txt.*.axe);
amount=${#files[@]};
echo '-----COUNT TEST-----';
if [ $amount -ne 3 ] 
then
    echo "The count of files must be 3 instead of $amount";
    exit 1;
else
    echo 'The count is valid';
fi
files_names=('file.txt' 'file.txt.1.axe' 'file.txt.2.axe' 'file.txt.3.axe' 'file_out.txt')
files_sha=('f29bc64a9d3732b4b9035125fdb3285f5b6455778edca72414671e0ca3b2e0de' '40f453ad94ef85960dc821ede50cceb92716acf80d11cc3a16509efc67415ba9' '068a7178ed16b5541cb5878c86cfd7d6327347bb4dfd6f15a61712fb6b7bc19d' '6633a6b45e448222b62dfbb520793386a1513a1f7ed9b8cde13d4f3c859b4930' 'f29bc64a9d3732b4b9035125fdb3285f5b6455778edca72414671e0ca3b2e0de')
echo '-----SHA TESTS-----';
for i in "${!files_names[@]}"; do
    if [ "$(sha256sum ${files_names[$i]} | awk '{print $1}')" != "${files_sha[$i]}" ]
    then
        echo "The value of the file ${files_names[$i]} is not valid, found $(sha256sum file.txt.1.axe | awk '{print $1}'), expected: '${files_sha[$i]}'";
        exit 1;
    else
        echo "Valid sha256 for ${files_names[$i]}";
    fi
done
echo '-----OUTPUT TESTS-----';
if [ "$(sha256sum file.txt | awk '{print $1}')" != "$(sha256sum file_out.txt | awk '{print $1}')" ]
then
    echo 'The file.txt sha256 is not equal to file_out.txt';
    exit 1;
else
    echo 'The same sha256 for file.txt and file_out.txt';
fi