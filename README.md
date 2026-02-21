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

dhandho 0.3.0
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

You bought a Pokemn Trading card in 2012 at a price of 100\$. In 2022 your card is listed on ebay at a price of 350$.
Calculate the compounded anual growth rate.

###### Solution

Let's find out our input variables for this problem. We have the
_final value_ = 350.0\$
_initial value_ = 100.0\$
_investment period_ = 10 years (2022 - 2012);

```bash
dhandho cagr 350 100 10

0.1334616 # 13.34% 
```

The anual compounded growth rate for the investment is **13.34%**.

#### dhandho intrinsic

Intrinsic value is the value of an asset based on the cash flows it generates.

We have a company which has a market cap of 1000\$. The company has a cashflow growth rate of 10%. We will use a discount rate of 10%.

```bash
dhandho intrinsic -c 100 -g 10,0.10 -r 0.10

year | fv         | pv        
------------------------------
0    | 0          | 100       
1    | 110        | 100       
2    | 121        | 100       
3    | 133.1      | 100       
4    | 146.41     | 99.99999  
5    | 161.05101  | 100       
6    | 177.15611  | 100       
7    | 194.87173  | 100       
8    | 214.35892  | 99.99999  
9    | 235.79482  | 99.99999  
10   | 259.3743   | 99.99999  
TV   | 2593.743   | 999.99994 
------------------------------
NPV               | 2000     

```

As we can see for the given requirements we have an intrinsic value of 2000\$. The company is undervalued.

#### dhandho kelly

Kelly criterion is a formula that helps you maximize your gains by providing different assumptions.

Let's say you have a 55% chance of winning 2x your investment and a 45% chance of losing 1x your investment.
The formula will help you calculate the optimal bet size.

```bash
dhandho kelly -a 0.55,2.0 -a 0.45,-1.0 -b 25000

Bankroll: 25000
Kelly: 0.32500002
Amount to wagger: 8125.0005
```

The optimal bet size is 32.5% of your investment.
If you have a bankroll of 25000\$, you should bet 8125\$.




