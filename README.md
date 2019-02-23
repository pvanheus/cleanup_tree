### (special purpose) BEAST tree cleaner

For a particular BEAST run sequence data was accidentially left in trees leading to a massive
expansion of tree file size. This code cleans up and removes these elements (see the code
for a precise description). In testing `data/sample.tre` is input and `data/sample.out` is
output which should be identifical to `data/sample2.out` if the code is working.

### TODO

* Add CircleCI integration.
