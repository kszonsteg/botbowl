import os
import platform
import shutil
import sysconfig
from distutils.ccompiler import new_compiler
from distutils.errors import DistutilsPlatformError

from setuptools import find_packages, setup
from setuptools_rust import RustExtension

# Warn if setup.py is not run from the project root
dir_of_file = os.path.dirname(__file__)
cwd = os.getcwd()
if dir_of_file != cwd:
    print(
        f"WARNING: setup.py is being run from a different directory than the install script. This can cause strange errors"
        f"\nCurrent directory='{cwd}', dir of file='{dir_of_file}'"
    )

try:
    error_msg = None
    import Cython.Compiler.Options
    from Cython.Build import cythonize

    Cython.Compiler.Options.annotate = True
    compile_available = True

    if platform.system() == "Windows":
        try:
            new_compiler().initialize()
        except AttributeError:
            compile_available = False
            error_msg = "No compatible Windows compiler"
    else:
        compile_available = "CXX" in sysconfig.get_config_vars()
        if not compile_available:
            error_msg = "No compiler found"

except ImportError:
    error_msg = "Cython could not be imported"
    compile_available = False
except DistutilsPlatformError:
    error_msg = "No compiler found"
    compile_available = False

files_to_compile = ["botbowl/core/pathfinding/cython_pathfinding.pyx"]

kwargs = {
    "name": "botbowl",
    "version": "1.1.0",
    "include_package_data": True,
    "packages": find_packages(),
    "zip_safe": False,
    "rust_extensions": [RustExtension("botbowl.rust_core", path="Cargo.toml")],
}

if compile_available:
    kwargs["ext_modules"] = cythonize(files_to_compile, annotate=True)

setup(**kwargs)

if compile_available:
    compiled_file_type = ".pyd" if platform.system() == "Windows" else ".so"
    copied_files = 0
    for root, dirs, files in os.walk("./build/"):
        for file in files:
            if file.endswith(compiled_file_type):
                if platform.system() == "Windows":
                    root = root.replace("\\", "/")
                from_file = f"{root}/{file}"
                to_file = "./botbowl/" + root.split("/botbowl/")[1] + "/" + str(file)
                print(f"Copying '{from_file}' -> '{to_file}'")
                shutil.copyfile(from_file, to_file)
                copied_files += 1

    print("\nYou've built botbowl with Cython.")
else:
    print(
        f"You've built botbowl without Cython compilation, error message='{error_msg}'. "
        f"Check docs/installation.md for details."
    )
