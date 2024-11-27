from grac import to_mono
from greek_accentuation.syllabify import syllabify as syl1
from grac import syllabify_gr_ref as syl2
from grac import syllabify_gr as syl3
from modern_greek_accentuation.syllabify import modern_greek_syllabify as msyl1
from grac import syllabify_el as msyl2
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


def print_rust_test(word: str, syllables: list[str]) -> None:
    print(f"    [\"{word}\", \"{"-".join(syllables)}\"],")


def main():
    start_time = time.time()

    ipath = Path("dump.txt")
    with ipath.open("r", encoding="utf-8") as f:
        text = f.read()

    mono_diff = 0

    for times in (10,):
        cur_text = text * times
        words = split_words(cur_text)
        print(f"Testing with {len(words)} words")

        # ref, el1 = timeit(syl1, words, "Py")
        # res1, _ = timeit(syl2, words, "1", el1)
        # res2, _ = timeit(syl3, words, "2", el1)
        #
        # for a, b, c, w in zip(ref, res1, res2, words):
        #     if a != b and b:
        #         print(f"{a} {b} '{w}' [comparing ref, res1]")
        #         assert False
        #     if a != c and c:
        #         print(f"{a} {c} '{w}' [comparing ref res2]")
        #         assert False

        # Modern greek

        start_time_to_mono = time.time()
        mwords = to_mono(cur_text)
        print(f"Took {time.time() - start_time_to_mono:.4f}s for to_mono")
        words = split_words(mwords)
        print(f"Testing with {len(words)} words")
        mref, mel1 = timeit(msyl1, words, "Py")
        mres1, _ = timeit(msyl2, words, "1", mel1)

        for a, b, w in zip(mref, mres1, words):
            if a != b and b:
                print(f"{a} {b} '{w}'")
                print_rust_test(w, a)
                mono_diff += 1
                # assert False

    print(f"Mono diff: {mono_diff}")
    elapsed = time.time() - start_time
    print(f"main took {elapsed:.4f}s")


if __name__ == "__main__":
    main()
