

default:
    just --list

create_day DAY:
    cd days && cargo generate --lib --name day{{DAY}} --path ../day-template 