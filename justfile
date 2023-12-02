set dotenv-load := true

default:
    just --list

create_day DAY:
    cd days && cargo generate --lib --name day{{DAY}} --path ../day-template
    mkdir -p days/day{{DAY}}/inputs
    curl "https://adventofcode.com/2023/day/{{DAY}}/input" -H "Cookie: session=$SESSION_TOKEN" >  days/day{{DAY}}/inputs/day{{DAY}}.txt