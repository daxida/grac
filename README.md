Fast accentuation and syllabification library for modern Greek that (partially) takes synizesis into account.

While written in rust, it also provides python bindings that can be installed via: 
```
pip install "git+https://github.com/daxida/grac.git#subdirectory=py-grac"
```

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

### Etc.

- TODO: Wheels, crate
- Explore [this](https://github.com/datio/grhyph) for synizesis
- Originally based on ideas from [greek-accentuation](https://github.com/jtauber/greek-accentuation), and [modern_greek_accentuation](https://github.com/PicusZeus/modern_greek_accentuation).
- Related projects @ [harper](https://github.com/automattic/harper)
