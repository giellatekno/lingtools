#!/usr/bin/env python3

"""
Collect all needed files from local lang-* repos.
Some files (transcriptors) must be compiled locally first.
"""

import argparse
import shutil
from pathlib import Path

REPOS_DIR = Path.home() / "gut" / "giellalt"
TARGET_DIR = Path("non-apnightly-files")

# fmt: off

# Langs I'm unsure if should be added:
# crk, sjd, est
LANGS = [
    "bxr", "ciw", "cor", "evn", "fao", "fin", "fit", "fkv", "gle", "hdn", "ipk",
    "izh", "kal", "kca", "koi", "kpv", "liv", "mdf", "mhr", "mns", "mrj",
    "myv", "nio", "nob", "olo", "rmf", "rus", "sje", "sju-x-sydlapsk", "sma",
    "sme", "smj", "smn", "sms", "som", "udm", "vep", "vot", "vro", "yrk",
]

TRANSCRIPTION_FILES = [
    "src/fst/transcriptions/transcriptor-numbers-digit2text.filtered.lookup.hfstol",
    "src/fst/transcriptions/transcriptor-clock-digit2text.filtered.lookup.hfstol",
    "src/fst/transcriptions/transcriptor-date-digit2text.filtered.lookup.hfstol",
]


KORPUSTAGS_FILE = [
    "devtools/testdata/korpustags.{lang}.txt",
    "src/fst/morphology/test/korpustags.{lang}.txt",
]

PARADIGM_FILES = [
    ["src/fst/morphology/test/paradigm.txt", "devtools/testdata/paradigm.txt"],
    [ "src/fst/morphology/test/paradigm.{lang}.txt", "devtools/testdata/paradigm.{lang}.txt", ],
    [ "src/fst/morphology/test/paradigm_min.{lang}.txt", "devtools/testdata/paradigm_min.{lang}.txt", ],
    [ "src/fst/morphology/test/paradigm_standard.{lang}.txt", "devtools/testdata/paradigm_standard.{lang}.txt", ],
    [ "src/fst/morphology/test/paradigm_full.{lang}.txt", "devtools/testdata/paradigm_full.{lang}.txt", ],
]

SJU_X_SYDLAPSK_FILES = [
    "tools/tokenisers/tokeniser-disamb-gt-desc.pmhfst",
    "src/fst/analyser-gt-desc.hfstol",
    "src/fst/analyser-gt-norm.hfstol",
    "src/fst/generator-gt-norm.hfstol",
    "src/fst/generator-gt-desc.hfstol",
    "src/cg3/functions.cg3",
    "tools/hyphenators/hyphenator-gt-desc.hfstol",
    "src/cg3/disambiguator.cg3",
    "src/cg3/dependency.cg3",
    "devtools/testdata/korpustags.sju-x-sydlapsk.txt",
    "src/fst/morphology/test/paradigm.sju-x-sydlapsk.txt",
    "src/fst/morphology/test/paradigm_full.sju-x-sydlapsk.txt",
]

DICT_HFST_FILE = "bin/{lang1}{lang2}-all.hfst"
# fmt: on

INSTALL_SCRIPT = """#!/usr/bin/env python3

from pathlib import Path
from shutil import copy2
from sys import argv

TARGET_DIR = Path("/usr/share/giella")
if len(argv) == 2:
    TARGET_DIR = Path(argv[1])

if not TARGET_DIR.is_dir():
    exit(f"fatal: target directory doesn't exist: {TARGET_DIR}")

def cp(src, dst):
    print(f"cp {src} {dst}")
    try:
        copy2(src, dst)
    except OSError as e:
        print(f"failed! ({e})")

for langdir in Path(".").glob("lang-*"):
    lang = langdir.name[5:]
    for src in langdir.glob("*"):
        if not src.is_file():
            continue
        dst_dir = TARGET_DIR / lang
        if not dst_dir.is_dir():
            print(f"skip {lang}: {dst_dir} does not exist")
            continue
        dst = dst_dir / src.name
        cp(src, dst)

print("all done")
"""


def parse_args():
    description = __doc__.split("\n\n")[0]
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument(
        "-o",
        "--target-dir",
        type=Path,
        default=TARGET_DIR,
        help=f"Directory to store collected files in. Default '{TARGET_DIR}'",
    )
    parser.add_argument(
        "-r",
        "--repos-dir",
        type=Path,
        default=REPOS_DIR,
        help=f"GiellaLT directory containing lang-* repos. Default '{REPOS_DIR}'",
    )
    parser.add_argument(
        "-l",
        "--langs",
        nargs="+",
        action="extend",
        help="Only get files for the these languages. By default: all langs",
    )
    parser.add_argument(
        "-v",
        "--verbose",
        action="store_true",
    )
    return parser.parse_args()


def copy_sju_x_sydlapsk(repo: Path, out_lang_dir: Path):
    copied = []
    for file in SJU_X_SYDLAPSK_FILES:
        src = repo / file
        if not src.exists():
            continue
        filename = Path(file).name
        dst = out_lang_dir / filename
        shutil.copy2(src, dst)
        copied.append(filename)
    return copied


def copy_lang_files(lang: str, repos_dir: Path, repo: Path, out_lang_dir: Path):
    copied = []
    # Copy over transcription files
    for filepath in TRANSCRIPTION_FILES:
        src = repo / filepath
        if not src.exists():
            continue

        filename = Path(filepath).name
        dst = out_lang_dir / filename
        shutil.copy2(src, dst)
        copied.append(filename)

    # Copy over paradigm files
    locations = [f.format(lang=lang) for f in KORPUSTAGS_FILE]
    korpustags_file = None
    if (repo / locations[0]).exists():
        korpustags_file = repo / locations[0]
    elif (repo / locations[1]).exists():
        korpustags_file = repo / locations[1]

    if korpustags_file:
        filename = Path(korpustags_file).name
        dst = out_lang_dir / filename
        shutil.copy2(korpustags_file, dst)
        copied.append(filename)

    for file in PARADIGM_FILES:
        locations = [f.format(lang=lang) for f in file]
        paradigm_file = None
        if (repo / locations[0]).exists():
            paradigm_file = repo / locations[0]
        elif (repo / locations[1]).exists():
            paradigm_file = repo / locations[1]

        if paradigm_file:
            filename = Path(paradigm_file).name
            dst = out_lang_dir / filename
            shutil.copy2(paradigm_file, dst)
            copied.append(filename)

    # Copy over dict hfst files
    dict_repos = sorted(repos_dir.glob(f"dict-{lang}-???"))
    for dict_repo in dict_repos:
        tr_lang = dict_repo.name[9:]
        dict_hfst_file = dict_repo / DICT_HFST_FILE.format(lang1=lang, lang2=tr_lang)

        if dict_hfst_file.exists():
            filename = Path(dict_hfst_file).name
            dst = out_lang_dir / filename
            shutil.copy2(dict_hfst_file, dst)
            copied.append(filename)
    return copied


def get_lang_files(repos_dir: Path, target_dir: Path, langs: list[str], verbose: bool):
    for lang in langs:
        repo = repos_dir / f"lang-{lang}"
        if not repo.exists():
            print(f"{lang}: repo not found")
            continue

        out_lang_dir = target_dir / f"lang-{lang}"
        out_lang_dir.mkdir(exist_ok=True)

        if lang == "sju-x-sydlapsk":
            copied = copy_sju_x_sydlapsk(repo, out_lang_dir)
        else:
            copied = copy_lang_files(lang, repos_dir, repo, out_lang_dir)

        if len(copied) == 0:
            shutil.rmtree(out_lang_dir)
        print(f"{lang}: copied {len(copied)} file(s)")
        if verbose:
            for file in copied:
                print(f"\t{file}")


def main():
    args = parse_args()
    target_dir = args.target_dir
    repos_dir = args.repos_dir

    if not repos_dir.is_dir():
        exit(f"fatal: {repos_dir} does not exist")

    shutil.rmtree(target_dir, ignore_errors=True)
    target_dir.mkdir()

    install_path = target_dir / "install.py"
    install_path.write_text(INSTALL_SCRIPT)

    langs = args.langs if args.langs else LANGS

    get_lang_files(repos_dir, target_dir, langs, args.verbose)


if __name__ == "__main__":
    raise SystemExit(main())
