#!/usr/bin/env python3

import argparse
import json
import io
import re
import sys
import xml.etree.ElementTree as ET

# there's also an external python lib called
# lxml, which may have more features, like
# reading dtd, or at least doing dtd verification.
# don't know if I need that or not

HTML_TAGS = ["a", "em", "strong", "code"]


def inner_xml(node):
    """Get all "inner xml", which includes any potential tags."""
    text = node.text or ""
    tail = node.tail or ""
    s = text
    for child in node:
        tag = child.tag
        attribs = " ".join(
            f'"{k}"="{v}"'
            for k, v in child.items()
        )
        inner = inner_xml(child)
        s += f"<{tag}"
        if attribs:
            s += " " + attribs
        s += f">{inner}</{tag}>"
    s += tail
    return s.strip()


def real_len(node):
    """How many children does this node have, that are
    just normal html tags that we should not recurse into."""
    total_children = len(node)
    ignored_children = 0
    for child in node:
        if child.tag in HTML_TAGS:
            ignored_children += 1
    return total_children - ignored_children


def _traverse(strings, node, pre):
    section = f"{pre}.{node.tag}."
    if real_len(node) == 0:
        # leaf
        section += ".".join(f"{k}.{v}." for k, v in node.items())
        strings[section] = inner_xml(node)
    else:
        for child in node:
            _traverse(strings, child, section)


def clean_key(key):
    i0 = 0
    if key.startswith(".content."):
        i0 = 10 
    elif key.startswith("."):
        i0 = 1
    i1 = -1 if key.endswith(".") else len(key)
    return key[i0:i1].replace("..", ".")


def traverse(node):
    strings_in = {}
    _traverse(strings_in, node, "")

    return {clean_key(k): v for k, v in strings_in.items()}


def parse_args():
    parser = argparse.ArgumentParser(prog="xmltojson")
    parser.add_argument("infile", type=argparse.FileType("r"))
    parser.add_argument("--dtd", nargs="*", type=argparse.FileType("r"))

    parsed = parser.parse_args()
    dtdfiles = [] if parsed.dtd is None else parsed.dtd
    return parsed.infile, dtdfiles


entity_re = r"<!ENTITY\s+(?P<entity>\w+)\s+\"(?P<val>.+)\">"
def parse_dtd(file):
    """Reads a dtd file, and return a dict of the entites"""
    lines = file.read()
    entities = {}
    for m in re.finditer(entity_re, lines):
        entity = m.group("entity")
        val = m.group("val")

        # do one level of substitution from previously defined entities
        # theoretically should do this differently, but this is sufficient here
        val = re.sub(re.compile(r"&(\w+);"), lambda m: entities[m.group(1)], val)
        entities[entity] = val
    return entities


def merge_dicts(dicts):
    out = dict()
    for d in dicts:
        for k, v in d.items():
            if k in out:
                print(f"key {k} already exists")
                sys.exit()
            out[k] = v
    return out


def replace_entities(infile, dtd_defs):
    """Replace entites found in infile (file) with
    dictionary lookup in dtd_defs (dict). Return a new (in-memory) file object of
    the xml data with entites replaced."""
    infile_s = infile.read()
    for entity, replacement in dtd_defs.items():
        infile_s = infile_s.replace(f"&{entity};", replacement)
    return io.StringIO(infile_s)


def main(infile, dtdfiles):
    dtd_defs = merge_dicts(parse_dtd(file) for file in dtdfiles)
    infile = replace_entities(infile, dtd_defs)

    xmltree = ET.parse(infile)
    root = xmltree.getroot()
    strings = traverse(root)

    print(json.dumps(strings, indent=4))


if __name__ == "__main__":
    infile, dtdfiles = parse_args()
    main(infile, dtdfiles)

