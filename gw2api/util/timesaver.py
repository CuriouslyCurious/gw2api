#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import sys
from pathlib import Path
import re


def parse_file(path):
    struct_re = re.compile(r"pub struct \w+ {\s*")  # match any pub struct
    comment_re = re.compile(r"^\s*[\/]{2,3}")  # match comments
    blank_re = re.compile(r"^\s*$")  # match blank rows
    hash_re = re.compile(r"^\s*#\[.*\]")  # match any attribute
    test_re = re.compile(r"^mod tests {\s*")  # match the test module
    use_re = re.compile(r"^\s*use .*")  # match use declarations

    with path.open(mode="r") as f:
        in_struct = False
        in_test = False
        comments = []
        names = []
        types = []
        for line in f:
            # If we are at the start of a struct
            if struct_re.fullmatch(line):
                print(line)
                in_struct = True
                continue

            # If we are at the start of the test mod
            if test_re.match(line):
                in_test = True
                continue

            # Skip line if it matches any of the regexes
            if hash_re.match(line) or blank_re.match(line):
                continue
            if not in_struct and comment_re.match(line):
                continue

            if in_struct:
                line = line.strip()
                if line.startswith("}"):
                    in_struct = False
                    continue

                # Extract the comment
                if comment_re.match(line):
                    comment = line.lstrip("/// ")
                    comments.append(comment)
                    continue

                (name, typ) = line.split(":")
                typ = typ.strip(" ,")
                names.append(name)
                names.append(typ)
                print((name, typ))

            if in_test:
                # If we are at the end of the test module we append new tests for accessors by
                # parsing the json file and adding them on
                if line.startswith("}"):
                    in_test = False

                    print(line)
                    f.seek(-1, whence=SEEK_CUR)
                    print(f.readline)

                    for struct in structs:
                        prelude = fr"""
    #[test]
    fn {struct.lower()}_accessors() \{
        let {struct.lower()} = serde_json::from_str::<{struct}>(JSON_{struct.upper()}).unwrap();
                        """
                        print(prelude)

                        for n in range(len(comments)):
                            s = f"assert_eq!()"
                            print(n)
                            print(comments[n])
                            print(names[n])
                            print(types[n])


                        epilogue = fr"""
    }
                        """


if __name__ == "__main__":
    if len(sys.argv) == 1:
        print("Please supply a list of rust source files to parse.")
        sys.exit(0)

    for line in sys.argv[1:]:
        try:
            path = Path(line)

            if path.suffix != ".rs":
                raise ValueError("ValueError: This parser will only work on .rs files.")

            if path.exists() and path.is_file():
                parse_file(path)
            else:
                raise OSError("OSError: File does not exist.")
        except Exception as e:
            print(e, file=sys.stderr)
            sys.exit(-1)
