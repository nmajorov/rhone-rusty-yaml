# Rust yaml lib

Library for python integration used as tests for my python projects
and for tests of rust language speed.:

Install dependencies:

    pip install -r requirements-dev.txt

test  rust with command:

    cargo test -- --nocapture

test with python:

    pytest -v tests



benchmark result:


	--------------------------------------------------------------------------------------------- benchmark: 2 tests --------------------------------------------------------------------------------------------
	Name (time in us)                    Min                   Max                Mean              StdDev              Median                IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
	-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
	test_parse_yaml_with_rust       167.8210 (1.0)        330.0870 (1.0)      190.0182 (1.0)       24.5759 (1.0)      183.4780 (1.0)      13.2228 (1.0)       242;262        5.2627 (1.0)        2931           1
	test_parst_yaml_pure_python     814.3130 (4.85)     1,907.1550 (5.78)     937.2391 (4.93)     140.9892 (5.74)     890.3840 (4.85)     69.3948 (5.25)      109;128        1.0670 (0.20)        981           1
	-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

it seems that rust serd parser is **4 till 5  times** faster then python yaml parser.


Python rust links:

https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/

https://github.com/PyO3/setuptools-rust


