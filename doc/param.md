# Parameter parsing

Parameters are supplied in a couple of input files and parsed early during Sim Setup in CovidSim.cpp.
Methods `GetInputParameter`, `GetInputParameter2`, `GetInputParameter3` and `readString` are used to locate the relevant square-bracketed label and read the following lines.
These methods are worth unpacking, so as to fully understand their process for potential reimplementation.

### GetInputParameter
Delegates to GetInputParameter2, and asserts that a value was found.
Perhaps this could be renamed GetRequiredParameter.

### GetInputParameter2
Uses GetInputParameter3 to search the second supplied FILE (if not null), then the first.

### readString
Utility for GetInputParameter3 which calls fscanf "%s", returning 1 if something was read and 0 on EOF, otherwise panics.

### GetInputParameter3
First attempts to identify a line containing the label supplied, surrounded by square brackets:
- reads a whitespace-terminated string into buffer `match`
- compares this string to the start of the target label (with square brackets)
- if it matches
  - starts tracking current position with ftell in `CurPos`
  - copies the string read from the file into buffer `ReadItemname`
  - tracks whether the last read string terminates in a closing square bracket
  - loops:
    - reads the next whitespace-terminated string
    - copies a space, then the additional chars into buffer `ReadItemName`
    - compares ReadItemName again
    - eventually either there is no more input (no match), the string doesn't match any more (no match) or it ends in a square bracket (match)
    - FindFlag holds 0 if no match, 1 if match
- on no match, work is done (return 0)
- otherwise, enter a further loop
  - if NumItem2 > 1, parse a 2-dimensional array
    - check next read doesn't begin with a square bracket

Reads following data into one of 3 types: double, int or char*. Aliases these types with variable n = 1,2,3.
