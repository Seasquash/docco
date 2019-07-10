# Docco

A simple parser wrapper.
It scans different source files to find Doc Blocks, groups them all and
writes the result in a README.md file, generating the documentation
automatically.

## Parsing

The comments within the DocBlock will be analysed, the src code is left as
it is.


The comments are extracted from a block of code and separated by single "lines", while the "delimiter" passed in the
config, and leading empty spaces will be removed from the comment itself.


The comments are only captured if the first is a MD header, to avoid
capturing function comments.

## Ordering

If there is an `index` entry within the configuration json, it will be used
to figure out the order in which the comments have to be written to the
documentation, otherwise spits them as they are found while processing.

