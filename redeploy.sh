#!/bin/sh
DIR="Location"

cd $DIR
rm -rf $DIR/redeploy/

git clone https://github.com/DenuxPlays/Denux-Dashboard.git redeploy
cd redeploy

bash trunk build --release

if [ $? -eq 0 ]; then
        rm -r $DIR/deploy
        mv $DIR/redeploy/dist $DIR/deploy
else
        echo FAILED TO COMPILE
fi