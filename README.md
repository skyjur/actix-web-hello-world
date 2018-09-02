A hello-world key/value store over http api with Rust, Actix & lmdb

```
## Writes ##
Running 20s test @ http://localhost:8005
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   991.44ms  125.61ms   1.40s    85.10%
    Req/Sec   228.56    106.06   484.00     70.77%
  8922 requests in 20.01s, 1.19MB read
  Socket errors: connect 58, read 0, write 0, timeout 0
Requests/sec:    445.96
Transfer/sec:     60.97KB
## Reads ##
Running 20s test @ http://localhost:8005
  2 threads and 1000 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.84ms   18.21ms 439.71ms   98.90%
    Req/Sec    27.49k     3.20k   40.60k    75.00%
  1074766 requests in 20.08s, 132.22MB read
  Socket errors: connect 58, read 0, write 0, timeout 0
  Non-2xx or 3xx responses: 1074766
Requests/sec:  53534.86
Transfer/sec:      6.59MB
```