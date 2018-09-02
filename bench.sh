PORT=${PORT:-8005}
# ~200MB of random data:
hexdump -v -e '10/1 "%02x""\n"' -n 100000000 /dev/urandom  > data.txt
echo '' >> data.txt
wrk $@ -s bench-writes.lua "http://localhost:$PORT" > writes.out &
writes=$!
wrk $@ -s bench-reads.lua "http://localhost:$PORT" > reads.out &
reads=$!
wait $writes
wait $reads
echo '## Writes ##'
cat writes.out
echo '## Reads ##'
cat reads.out