# Budget

## Spending tracking via command line

After setting up all of the automatic withdraws for bills, saving, investment,
etc, the only real problem left is making sure that, over time, your checking  account
balance stays about the same (or increases). Involved, detailed budgeting
tools are overkill when everything is automated. This tool allows for an account
balance and a date to be entered from the command line and then differentials
for each interval, or aggregated over time, to be reported.

The idea is for a balance to be entered approximately once a month

```bash
$ budget add 2016-01-01 1000.00
```

These are recorded to a file.

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
