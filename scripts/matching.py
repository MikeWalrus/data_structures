#! /usr/bin/python3

import os
import pathlib
from subprocess import Popen, PIPE

def main():
    directory = pathlib.Path(os.path.dirname(__file__))
    binary = directory/"../target/release/matching"

    with open(directory / "matching_test_cases.txt") as file:
        while True:
            text = file.readline()[:-1].encode()
            if not text:
                return
            pattern = file.readline()[:-1].encode()
            pos = text.find(pattern)
            matches = pos != -1

            with Popen(binary, shell=False, stdin=PIPE, stdout=PIPE) as p:
                out, err = p.communicate(input=text+"\n".encode()+pattern+"\n".encode())
                out = out.splitlines()
                if matches:
                    try:
                        result = (int(out[1]), int(out[3]))
                        if not all([b == pos for b in result]):
                            raise ValueError
                    except ValueError:
                        quit_with_error(text, pattern, (out[1], out[3]), pos)
                else:
                    if not out[1] == "No match." and out[3] == "No match.":
                        quit_with_error(text, pattern, (out[1], out[3]), pos)


def quit_with_error(text, pattern, result, expect):
    print(f"Text: {text}\nPattern: {pattern}")
    print("Result: ", result)
    print("Expect: ", expect)
    quit(1)

if __name__ == "__main__":
    main()
    print("All tests passed.")
