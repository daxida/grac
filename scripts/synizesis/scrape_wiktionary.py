"""Scrape wiktionary for words with synizesis at the last syllable.

Ignores the pronoun category since it only contains τέτοιος.
https://el.wiktionary.org/wiki/Κατηγορία:Αντωνυμίες_με_συνίζηση_στην_κατάληξη_(νέα_ελληνικά)
"""

import re
import urllib.parse
from pathlib import Path

import requests
from bs4 import BeautifulSoup

ppath = Path("scripts/synizesis/data")
output_path = ppath / "wiki.txt"

BASE_URL = "https://el.wiktionary.org"
CATEGORY_URLS = [
    (
        "adj",
        "https://el.wiktionary.org/wiki/Κατηγορία:Επίθετα_με_συνίζηση_στην_κατάληξη_(νέα_ελληνικά)",
    ),
    # "https://el.wiktionary.org/wiki/Κατηγορία:Ουσιαστικά_με_συνίζηση_στην_κατάληξη_(νέα_ελληνικά)",
    (
        "noun_masc",
        "https://el.wiktionary.org/wiki/Κατηγορία:Ουσιαστικά_αρσενικά_με_συνίζηση_στην_κατάληξη_(νέα_ελληνικά)",
    ),
    (
        "noun_fem",
        "https://el.wiktionary.org/wiki/Κατηγορία:Ουσιαστικά_θηλυκά_με_συνίζηση_στην_κατάληξη_(νέα_ελληνικά)",
    ),
    (
        "noun_neut",
        "https://el.wiktionary.org/wiki/Κατηγορία:Ουσιαστικά_ουδέτερα_με_συνίζηση_στην_κατάληξη_(νέα_ελληνικά)",
    ),
]
"""(label, url)"""

HEADERS = {
    "User-Agent": (
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
        "AppleWebKit/537.36 (KHTML, like Gecko) "
        "Chrome/120.0.0.0 Safari/537.36"
    ),
}


def extract_category(url: str) -> str:
    return url.split("/")[-1].replace("_", " ")


def scrape_category(url: str) -> list[str]:
    words = []
    category = None

    while url:
        print(f"Requesting {urllib.parse.unquote(url)}")
        response = requests.get(url, headers=HEADERS)
        if response.status_code != 200:
            print(f"Failed to fetch page: {response.status_code}")
            break

        soup = BeautifulSoup(response.text, "html.parser")

        # select <div id="mw-pages"> first
        selector = "#mw-pages .mw-category-group ul li a"
        words.extend(a.text for a in soup.select(selector))

        if category is None:
            category = extract_category(url)

        next_page = soup.select_one(f"a[title='{category}']:-soup-contains('επόμενη σελίδα')")
        url = ""
        if next_page:
            url = f"{BASE_URL}{next_page['href']}"

    return words


def extract_label(raw: str) -> str:
    return re.findall(r"adj|noun_fem|noun_masc|noun_neut", raw)[0]


def postprocess(labelled_words: dict[str, list[str]]) -> dict[str, list[str]]:
    """Similar to extract_neuter::filter_neuter."""
    words_by_category = {}

    for label_or_stem, words in labelled_words.items():
        # If we are reading from a file...
        label = extract_label(label_or_stem)

        match label:
            case "adj":
                allowed_suffixes = ["ιος"]
            case "noun_masc":
                allowed_suffixes = ["ιας", "ιος"]
            case "noun_fem":
                allowed_suffixes = ["ια"]
            case "noun_neut":
                # Only το βιος ends in ιος, and it is included elsewhere
                allowed_suffixes = ["ιο", "ια"]
            case _:
                raise RuntimeError(f"Unexpected label {label}")

        filtered_words = set()
        for word in words:
            if word[0].isupper():
                continue
            if word[0] == "-":
                continue
            if len(word) < 2:
                continue
            if not any(word.endswith(suf) for suf in allowed_suffixes):
                # print(f"Banned ({label=}): {word}")
                continue
            filtered_words.add(word)

        print(f"{label = }")
        print(f"  Selected {len(filtered_words)} from {len(words)}:")

        for suf in allowed_suffixes:
            category = f"{label}_{suf}"
            words_with_suf = [word for word in filtered_words if word.endswith(suf)]
            if words_with_suf:
                words_by_category[category] = sorted(words_with_suf)
                print(f"    * words with suffix {suf}: {len(words_with_suf)}")

    pre_size = sum(len(x) for x in labelled_words.values())
    cur_size = sum(len(x) for x in words_by_category.values())
    print(f"(Total) Selected {cur_size} from {pre_size}")

    return words_by_category


def label_path(label: str) -> Path:
    return output_path.with_stem(f"{output_path.stem}_{label}")


def main() -> None:
    download = True
    print(f"Download set to {download}.")

    if download:
        print("Downloading")
        labelled_words = {}
        for label, category in CATEGORY_URLS:
            category_words = scrape_category(category)
            sorted_words = sorted(set(category_words))
            labelled_words[label] = sorted_words
    else:
        print("Skipped download")
        labelled_words = {}
        for path in ppath.glob("wiki*.txt"):
            label = path.stem[5:]  # remove the wiki_ prefix
            labelled_words[label] = path.read_text().split("\n")

    labelled_words = postprocess(labelled_words)

    for label, words in labelled_words.items():
        path = label_path(label)
        with path.open("w") as f:
            f.write("\n".join(words))


if __name__ == "__main__":
    main()
