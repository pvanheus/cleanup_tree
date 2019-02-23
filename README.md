### (special purpose) BEAST tree cleaner

For a particular BEAST run sequence data was accidentially left in trees leading to a massive
expansion of tree file size. This code cleans up and removes these elements (see the code
for a precise description). In testing `data/sample.tre` is input and `data/sample.out` is
output which should be identifical to `data/sample2.out` if the code is working.

[![CircleCI](https://circleci.com/gh/pvanheus/cleanup_tree.svg?style=svg)](https://circleci.com/gh/pvanheus/cleanup_tree)

### TODO

* Investigate why (0.2.0) a3274276313d24c982bc5e5b1ff7099da5ba596c is faster that 0.3.0 
