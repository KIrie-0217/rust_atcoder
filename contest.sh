#!/bin/bash

echo "Create a new contest"
newcontest="./src/$1"

if [ ! -d $newcontest ]; then
    mkdir $newcontest
fi

cp ./src/main.rs "$newcontest/a.rs"
cp ./src/main.rs "${newcontest}/b.rs"
cp ./src/main.rs "${newcontest}/c.rs"
cp ./src/main.rs "${newcontest}/d.rs"
cp ./src/main.rs "${newcontest}/e.rs"
cp ./src/main.rs "${newcontest}/f.rs"
cp ./src/main.rs "${newcontest}/g.rs"
cp ./src/main.rs "${newcontest}/_ex.rs"

python3 makeToml.py