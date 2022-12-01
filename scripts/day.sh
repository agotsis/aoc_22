#! /bin/bash

mv -n main.rs bin/$1.rs
cp -n tmp.rs main.rs
next=$(($1 + 1))
echo $next
sed -i "" "s/DATEHERE/$next/" main.rs
