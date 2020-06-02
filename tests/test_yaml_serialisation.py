# -*- coding: utf-8 -*-
import os

import pytest
import logging
import yaml
import json
import rhone_rusty_yaml

LOGGER = logging.getLogger(__name__)

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
        f.write(text)
    yield
    os.remove(path)


def test_parse_yaml_file_with_rust(benchmark):
    LOGGER.info("parse yaml file: {}".format(path))
    json = benchmark(rhone_rusty_yaml.get_json_from_yaml_file, path)
    LOGGER.info("get json: {}".format(json))
    assert json is not None


def test_parse_yaml_string_with_rust(benchmark):
    yaml = """
---
name: express-train
apiVersion: build.rhone.io/v1
description: some framework
version: 2.1.3
language: python
interpreter-version: '3.4'
contributors:
- name: Nikolaj Majorov
  email: nikolaj@majorov.biz
- name: Oleg Mayer
  email: oleg@majorov.biz
repository:
  brache: develop
build_trigger:
  every: 5 minutes
notify:
  success: 'false'
  failure: 'true'
"""


    LOGGER.info("parse yaml string: {}".format(yaml))
    json = benchmark(rhone_rusty_yaml.get_json_from_yaml_str, yaml)
    LOGGER.info("get json: {}".format(json))
    assert json is not None


def test_parst_yaml_pure_python(benchmark):
 
    LOGGER.info("pure python parse yaml: {}".format(path))
    
    @benchmark
    def result():
        with open(path) as f:
            jsonStr = json.dumps(yaml.safe_load(f))
        return jsonStr

    LOGGER.info("get json: {}".format(result))
    assert result is not None
    