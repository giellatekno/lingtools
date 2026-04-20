#!/usr/bin/env python
import json
import subprocess
import sys

LANGS = ["sme", "eng", "nob", "rus", "fin"]


required_files = [
    dict(filename="data/new-{lang}.xml", dtds=[]),
    dict(filename="data/cgi-{lang}.xml", dtds=[]),
    dict(filename="data/index-{lang}.xml", dtds=["data/can{lang}.dtd"]),
]


def process(lang):
    # first resolve files
    files = [
        {
            "filename": file["filename"].format(lang=lang),
            "dtds": [dtd.format(lang=lang) for dtd in file["dtds"]],
        }
        for file in required_files
    ]

    # run them
    jsons = []
    for file in files:
        prog = ["python", "xmltojson.py", file["filename"]]
        for dtd in file["dtds"]:
            prog.extend(["--dtd", dtd])
        try:
            res = subprocess.run(prog, text=True, capture_output=True)
        except Exception as e:
            print("unhandled exception:", e)
            sys.exit()

        if res.stderr != "":
            print(f"error when running {prog}: {res.stderr}")
            sys.exit()

        jsons.append(res.stdout)

    # merge them
    jsons = [json.loads(j) for j in jsons]
    obj = {}

    for j in jsons:
        for k, v in j.items():
            if k in obj:
                print("conflict on key", k)
                sys.exit()
            obj[k] = v

    with open(f"{lang}.json", "w") as f:
        json.dump(obj, f)


def main():
    for lang in LANGS:
        process(lang)


if __name__ == "__main__":
    raise SystemExit(main())
