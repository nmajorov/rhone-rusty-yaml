# -*- coding: utf-8 -*-
import os

import pytest
import rhone_rusty_yaml

current_dir = os.path.abspath(os.path.dirname(__file__))
path = os.path.join(current_dir, "rhone.yaml")


@pytest.fixture(scope="session", autouse=True)
def textfile():
    text = """
---
apiVersion: build.rhone.io/v1
name: simple-go
version: v1.dev
language: golang
interpreter-version: 1.13.6
build_trigger: none

"""

    with open(path, "w") as f:
        f.write(text * 1000)
    yield
    os.remove(path)


def test_parse_yaml_with_rust(benchmark):
    json = benchmark(rhone_rusty_yaml.get_json_from_yaml(path))
    assert json is not None
