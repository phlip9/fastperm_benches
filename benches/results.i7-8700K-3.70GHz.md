Processor: Intel i7-8700K @ 3.70 GHz

```
$ uname -a
Linux 4.19.104-microsoft-standard #1 SMP Wed Feb 19 06:37:35 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
```

### index_permutation_64_all

```
index_permutation_64_all/shuffle
                        time:   [347.35 ns 349.76 ns 352.88 ns]
                        thrpt:  [181.36 Melem/s 182.98 Melem/s 184.25 Melem/s]
index_permutation_64_all/shuffle_array
                        time:   [348.64 ns 349.35 ns 350.15 ns]
                        thrpt:  [182.78 Melem/s 183.20 Melem/s 183.57 Melem/s]
index_permutation_64_all/shuffle_array_incr
                        time:   [119.19 ns 119.82 ns 120.56 ns]
                        thrpt:  [530.85 Melem/s 534.12 Melem/s 536.95 Melem/s]
index_permutation_64_all/bit_scatter
                        time:   [151.67 ns 152.02 ns 152.50 ns]
                        thrpt:  [419.66 Melem/s 421.00 Melem/s 421.97 Melem/s]
index_permutation_64_all/bit_scatter_rng_ref
                        time:   [153.86 ns 154.41 ns 155.04 ns]
                        thrpt:  [412.79 Melem/s 414.49 Melem/s 415.96 Melem/s]
```

### index_permutation_64_iter_period

```
index_permutation_64_iter_period/shuffle
                        time:   [348.05 ns 348.43 ns 348.84 ns]
                        thrpt:  [183.47 Melem/s 183.68 Melem/s 183.88 Melem/s]
index_permutation_64_iter_period/shuffle_array
                        time:   [346.97 ns 347.39 ns 347.83 ns]
                        thrpt:  [184.00 Melem/s 184.23 Melem/s 184.45 Melem/s]
index_permutation_64_iter_period/shuffle_array_incr
                        time:   [119.13 ns 119.40 ns 119.67 ns]
                        thrpt:  [534.79 Melem/s 536.02 Melem/s 537.23 Melem/s]
index_permutation_64_iter_period/bit_scatter
                        time:   [118.24 ns 118.33 ns 118.46 ns]
                        thrpt:  [540.28 Melem/s 540.85 Melem/s 541.29 Melem/s]
index_permutation_64_iter_period/bit_scatter_rng_ref
                        time:   [155.18 ns 155.36 ns 155.59 ns]
                        thrpt:  [411.34 Melem/s 411.95 Melem/s 412.43 Melem/s]
```

### index_permutation_64_one_idx

```
index_permutation_64_one_idx/shuffle
                        time:   [5.6758 ns 5.6853 ns 5.6970 ns]
                        thrpt:  [175.53 Melem/s 175.89 Melem/s 176.19 Melem/s]
index_permutation_64_one_idx/shuffle_array
                        time:   [5.5870 ns 5.5936 ns 5.6010 ns]
                        thrpt:  [178.54 Melem/s 178.78 Melem/s 178.99 Melem/s]
index_permutation_64_one_idx/shuffle_array_incr
                        time:   [1.8835 ns 1.8863 ns 1.8905 ns]
                        thrpt:  [528.97 Melem/s 530.14 Melem/s 530.93 Melem/s]
index_permutation_64_one_idx/bit_scatter
                        time:   [2.4146 ns 2.4163 ns 2.4185 ns]
                        thrpt:  [413.47 Melem/s 413.86 Melem/s 414.15 Melem/s]
index_permutation_64_one_idx/bit_scatter_rng_ref
                        time:   [2.4206 ns 2.4229 ns 2.4256 ns]
                        thrpt:  [412.26 Melem/s 412.72 Melem/s 413.12 Melem/s]
```

### index_permutation_64_reset_and_one_idx

```
index_permutation_64_reset_and_one_idx/shuffle
                        time:   [321.24 ns 321.54 ns 321.85 ns]
                        thrpt:  [3.1070 Melem/s 3.1100 Melem/s 3.1130 Melem/s]
index_permutation_64_reset_and_one_idx/shuffle_array
                        time:   [322.06 ns 322.34 ns 322.61 ns]
                        thrpt:  [3.0998 Melem/s 3.1023 Melem/s 3.1050 Melem/s]
index_permutation_64_reset_and_one_idx/shuffle_array_incr
                        time:   [1.5277 ns 1.5298 ns 1.5325 ns]
                        thrpt:  [652.51 Melem/s 653.67 Melem/s 654.58 Melem/s]
index_permutation_64_reset_and_one_idx/bit_scatter
                        time:   [2.2594 ns 2.2628 ns 2.2669 ns]
                        thrpt:  [441.14 Melem/s 441.93 Melem/s 442.59 Melem/s]
index_permutation_64_reset_and_one_idx/bit_scatter_rng_ref
                        time:   [2.3716 ns 2.3732 ns 2.3749 ns]
                        thrpt:  [421.07 Melem/s 421.36 Melem/s 421.66 Melem/s]
```

### pdep32

```
pdep32/pdep32           time:   [437.77 ns 437.99 ns 438.24 ns]
                        thrpt:  [2.3366 Gelem/s 2.3379 Gelem/s 2.3391 Gelem/s]
pdep32/pdep32_fallback  time:   [13.745 us 13.896 us 14.029 us]
                        thrpt:  [72.992 Melem/s 73.692 Melem/s 74.499 Melem/s]
```

### pdep32_one_input

```
pdep32_one_input/pdep32 time:   [632.12 ps 632.48 ps 632.90 ps]
pdep32_one_input/pdep32_fallback
                        time:   [10.312 ns 10.562 ns 10.819 ns]
```

### select64

```
select64/select64       time:   [437.57 ns 437.79 ns 438.19 ns]
                        thrpt:  [2.3369 Gelem/s 2.3390 Gelem/s 2.3402 Gelem/s]
select64/select64_via_pdep32
                        time:   [1.0649 us 1.2104 us 1.3783 us]
                        thrpt:  [742.97 Melem/s 846.01 Melem/s 961.59 Melem/s]
select64/select64_fallback
                        time:   [6.3729 us 6.3825 us 6.3941 us]
                        thrpt:  [160.15 Melem/s 160.44 Melem/s 160.68 Melem/s]
```
