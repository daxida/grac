from greek_accentuation.syllabify import syllabify as syl1
from grac import syllabify as syl3
from grac import syllabify_ref as syl2
from pathlib import Path
import time


def timeit(fn, words, version, ref_elapsed=0.0):
    start_time = time.time()
    for word in words:
        fn(word)
    elapsed = time.time() - start_time
    res = [fn(word) for word in words]
    delta_str = ""
    if ref_elapsed:
        delta = 100 * (ref_elapsed - elapsed) / ref_elapsed
        delta_str = f"[Δ={delta:.2f}%]"
    print(f"syllabify{version} took {elapsed:.4f}s {delta_str}")
    return res, elapsed


def split_words(text: str) -> list[str]:
    words = []
    for line in text.splitlines():
        words.extend(line.split(" "))
    return words


def main():
    start_time = time.time()

    ipath = Path("dump.txt")
    with ipath.open("r", encoding="utf-8") as f:
        text = f.read()

    for times in (10,):
        words = split_words((text * times))
        print(f"Testing with {len(words)} words")

        for _ in range(1):
            ref, el1 = timeit(syl1, words, "Py")
            res1, _ = timeit(syl2, words, "1", el1)
            res2, _ = timeit(syl3, words, "2", el1)

        for a, b, c, w in zip(ref, res1, res2, words):
            if a != b and b:
                print(f"{a} {b} '{w}' [comparing ref, res1]")
                assert False
            if a != c and c:
                print(f"{a} {c} '{w}' [comparing ref res2]")
                assert False

    elapsed = time.time() - start_time
    print(f"main took {elapsed:.4f}s")


if __name__ == "__main__":
    main()
