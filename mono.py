"""Read, convert and compare monotonic."""

import sys

from poly2mono.main import poly2mono
from grac import to_mono
from modern_greek_accentuation.accentuation import convert_to_monotonic
from time import time


def tm(func, *args, **kwargs):
    start = time()
    result = func(*args, **kwargs)
    end = time()
    print(f"{func.__name__}: {end - start:.6f}s")
    return result


def print_rust_test(word: str, expected: str) -> None:
    print(f'    ["{word}", "{expected}"],')


def main():
    fpath = sys.argv[1]

    with open(fpath, "r") as f:
        content = f.read()

    cont = content.split()
    poly = tm(poly2mono, content).split()
    grac = tm(to_mono, content).split()
    mgaa = grac
    # Too slow...
    # mgaa = tm(convert_to_monotonic, content).split()

    # Test modern_accentuation too

    for idx, (w, w1, w2, w3) in enumerate(zip(cont, poly, grac, mgaa)):
        if w1 != w2:
            # print(f"{w1} {w2} '{w}' [comparing poly, grac]")
            print(f"Orig: '{w}'")
            bstring = " ".join(f"{byte:02x}" for byte in w.encode("utf-8"))
            print(f"Byte: '{bstring}'")
            print(f"Ctxt: '{cont[idx:idx+3]}'")
            print(f"   P: '{" ".join(cont[idx-1:idx+5])}'")
            print(f"   M: '{" ".join(map(to_mono, cont[idx-1:idx+5]))}'")
            # Relevant
            print(f"Poly: '{w1}'")
            print(f"Grac: '{w2}'")
            # Relevant end
            # print(f"Mgaa: '{w3}'")
            for a, b in zip(w1, w2):
                if a != b:
                    print(f"Difference at char '{a}' != '{b}'")
                    break
            print_rust_test(w, w1)
            # assert False
        # if w1 != w3:
        #     print(f"{w1} {w3} '{w}' [comparing poly, mgaa]")
        #     assert False

    with open("out.txt", "w") as f:
        f.write(to_mono(content))
    print("Wrote monotonic version at out.txt")

    print("All good!")


if __name__ == "__main__":
    main()
