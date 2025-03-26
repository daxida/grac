"""Scrape and parse https://www.greek-language.gr/ for various lemmas.

Used to determine if we should merge ai/oi clusters (αη, άη, αϊ, άι etc.)
"""

import requests
from bs4 import BeautifulSoup
from grac import remove_all_diacritics
from dataclasses import dataclass
import re


@dataclass
class Lemma:
    word: str
    text: str
    is_merged: bool

    def __str__(self) -> str:
        return f"({self.word}, {self.is_merged})"


def double_check(query: str, word: str) -> bool:
    nquery = remove_all_diacritics(query)
    nword = remove_all_diacritics(word)
    return nquery.strip("*") in nword


def parse_html(query: str, text: str) -> list[Lemma]:
    soup = BeautifulSoup(text, "html.parser")
    lemmas_container = soup.find(id="lemmas")
    lemmas: list[Lemma] = []
    for entry in lemmas_container.find_all("dl"):  # type: ignore
        text = entry.text.strip()
        # Sometimes the merging is represented with this underline:
        is_merged = bool(entry.find("span", class_="arcChar"))  # type: ignore
        terms_section = text.split(" : ")[0]
        terms = terms_section.split(" & ")
        for term in terms:
            word = term.split()[0]
            # Sometimes the merging is represented with parens:
            # ['k(ai)menáki'] < καημενάκι
            pronunciation = re.findall(r"\[([^\]]+)\]", term)
            if pronunciation and "(" in pronunciation[0]:
                is_merged = True
            if not double_check(query, word):
                # Can happen if we query "*αη*" and we find αέναος -ν -ο
                # where the αη match occurs at the αέναη declination.
                continue
            if ("αη" in query and "αή" in word) or ("οη" in query and "οή" in word):
                # We only want άη αη όη οη
                continue
            lemma = Lemma(word, text, is_merged)
            lemmas.append(lemma)
    return lemmas


def scrape_dictionary(query: str) -> list[Lemma]:
    base_url = "https://www.greek-language.gr/greekLang/modern_greek/tools/lexica/triantafyllides/search.html"
    start = 0
    lemmas: list[Lemma] = []
    seen = set()

    while True:
        url = f"{base_url}?lq={query}&dq=&start={start}"

        response = requests.get(url)
        if response.status_code != 200:
            print("Failed to fetch the webpage.")
            break

        if start == 0:
            # Just check that we don't have to walk a very long pagination
            soup = BeautifulSoup(response.text, "html.parser")
            toolbar = soup.find(id="toolbar")
            link = toolbar.find(id="last").find("a")["href"]  # type: ignore
            last_page = int(link.split(", ")[1][1:-3])  # type: ignore
            if last_page > 1000:
                print(f"{last_page = } over 1000, that's a lot of requests...")
                return []

        page_lemmas = parse_html(query, response.text)
        # When the pagination is over the limit, the site does not return an empty
        # page, but instead returns to the first page (maybe because it ignores the
        # start=X part altogether.
        key = ";".join(pl.text for pl in page_lemmas)
        if key in seen:
            break
        seen.add(key)

        lemmas.extend(page_lemmas)
        start += 10

    return lemmas


def run_query(query: str) -> None:
    lemmas = scrape_dictionary(query)
    if not lemmas:
        print("No lemmas found.")
        return

    merged_cnt = {True: [], False: []}
    for lemma in lemmas:
        word = lemma.word
        merged_cnt[lemma.is_merged].append(word)

    merged_words = merged_cnt[True]
    not_merged_words = merged_cnt[False]

    print(f"Merged words summary for {query = }:")
    # print(f"* Merged words ({len(merged_words)}):    \n{merged_words}")
    # print(f"* Not merged words ({len(not_merged_words)}):\n{not_merged_words}")
    print(f"* Merged words ({len(merged_words)})")
    print(f"* Not merged words ({len(not_merged_words)})")


def main() -> None:
    queries = ("*αη*", "*οη*")
    # queries = ("*αι",)
    for query in queries:
        run_query(query)


if __name__ == "__main__":
    main()
