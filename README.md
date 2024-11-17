Port of [greek-accentuation](https://github.com/jtauber/greek-accentuation) to rust / python via PyO3 + madurin.

WIP: Finish modern greek syllabification (with no support for synizesis)

To install, clone the repo and:
```
pip install -e py-grac/
```

To test the quick comparison with `greek-accentuation`:
```
pip install greek-accentuation
python3 cmp.py
```