# advent-of-code
Advend of code in RUST

## running 
``` 
cargo r --release -p aoc{YEAR} --bin day{DAY}
cargo r --release -p aoc{YEAR}
```

## Get puzzle input
Get your session id from your browser developer tools
```
AOC_SESSION={SESSION} ./fetch.sh {YEAR} {DAY}
```

## Getting started
Init a new year
```
./init_year.sh {YEAR}
```
Init a new day (creates a new day binary source and downloads the input file)
```
./init_day.sh {YEAR} {DAY}
```
