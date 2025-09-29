lru_rs_put/put/2        time:   [65.673 ns 69.179 ns 72.112 ns]
lru_rs_put/put/4        time:   [112.39 ns 116.60 ns 120.29 ns]
lru_rs_put/put/8        time:   [184.16 ns 188.94 ns 193.19 ns]
lru_rs_put/put/16       time:   [339.30 ns 347.21 ns 354.29 ns]
lru_rs_put/put/32       time:   [572.05 ns 583.22 ns 593.87 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
lru_rs_put/put/64       time:   [1.0217 µs 1.0233 µs 1.0250 µs]
Found 23 outliers among 100 measurements (23.00%)
  15 (15.00%) low severe
  4 (4.00%) low mild
  4 (4.00%) high mild
lru_rs_put/put/128      time:   [1.9935 µs 1.9996 µs 2.0068 µs]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

lru_rs_get/get/2        time:   [23.636 ns 24.054 ns 24.381 ns]
lru_rs_get/get/4        time:   [36.065 ns 36.705 ns 37.304 ns]
lru_rs_get/get/8        time:   [54.520 ns 55.951 ns 57.263 ns]
Found 14 outliers among 100 measurements (14.00%)
  14 (14.00%) low mild
lru_rs_get/get/16       time:   [84.263 ns 86.449 ns 88.562 ns]
Found 27 outliers among 100 measurements (27.00%)
  17 (17.00%) low severe
  3 (3.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe
lru_rs_get/get/32       time:   [131.77 ns 133.19 ns 134.57 ns]
Found 22 outliers among 100 measurements (22.00%)
  17 (17.00%) low severe
  4 (4.00%) low mild
  1 (1.00%) high mild
lru_rs_get/get/64       time:   [205.42 ns 206.75 ns 208.15 ns]
Found 22 outliers among 100 measurements (22.00%)
  8 (8.00%) low severe
  11 (11.00%) low mild
  3 (3.00%) high mild
lru_rs_get/get/128      time:   [363.45 ns 365.17 ns 366.82 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

lru_rs_peek/peek/2      time:   [17.898 ns 18.373 ns 18.745 ns]
lru_rs_peek/peek/4      time:   [29.688 ns 30.153 ns 30.537 ns]
lru_rs_peek/peek/8      time:   [45.445 ns 46.181 ns 46.810 ns]
Found 17 outliers among 100 measurements (17.00%)
  17 (17.00%) low mild
lru_rs_peek/peek/16     time:   [77.591 ns 78.779 ns 79.911 ns]
Found 22 outliers among 100 measurements (22.00%)
  17 (17.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
lru_rs_peek/peek/32     time:   [122.92 ns 126.54 ns 129.73 ns]
Found 17 outliers among 100 measurements (17.00%)
  17 (17.00%) low mild
lru_rs_peek/peek/64     time:   [158.07 ns 160.54 ns 163.02 ns]
Found 16 outliers among 100 measurements (16.00%)
  15 (15.00%) low mild
  1 (1.00%) high mild
lru_rs_peek/peek/128    time:   [272.35 ns 279.54 ns 286.49 ns]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

lru_rs_pop_lru/pop_lru/2
                        time:   [101.19 ns 107.03 ns 111.84 ns]
lru_rs_pop_lru/pop_lru/4
                        time:   [146.12 ns 154.09 ns 161.21 ns]
lru_rs_pop_lru/pop_lru/8
                        time:   [237.25 ns 249.90 ns 261.06 ns]
lru_rs_pop_lru/pop_lru/16
                        time:   [331.84 ns 337.73 ns 343.17 ns]
Found 18 outliers among 100 measurements (18.00%)
  18 (18.00%) low mild
lru_rs_pop_lru/pop_lru/32
                        time:   [564.45 ns 569.99 ns 574.70 ns]
Found 21 outliers among 100 measurements (21.00%)
  3 (3.00%) low severe
  18 (18.00%) low mild
lru_rs_pop_lru/pop_lru/64
                        time:   [977.44 ns 983.24 ns 988.75 ns]
Found 22 outliers among 100 measurements (22.00%)
  13 (13.00%) low severe
  9 (9.00%) low mild
lru_rs_pop_lru/pop_lru/128
                        time:   [1.8044 µs 1.8165 µs 1.8269 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild


const_lru_insert/insert/2
                        time:   [19.098 ns 19.881 ns 20.601 ns]
                        change: [-7.3706% -2.3913% +2.9543%] (p = 0.39 > 0.05)
                        No change in performance detected.
Found 24 outliers among 100 measurements (24.00%)
  12 (12.00%) low mild
  12 (12.00%) high mild
const_lru_insert/insert/4
                        time:   [38.005 ns 38.605 ns 39.149 ns]
                        change: [-3.5777% -1.0464% +1.4709%] (p = 0.42 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) low mild
  1 (1.00%) high mild
const_lru_insert/insert/8
                        time:   [83.181 ns 83.965 ns 84.673 ns]
                        change: [-0.8246% +0.6799% +2.1930%] (p = 0.39 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low mild
const_lru_insert/insert/16
                        time:   [182.47 ns 183.43 ns 184.38 ns]
                        change: [+1.6196% +2.9623% +4.2346%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild
  3 (3.00%) high mild
const_lru_insert/insert/32
                        time:   [417.77 ns 420.87 ns 423.98 ns]
                        change: [+3.6454% +4.4959% +5.2567%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
const_lru_insert/insert/64
                        time:   [952.09 ns 960.36 ns 971.42 ns]
                        change: [+2.6212% +3.2814% +4.0120%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
const_lru_insert/insert/128
                        time:   [2.1154 µs 2.1198 µs 2.1251 µs]
                        change: [-0.5057% -0.0559% +0.3914%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

const_lru_get/get/2     time:   [15.067 ns 15.433 ns 15.750 ns]
                        change: [-4.8204% +1.8904% +8.9196%] (p = 0.59 > 0.05)
                        No change in performance detected.
const_lru_get/get/4     time:   [35.182 ns 36.057 ns 36.820 ns]
                        change: [-5.2757% +0.6854% +6.8179%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  11 (11.00%) low mild
const_lru_get/get/8     time:   [60.289 ns 61.192 ns 61.999 ns]
                        change: [-1.6822% +2.9047% +7.7502%] (p = 0.23 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  14 (14.00%) low mild
const_lru_get/get/16    time:   [103.50 ns 104.81 ns 106.20 ns]
                        change: [-3.2357% -1.2982% +0.6996%] (p = 0.21 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  10 (10.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
const_lru_get/get/32    time:   [222.57 ns 226.04 ns 230.41 ns]
                        change: [-0.9450% +1.0685% +3.1309%] (p = 0.33 > 0.05)
                        No change in performance detected.
Found 18 outliers among 100 measurements (18.00%)
  1 (1.00%) low severe
  11 (11.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
const_lru_get/get/64    time:   [470.73 ns 473.41 ns 476.07 ns]
                        change: [-7.7651% -5.8860% -4.0712%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 19 outliers among 100 measurements (19.00%)
  13 (13.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
const_lru_get/get/128   time:   [1.0260 µs 1.0378 µs 1.0499 µs]
                        change: [-4.0388% -3.1092% -2.2035%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe

const_lru_get_untouched/get_untouched/2
                        time:   [9.1734 ns 9.7305 ns 10.289 ns]
                        change: [-1.9166% +7.6096% +18.001%] (p = 0.13 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
const_lru_get_untouched/get_untouched/4
                        time:   [15.096 ns 16.688 ns 18.514 ns]
                        change: [-9.3383% -1.0025% +8.6827%] (p = 0.84 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe
const_lru_get_untouched/get_untouched/8
                        time:   [26.392 ns 26.960 ns 27.493 ns]
                        change: [-5.3278% -1.1746% +2.9992%] (p = 0.58 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  1 (1.00%) low severe
  13 (13.00%) low mild
  3 (3.00%) high mild
const_lru_get_untouched/get_untouched/16
                        time:   [53.754 ns 54.507 ns 55.273 ns]
                        change: [-1.8673% -0.1987% +1.4470%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 29 outliers among 100 measurements (29.00%)
  13 (13.00%) low severe
  2 (2.00%) low mild
  9 (9.00%) high mild
  5 (5.00%) high severe
const_lru_get_untouched/get_untouched/32
                        time:   [130.46 ns 131.41 ns 132.40 ns]
                        change: [-3.9226% -2.5520% -1.3237%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 21 outliers among 100 measurements (21.00%)
  10 (10.00%) low severe
  4 (4.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe
const_lru_get_untouched/get_untouched/64
                        time:   [334.29 ns 339.87 ns 344.63 ns]
                        change: [+1.9920% +3.3838% +4.7419%] (p = 0.00 < 0.05)
                        Performance has regressed.
const_lru_get_untouched/get_untouched/128
                        time:   [692.95 ns 696.99 ns 700.82 ns]
                        change: [+2.1652% +2.9121% +3.6151%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

const_lru_remove/remove/2
                        time:   [19.006 ns 19.655 ns 20.276 ns]
                        change: [-0.0297% +6.1049% +12.630%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
const_lru_remove/remove/4
                        time:   [61.369 ns 62.142 ns 62.928 ns]
                        change: [+4.4295% +8.4295% +12.610%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 22 outliers among 100 measurements (22.00%)
  16 (16.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
const_lru_remove/remove/8
                        time:   [123.43 ns 124.32 ns 125.19 ns]
                        change: [-3.9586% -0.8548% +2.3007%] (p = 0.60 > 0.05)
                        No change in performance detected.
Found 19 outliers among 100 measurements (19.00%)
  15 (15.00%) low severe
  1 (1.00%) low mild
  3 (3.00%) high mild
const_lru_remove/remove/16
                        time:   [311.45 ns 313.86 ns 316.41 ns]
                        change: [-3.2780% -2.0227% -0.8527%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
const_lru_remove/remove/32
                        time:   [818.48 ns 823.20 ns 828.16 ns]
                        change: [-7.4029% -6.4200% -5.4536%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
const_lru_remove/remove/64
                        time:   [2.3994 µs 2.4036 µs 2.4081 µs]
                        change: [-4.3207% -3.8722% -3.4323%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
const_lru_remove/remove/128
                        time:   [8.6789 µs 8.7024 µs 8.7324 µs]
                        change: [-3.2970% -2.9776% -2.5820%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

uluru_insert/insert/2   time:   [4.4475 ns 4.5258 ns 4.5929 ns]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) low mild
uluru_insert/insert/4   time:   [7.0600 ns 7.1608 ns 7.2628 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
uluru_insert/insert/8   time:   [20.339 ns 20.414 ns 20.486 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
uluru_insert/insert/16  time:   [28.401 ns 28.886 ns 29.357 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
uluru_insert/insert/32  time:   [50.145 ns 50.440 ns 50.729 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
uluru_insert/insert/64  time:   [97.773 ns 98.132 ns 98.484 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) low mild
  1 (1.00%) high mild
uluru_insert/insert/128 time:   [192.36 ns 193.13 ns 193.88 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

uluru_find/find/2       time:   [7.8776 ns 8.4620 ns 9.0640 ns]
Found 18 outliers among 100 measurements (18.00%)
  13 (13.00%) high mild
  5 (5.00%) high severe
uluru_find/find/4       time:   [19.798 ns 20.041 ns 20.262 ns]
uluru_find/find/8       time:   [67.709 ns 67.746 ns 67.782 ns]
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) low severe
  3 (3.00%) low mild
  1 (1.00%) high mild
  5 (5.00%) high severe
uluru_find/find/16      time:   [252.55 ns 253.08 ns 253.57 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
uluru_find/find/32      time:   [968.77 ns 969.11 ns 969.50 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  5 (5.00%) high severe
uluru_find/find/64      time:   [4.3884 µs 4.3940 µs 4.4011 µs]
uluru_find/find/128     time:   [16.767 µs 16.790 µs 16.813 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe

uluru_front/front/2     time:   [2.9412 ns 3.2261 ns 3.5038 ns]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
uluru_front/front/4     time:   [3.6503 ns 3.9958 ns 4.3291 ns]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
uluru_front/front/8     time:   [3.7935 ns 4.0989 ns 4.3928 ns]
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low mild
  9 (9.00%) high mild
uluru_front/front/16    time:   [4.5162 ns 4.9641 ns 5.3940 ns]
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
uluru_front/front/32    time:   [6.7948 ns 7.8344 ns 8.8427 ns]
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high severe
uluru_front/front/64    time:   [9.6592 ns 10.737 ns 11.767 ns]
Found 12 outliers among 100 measurements (12.00%)
  12 (12.00%) high severe
uluru_front/front/128   time:   [14.141 ns 14.693 ns 15.254 ns]
Found 9 outliers among 100 measurements (9.00%)
  9 (9.00%) high severe

uluru_get/get/2         time:   [4.2053 ns 4.6868 ns 5.1484 ns]
Found 9 outliers among 100 measurements (9.00%)
  4 (4.00%) high mild
  5 (5.00%) high severe
uluru_get/get/4         time:   [6.8698 ns 7.3544 ns 7.8478 ns]
Found 13 outliers among 100 measurements (13.00%)
  7 (7.00%) low mild
  1 (1.00%) high mild
  5 (5.00%) high severe
uluru_get/get/8         time:   [15.264 ns 15.521 ns 15.807 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high severe
uluru_get/get/16        time:   [49.968 ns 50.076 ns 50.195 ns]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low mild
  5 (5.00%) high mild
  5 (5.00%) high severe
uluru_get/get/32        time:   [253.92 ns 254.26 ns 254.65 ns]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
uluru_get/get/64        time:   [1.3366 µs 1.3419 µs 1.3474 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
uluru_get/get/128       time:   [6.7974 µs 6.8216 µs 6.8475 µs]

schnellru_insert/insert/2
                        time:   [23.269 ns 23.425 ns 23.579 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
schnellru_insert/insert/4
                        time:   [103.29 ns 103.55 ns 103.84 ns]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
schnellru_insert/insert/8
                        time:   [245.17 ns 245.89 ns 246.65 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
schnellru_insert/insert/16
                        time:   [484.15 ns 484.84 ns 485.61 ns]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) low mild
  7 (7.00%) high mild
  1 (1.00%) high severe
schnellru_insert/insert/32
                        time:   [889.97 ns 891.21 ns 892.64 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
schnellru_insert/insert/64
                        time:   [1.6827 µs 1.6852 µs 1.6880 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
schnellru_insert/insert/128
                        time:   [3.2331 µs 3.2382 µs 3.2439 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

schnellru_get/get/2     time:   [7.4847 ns 7.5622 ns 7.6442 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
schnellru_get/get/4     time:   [13.460 ns 13.616 ns 13.791 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
schnellru_get/get/8     time:   [26.308 ns 26.460 ns 26.626 ns]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
schnellru_get/get/16    time:   [52.827 ns 53.249 ns 53.697 ns]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
schnellru_get/get/32    time:   [104.57 ns 104.80 ns 105.03 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
schnellru_get/get/64    time:   [212.41 ns 213.55 ns 214.89 ns]
schnellru_get/get/128   time:   [425.18 ns 426.93 ns 428.53 ns]

schnellru_peek/peek/2   time:   [5.8227 ns 5.9621 ns 6.1357 ns]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
schnellru_peek/peek/4   time:   [11.155 ns 11.243 ns 11.336 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
schnellru_peek/peek/8   time:   [21.156 ns 21.340 ns 21.558 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
schnellru_peek/peek/16  time:   [41.754 ns 41.853 ns 41.945 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
schnellru_peek/peek/32  time:   [86.819 ns 87.068 ns 87.322 ns]
schnellru_peek/peek/64  time:   [177.97 ns 178.67 ns 179.20 ns]
schnellru_peek/peek/128 time:   [347.28 ns 348.50 ns 349.46 ns]

schnellru_pop_oldest/pop_oldest/2
                        time:   [8.7163 ns 8.7718 ns 8.8323 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
schnellru_pop_oldest/pop_oldest/4
                        time:   [21.445 ns 21.588 ns 21.756 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
schnellru_pop_oldest/pop_oldest/8
                        time:   [46.806 ns 46.928 ns 47.063 ns]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
schnellru_pop_oldest/pop_oldest/16
                        time:   [90.708 ns 90.991 ns 91.278 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
schnellru_pop_oldest/pop_oldest/32
                        time:   [131.76 ns 131.93 ns 132.12 ns]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
schnellru_pop_oldest/pop_oldest/64
                        time:   [254.61 ns 256.31 ns 257.60 ns]
schnellru_pop_oldest/pop_oldest/128
                        time:   [524.15 ns 529.58 ns 533.81 ns]

push/push/2             time:   [9.0140 ns 9.4562 ns 9.8426 ns]
                        change: [-10.922% -5.4408% +0.6741%] (p = 0.08 > 0.05)
                        No change in performance detected.
push/push/4             time:   [15.265 ns 15.730 ns 16.160 ns]
                        change: [-6.8256% -4.6710% -2.5024%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) high mild
  13 (13.00%) high severe
push/push/8             time:   [30.818 ns 31.290 ns 31.769 ns]
                        change: [-4.2241% -2.7938% -1.4660%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  10 (10.00%) high severe
push/push/16            time:   [63.669 ns 64.040 ns 64.566 ns]
                        change: [-5.4809% -4.7012% -3.9820%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
push/push/32            time:   [169.39 ns 169.73 ns 170.06 ns]
                        change: [-0.7166% -0.3583% -0.0488%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
push/push/64            time:   [728.97 ns 735.60 ns 740.71 ns]
                        change: [+8.2953% +10.550% +12.780%] (p = 0.00 < 0.05)
                        Performance has regressed.
push/push/128           time:   [2.6909 µs 2.6981 µs 2.7047 µs]
                        change: [-1.7024% -1.3660% -1.0016%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) low mild

get/get/2               time:   [9.5426 ns 10.088 ns 10.638 ns]
                        change: [+2.6104% +7.4659% +13.235%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
get/get/4               time:   [14.774 ns 15.348 ns 15.999 ns]
                        change: [-1.0943% +1.5429% +4.3387%] (p = 0.28 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  9 (9.00%) high severe
get/get/8               time:   [29.288 ns 29.620 ns 29.979 ns]
                        change: [-2.2387% -1.3248% -0.4258%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  3 (3.00%) low mild
  4 (4.00%) high mild
  10 (10.00%) high severe
get/get/16              time:   [71.568 ns 72.074 ns 72.647 ns]
                        change: [-4.5304% -3.3116% -2.1583%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
get/get/32              time:   [218.01 ns 220.08 ns 221.84 ns]
                        change: [-1.7587% -0.7995% +0.2274%] (p = 0.13 > 0.05)
                        No change in performance detected.
get/get/64              time:   [817.68 ns 820.09 ns 822.54 ns]
                        change: [-2.0017% -1.5857% -1.1682%] (p = 0.00 < 0.05)
                        Performance has improved.
get/get/128             time:   [3.3670 µs 3.3735 µs 3.3800 µs]
                        change: [-3.1377% -2.8394% -2.5379%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

peek/peek/2             time:   [7.1099 ns 7.7793 ns 8.4287 ns]
                        change: [+4.3811% +17.112% +29.813%] (p = 0.01 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
peek/peek/4             time:   [9.2712 ns 10.269 ns 11.293 ns]
                        change: [-6.6425% +0.0606% +8.3700%] (p = 0.99 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe
peek/peek/8             time:   [17.681 ns 18.188 ns 18.725 ns]
                        change: [-4.3774% -1.9128% +0.7973%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
peek/peek/16            time:   [49.445 ns 49.828 ns 50.217 ns]
                        change: [-4.5314% -2.8055% -1.2443%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
peek/peek/32            time:   [158.30 ns 159.30 ns 160.33 ns]
                        change: [-12.874% -11.581% -9.8380%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
peek/peek/64            time:   [842.21 ns 843.04 ns 843.98 ns]
                        change: [-1.4121% -0.6596% +0.1874%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
peek/peek/128           time:   [3.1609 µs 3.1653 µs 3.1702 µs]
                        change: [-2.1131% -0.8722% +0.3478%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

pop/pop/2               time:   [7.2013 ns 7.7301 ns 8.2363 ns]
                        change: [-62.499% -59.893% -56.287%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  8 (8.00%) high mild
pop/pop/4               time:   [16.800 ns 17.029 ns 17.278 ns]
                        change: [-33.412% -30.413% -26.304%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
pop/pop/8               time:   [34.861 ns 35.039 ns 35.201 ns]
                        change: [-38.746% -35.605% -30.753%] (p = 0.00 < 0.05)
                        Performance has improved.
pop/pop/16              time:   [81.697 ns 81.928 ns 82.139 ns]
                        change: [-29.977% -27.086% -22.710%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) low mild
  1 (1.00%) high severe
pop/pop/32              time:   [160.45 ns 160.89 ns 161.36 ns]
                        change: [-16.713% -14.639% -11.658%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
pop/pop/64              time:   [306.90 ns 307.36 ns 307.88 ns]
                        change: [-12.009% -9.9515% -7.4241%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) low mild
  2 (2.00%) high mild
pop/pop/128             time:   [605.09 ns 606.81 ns 608.67 ns]
                        change: [-4.0190% -2.9598% -1.9134%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild