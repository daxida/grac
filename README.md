Port of [greek-accentuation](https://github.com/jtauber/greek-accentuation) to rust / python via PyO3 + maturin.

To install the python package, clone the repo and:
```
pip install py-grac/
```

To test the quick comparison with `greek-accentuation`:
```
pip install greek-accentuation
python3 cmp.py
```

Other testing commands at:
```
cargo test
cargo bench
```

TODO: 
- Wheels, crate, LICENCE
- WIP: Finish modern greek syllabification (with no support for synizesis)
- Explore [this](https://github.com/datio/grhyph) for synizesis