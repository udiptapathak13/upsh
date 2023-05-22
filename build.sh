mkdir -p tmp
mkdir -p bin
cd tmp
rustc -o upsh ../src/upsh.rs
mv upsh ../bin/
cd ../
rm -rf tmp
