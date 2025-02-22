Fast accentuation and syllabification library for modern Greek that (partially) takes synizesis into account.

Consider using this if speed (over python implementations) and accuracy (over generic hyphenation libraries) are relevant to your task.

It also provides some (unfinished) python bindings that can manually be installed by cloning the repo and running: `pip install py-grac/`

Based originally on ideas from [greek-accentuation](https://github.com/jtauber/greek-accentuation), and [modern_greek_accentuation](https://github.com/PicusZeus/modern_greek_accentuation).

### Testing

To test the quick comparison with [greek-accentuation](https://github.com/jtauber/greek-accentuation):
```
pip install greek-accentuation
python3 cmp.py
```

Other testing commands at:
```
cargo test
cargo bench
```

### TODO

- Wheels, crate, LICENCE
- WIP: Finish modern greek syllabification (with no support for synizesis)
- Explore [this](https://github.com/datio/grhyph) for synizesis
