set dotenv-load := true

alias ob := open_bench
alias cd := create_day
alias b := bench
default:
    just --list

create_day DAY:
    cd days && cargo generate --lib --name day{{DAY}} --path ../day-template
    mkdir -p days/day{{DAY}}/inputs
    @curl "https://adventofcode.com/2023/day/{{DAY}}/input" -H "Cookie: session=$SESSION_TOKEN" >  days/day{{DAY}}/inputs/day{{DAY}}.txt
download_input DAY:
    @curl "https://adventofcode.com/2023/day/{{DAY}}/input" -H "Cookie: session=$SESSION_TOKEN" >  days/day{{DAY}}/inputs/day{{DAY}}.txt
open_bench:
    firefox target/criterion/report/index.html
bench DAY:
    cd days/day{{DAY}} && cargo bench
