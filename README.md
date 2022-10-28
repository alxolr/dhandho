# Dhandho

It's a cli tool build in rust, that helps calculate intrinsic value of an investment.
The cli is inspired by the Mohnish Pabrai book "Dhandho Investor" which I still feel it's one of the best starting investing book.

## Install

```bash
cargo install dhandho
```

## Usage

```bash
dhandho help

dhandho 0.1.0
Alexandru Olaru. <alxolr@gmail.com>

USAGE:
    dhandho <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cagr         Calculated the compounded anual growth rate
    help         Prints this message or the help of the given subcommand(s)
    intrinsic    Computes the intrinsic value of an asset by providing different parameters
    kelly        Maximaze the gains by providing different assumptions. Ex: -a 0.8,21.0
```

### dhandho cagr

> [CAGR](https://www.investopedia.com/terms/c/cagr.asp) Compounded anual growth rate.

###### Problem

> You bought a Pokemn Trading card in 2012 at a price of 100\$. In 2022 your card is listed on ebay at a price of 350$.
Calculate the compounded anual growth rate.

###### Solution

Let's find out our input variables for this problem. We have the **final value** = 350.0\$ our **initial value** = 100.0\$, and the **investment period** of 10 years (2022 - 2012);


```bash
dhandho cagr 350 100 10

0.1334616 # 13.34% 
```

The anual compounded growth rate for the investment is **13.34%**.





#### dhandho intrinsic

#### dhandho kelly



