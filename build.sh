#!/bin/bash

set -e

docker build . -t inkbox/until
test -d temp || mkdir temp
cd temp
docker save -o until.tar inkbox/until
tar xf until.tar
echo "layer:$(sed 's/","/\n/g' manifest.json | sed 's/"]}/\n/g' | tac | sed -n "2,2p")"
# cat manifest.json
tar xf $(sed 's/"/\n/g' manifest.json | sed 's/"]}/\n/g' | tac | sed -n "2,2p")
tar xf $(sed 's/"/\n/g' manifest.json | sed 's/"]}/\n/g' | tac | sed -n "4,4p")
tar xf $(sed 's/"/\n/g' manifest.json | sed 's/"]}/\n/g' | tac | sed -n "6,6p")
rm -rf ./app/.wh..wh..opq # 不知道是什么直接删除
rm -rf ../out
cp -r ./app ../out
cd ../
rm -rf ./temp
docker rmi inkbox/until