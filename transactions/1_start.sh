#!/bin/bash

export radix=030000000000000000000000000000000000000000000000000004
export op1=$(resim new-account)
export priv_key1=$(echo $op1 | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export address1=$(echo $op1 | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
export op2=$(resim new-account)
export priv_key2=$(echo $op2 | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export address2=$(echo $op2 | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")

export op3=$(resim publish .)
export package=$(echo $op3 | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")

export op4=$(resim call-function $package EventService create "Rust London Ticket" "RLT" 100 100 250)

export component=$(echo $op4 | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
export adminBadge=$(echo $op4 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '2q;d')
export ticket=$(echo $op4 | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p" | sed '3q;d')
