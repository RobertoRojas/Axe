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
files_sha=('f627813bcd67cb4a3612965a4d41aec35855b544ec2497484ca07ecfc26b9cd2' 'cbdde3391e3417a5d250a322ff5d188adbbbd968bafbfda89891e7d97119f4e7' '96d4d45a84946c6dfc746895121eb68568bb9a2e9a8a811696294a237142de0c' '86e2087c7701ad65eeb8db13cd53834f4d0cdce8e4049c9beeb2bc404e094181' 'f627813bcd67cb4a3612965a4d41aec35855b544ec2497484ca07ecfc26b9cd2')
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