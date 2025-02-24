"""Read, convert and compare monotonic."""

import sys
from time import time
from typing import Callable

from grac import to_monotonic
from modern_greek_accentuation.accentuation import convert_to_monotonic
from poly2mono.main import poly2mono


def tm(func, *args, **kwargs):
    start = time()
    result = func(*args, **kwargs)
    end = time()
    print(f"{func.__name__}: {end - start:.6f}s")
    return result


def print_rust_test(word: str, expected: str) -> None:
    print(f'    ["{word}", "{expected}"],')


def get_ctx(words: list[str], idx: int, fn: Callable[[str], str]) -> str:
    ctx = [fn(word) for word in words[idx - 1 : idx + 5]]
    ctx[1] = f"[[{ctx[1]}]]"
    return " ".join(ctx)


def main() -> None:
    try:
        fpath = sys.argv[1]
    except IndexError:
        fpath = "dump.txt"
        # print("Error: needs a path to the text file.")
        # return

    with open(fpath, "r") as f:
        lines = f.readlines()
        content = "".join(line for line in lines if not line.startswith("---"))

    cont = content.split()

    suite: dict[str, list[str]] = {
        "orig": cont,
        "poly": tm(poly2mono, content).split(),
        "grac": tm(to_monotonic, content).split(),
        # Too slow...
        "mgaa": tm(convert_to_monotonic, content).split(),
        # "mgaa": grac,
    }
    print("------------")

    labels = [label.capitalize() for label in suite]
    suite_it = enumerate(zip(*suite.values()))
    cnt = 0

    # Make the logs more succint in exchange of possible loss of information
    seen_words = set()

    for idx, (w, w1, w2, w3) in suite_it:
        lw, lw1, lw2, lw3 = labels
        if w1 != w2:
            if {w1, w2} & seen_words:
                continue
            seen_words |= {w1, w2}
            cnt += 1

            # print(f"{w1} {w2} '{w}' [comparing poly, grac]")
            print(f"{lw}: '{w}'")
            bstring = " ".join(f"{byte:02x}" for byte in w.encode("utf-8"))
            print(f"Byte: '{bstring}'")
            print()
            print("Ctxt:")
            print(f"  {lw}: '{get_ctx(cont, idx, lambda x: x)}'")
            print(f"  {lw1}: '{get_ctx(cont, idx, poly2mono)}'")
            print(f"  {lw2}: '{get_ctx(cont, idx, to_monotonic)}'")
            print(f"  {lw3}: '{get_ctx(cont, idx, convert_to_monotonic)}'")

            # Relevant end
            for a, b in zip(w1, w2):
                if a != b:
                    print(f"Difference at char '{a}' != '{b}'")
                    break
            print_rust_test(w, w1)
            print("===============")
            # assert False
        # if w1 != w3:
        #     print(f"{w1} {w3} '{w}' [comparing poly, mgaa]")
        #     assert False

    with open("out.txt", "w") as f:
        f.write(to_monotonic(content))
    print("Wrote monotonic version at out.txt")

    print()
    print("All good!")
    print(f"There were {cnt} differences")


if __name__ == "__main__":
    main()
