# Budget

## Spending tracking via command line

After setting up all of the automatic withdraws for bills, saving, investment,
etc, the only real problem left is making sure that, over time, your checking  account
balance stays about the same (or increases). Involved, detailed budgeting
tools are overkill when everything is automated. This tool allows for an account
balance and a date to be entered from the command line and then differentials
for each interval, or aggregated over time, to be reported.

The idea is for a balance to be entered approximately once a month.

### Example usage

```bash
$ budget add 2016-01-01 1000.00
$ budget add 2016-02-01 1500.00
$ budget add 2016-03-05 1250.00
$ budget add 2016-04-02 1350.00
$ budget show -n 3
2016-02-01 -> 2016-03-05: 1500.00 -> 1250.00 | -250
2016-03-05 -> 2016-04-02: 1250.00 -> 1350.00 | 100
$ budget show -d 2016-03-01
2016-03-05 -> 2016-04-02: 1250.00 -> 1350.00 | 100
$ budget show -n 4 --aggregate
2016-01-01 -> 2016-04-02: 1000.00 -> 1350.00 | 350
```

Use `--help` for complete details.

### File setup

Use the `-f` or `--file` switch to point at a particular file. All data is read
and written as plain text. Multiple files can be used to track multiple
accounts. The default file if none is provided is `~/.budget`.

### Two modes

`add` adds an entry to the given file; it takes positional arguments for `date`
and `amount`. The formatting of the date is required to be `yyyy-mm-dd`; this is
enforced at write time to make reads easy.

`show` shows progress over time. Either `-n`, the number of rows, or `-d` the
date lower-bound is required. The format of the date for `-d` is the same as for
adding an entry. An optional `-a` or `--aggregate` flag shows a single step for
the entire time period; the default shows a diff for each line in the record.

## License
### MIT

```
Copyright (c) 2015 Stuart Terrett

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```
