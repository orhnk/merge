# NOTES

> Additional links and notes for further documentation improvements.

- Project with the same idea but incomplete: https://github.com/timols/yumitude
- Project similar to pacaptr: https://github.com/fossasia/mew

## Two-way Interpretation

Use some config file `toml`, `RON`, `JSON` or smth to define the appropriate command translations instead of defining an entire module in rust and abstract it over a library.
I thought inspiring from project [MEW](https://github.com/fossasia/mew) but MEW's approach is unmaintainable and against DRY principle (Writing every possibility by hand instead of parsing the cmd)