import os
import sys

from setuptools import setup
from setuptools.command.test import test as TestCommand
from setuptools.command.sdist import sdist as SdistCommand
from setuptools.dist import Distribution


def wheel_name(**kwargs):
    # create a fake distribution from arguments
    dist = Distribution(attrs=kwargs)
    # finalize bdist_wheel command
    bdist_wheel_cmd = dist.get_command_obj('bdist_wheel')
    bdist_wheel_cmd.ensure_finalized()
    # assemble wheel file name
    distname = bdist_wheel_cmd.wheel_dist_name
    print("*** " + distname)
    tag = '-'.join(bdist_wheel_cmd.get_tag())
    return f'{distname}-{tag}.whl'


#import rhone_rusty_yaml
try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension


class CargoModifiedSdist(SdistCommand):
    """Modifies Cargo.toml to use an absolute rather than a relative path

    The current implementation of PEP 517 in pip always does builds in an
    isolated temporary directory. This causes problems with the build, because
    Cargo.toml necessarily refers to the current version of pyo3 by a relative
    path.

    Since these sdists are never meant to be used for anything other than
    tox / pip installs, at sdist build time, we will modify the Cargo.toml
    in the sdist archive to include an *absolute* path to pyo3.
    """

    def make_release_tree(self, base_dir, files):
        """Stages the files to be included in archives"""
        super().make_release_tree(base_dir, files)

        import toml

        # Cargo.toml is now staged and ready to be modified
        cargo_loc = os.path.join(base_dir, "Cargo.toml")
        assert os.path.exists(cargo_loc)

        with open(cargo_loc, "r") as f:
            cargo_toml = toml.load(f)

        rel_pyo3_path = cargo_toml["dependencies"]["pyo3"]["path"]
        base_path = os.path.dirname(__file__)
        abs_pyo3_path = os.path.abspath(os.path.join(base_path, rel_pyo3_path))

        cargo_toml["dependencies"]["pyo3"]["path"] = abs_pyo3_path

        with open(cargo_loc, "w") as f:
            toml.dump(cargo_toml, f)


class PyTest(TestCommand):
    user_options = []

    def run(self):
        self.run_command("test_rust")

        import subprocess

        subprocess.check_call(["pytest", "tests"])


setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []
tests_require = install_requires + ["pytest", "pytest-benchmark"]

# Utility function to read the README file.
# Used for the long_description.  It's nice, because now 1) we have a top level
# README file and 2) it's easier to type in the README file than to put a raw
# string in below ...
def read(fname):
    return open(os.path.join(os.path.dirname(__file__), fname)).read()

setup(
    name="rhone-rusty-yaml",
    version="1.0.2",
    author= "Nikolaj Majorov",
    author_email = "nikolaj@majorov.biz",
    description = ("parse rhone yaml files with rust in python"),
    license = "BSD",
    long_description=read('ReadMe.md'),
    url="https://github.com/nmajorov/rhone-rusty-yaml",
     project_urls={
        "Bug Tracker": "https://github.com/nmajorov/rhone-rusty-yaml",
        "Documentation": "https://github.com/nmajorov/rhone-rusty-yaml",
        "Source Code": "https://github.com/nmajorov/rhone-rusty-yaml",
    },
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
        "License :: OSI Approved :: BSD License"
    ],
    packages=["rhone_rusty_yaml"],
    rust_extensions=[RustExtension("rhone_rusty_yaml.rhone_rusty_yaml", "Cargo.toml", debug=True)],
    install_requires=install_requires,
    tests_require=tests_require,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    python_requires='>=3.7',
    cmdclass={"test": PyTest, "sdist": CargoModifiedSdist},
)
