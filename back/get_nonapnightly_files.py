#!/usr/bin/env python3

"""
Download the paradigm text files that are not present in the giella-* apertium
nightly repositories.

The script does the following:
    - make directory "non-apnightly-files" in the current working directory
      (name overridable with --target-dir)
    - cd to it
    - write "install.py", a script that copies the files inside the directory
      to /usr/share/giella/ (so the api can see them in the directory)
    - check out giellalt/lang-* for all languages the api supports (see LANGS
      variable in code)
      Notice: does a very sparse checkout, without downloading anything, so
      it's very fast.
    - Checks out the paradigm files, so that they are downloaded (see
      PARADIGM_FILES variable in code)
"""

import argparse
import asyncio
import builtins
import os
import pathlib
import re
import shutil

REPOS_URL = "https://github.com/giellalt/lang-"
LANGS = [
    "bxr", "ciw", "cor", "evn", "fao", "fin", "fit", "fkv", "gle", "ipk",
    "izh", "kal", "kca", "koi", "kpv", "liv", "mdf", "mhr", "mns", "mrj",
    "myv", "nio", "nob", "olo", "rus", "sjd", "sje", "sju-x-sydlapsk", "sma", 
    "sme", "smj", "smn", "sms", "som", "udm", "vep", "vot", "vro", "yrk",
]
TARGET_DIR = "non-apnightly-files"
PARADIGM_FILES = [
    "devtools/testdata/korpustags.{lang}.txt",
    "src/fst/morphology/test/paradigm.{lang}.txt",
    "src/fst/morphology/test/paradigm_min.{lang}.txt",
    "src/fst/morphology/test/paradigm_standard.{lang}.txt",
    "src/fst/morphology/test/paradigm_full.{lang}.txt",
]
PAT_UNKNOWN_IN_GIT = re.compile(
    r"^error: pathspec '([^']*)' did not match any file\(s\) known to git$"
)


# A scripte to transfer the downloaded files to the apertium nightly
# directories. Stored as TARGET_DIR/install.py, so that it can be done in
# two steps.
INSTALL_SCRIPT = """#!/usr/bin/env python3

from pathlib import Path
from shutil import copyfile
from sys import argv

TARGET_DIR = Path("/usr/share/giella")
if len(argv) == 2:
    TARGET_DIR = Path(argv[1])

if not TARGET_DIR.is_dir():
    exit(f"fatal: target directory doesn't exist: {TARGET_DIR}")

def cp(src, dst):
    print(f"cp {src} {dst}")
    try:
        copyfile(src, dst)
    except OSError as e:
        print(f"failed! ({e})")

def all_files(langdir):
    for file in langdir.glob("**/*"):
        if list(file.parts)[1] == ".git":
            continue

        if file.is_file():
            yield file

for langdir in Path(".").glob("lang-*"):
    lang = langdir.name[5:]

    for src in all_files(langdir):
        dst = Path(f"{TARGET_DIR}/{lang}/{src.name}")
        cp(src, dst)

print("all done")
"""


def ok(**kwargs):
    return {"success": True, **kwargs}


def err(**kwargs):
    assert "reason" in kwargs
    return {"success": False, **kwargs}


def is_ok(d):
    return d["success"]


def is_err(d):
    return not d["success"]


async def shell(
    cmd,
    stdin=None,
    cwd=None,
    timeout=None,
    print=builtins.print,
):
    print(f"running: {cmd}")
    proc = await asyncio.subprocess.create_subprocess_shell(
        cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
        cwd=cwd,
    )

    try:
        async with asyncio.timeout(timeout):
            ret = await proc.wait()
    except TimeoutError:
        print(f"running timeout! {cmd}")
        return err(reason="timeout", timeout=timeout, cmd=cmd)

    stderr = (await proc.stderr.read()).decode("utf-8")
    stdout = (await proc.stdout.read()).decode("utf-8")

    if ret != 0:
        return err(reason="retcode", retcode=ret, stdout=stdout, stderr=stderr)

    return ok(stdout=stdout, stderr=stderr)


async def do_lang(lang, print=builtins.print):
    def printer(*msg):
        print(f"lang {lang}:", *msg)

    # this will clone the repo without downloading almost anything at all,
    # and will pull from the repo only when needed
    cmd = (
        "git clone --no-checkout --depth 1 --filter=blob:none "
        f"{REPOS_URL}{lang}"
    )
    result = await shell(cmd, print=printer, timeout=5)
    if is_err(result):
        return err(lang=lang, **result)

    files = [file.format(lang=lang) for file in PARADIGM_FILES]
    missing = []
    for file in files:
        cmd = f"git checkout main -- {file}"
        result = await shell(cmd, cwd=f"lang-{lang}", timeout=10, print=printer)
        if is_err(result):
            stderr = result["stderr"]
            if any(PAT_UNKNOWN_IN_GIT.match(line) for line in stderr.splitlines()):
                missing.append(file)
            else:
                return err(lang=lang, **result)

    return ok(lang=lang, stdout="", stderr="", missing=missing)


def parse_args():
    description = __doc__.split("\n\n")[0]
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument(
        "-o",
        "--target-dir",
        type=pathlib.Path,
        default=TARGET_DIR,
        help=(
            "The directory to store the repos with the downloaded files in. "
            f"Default '{TARGET_DIR}'"
        ),
    )
    parser.add_argument(
        "-l",
        "--langs",
        nargs="+",
        action="extend",
        help="Only get files for the these languages. By default: all langs",
    )
    args = parser.parse_args()
    return args


async def main():
    args = parse_args()

    target_dir = args.target_dir
    langs = args.langs or LANGS

    try:
        print(f"rm -rf {target_dir}")
        shutil.rmtree(target_dir)
    except FileNotFoundError:
        pass

    target_dir.mkdir()
    print(f"cd {target_dir}")
    os.chdir(target_dir)
    print("create install.py")
    with open("install.py", "w") as f:
        f.write(INSTALL_SCRIPT)

    tasks = [do_lang(lang) for lang in langs]

    try:
        results = await asyncio.gather(*tasks, return_exceptions=True)
    except asyncio.exceptions.CancelledError:
        return

    for result in results:
        if isinstance(result, Exception):
            print(result)
        elif is_ok(result):
            #print(result["lang"], "ok")
            if missing := result.get("missing"):
                print(result["lang"], "missing:")
                print("\n".join(missing))
        else:
            error = result["reason"]
            if error == "timeout":
                timeout = result["timeout"]
                error += f" after {timeout}s"
            elif error == "retcode":
                retcode = result["retcode"]
                stdout = result["stdout"]
                stderr = result["stderr"]
                error += f" return code {retcode}: \n== STDOUT ==\n{stdout}\n== STDERR == \n{stderr}\n"
            print(result["lang"], "ERROR:", error)
        #print(result)


if __name__ == "__main__":
    raise SystemExit(asyncio.run(main()))
