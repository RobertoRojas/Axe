#/usr/bin/env bash
set -e;
cd "$(dirname "$0")";
rm -f file.txt.*;
rm -f file_out.txt;
echo '-----CUT COMMAND-----';
./axe CUT -f file.txt -s 10 -s 20,30 -s 40;
files=($(ls file.txt.*.axe));
echo '-----MELD COMMAND-----';
./axe MELD -o file_out.txt -f $(ls file.txt.*.axe);
amount=${#files[@]};
echo '-----COUNT TEST-----';
if [ $amount -ne 5 ] 
then
    echo "The count of files must be 3 instead of $amount";
    exit 1;
else
    echo 'The count is valid';
fi
files_names=('file.txt' 'file.txt.1.axe' 'file.txt.2.axe' 'file.txt.3.axe' 'file.txt.4.axe' 'file.txt.5.axe' 'file_out.txt')
files_sha=('f627813bcd67cb4a3612965a4d41aec35855b544ec2497484ca07ecfc26b9cd2' 'ab4fd153899c1f3aca272807e3ea40bf59a57d2754eda18f2f86d3fd7b7d2c82' 'f7bb51adbb34c8a3527ce7b9fe7a1cc0f304a51ffbc944fb810f3f7f63b3ec52' 'cc8f5e7ae5a9efbb97d795429268e4fed9458108e85a5d88b3b290fe8567e020' '61a9be393e46da3b68c5250189958e4637d34a968fa49f1abe57eed76f0c96ae' 'c6ae9906135267e41aa785eb107889b597f0efeae3316c4a8d353fb01d7326a6' 'f627813bcd67cb4a3612965a4d41aec35855b544ec2497484ca07ecfc26b9cd2')
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