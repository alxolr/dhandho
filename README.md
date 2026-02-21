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

Let's value Coca-Cola with more sophisticated assumptions (all values in billions):
- Current Free Cash Flow: $9.8B
- Growth Rates (varying by period):
  - Years 1-3: 5% growth (near-term strong growth)
  - Years 4-7: 3% growth (moderate growth phase)
  - Years 8-10: 2% growth (mature/stable growth)
- Discount Rate: 12% (weighted average cost of capital)
- Cash: $15.8B
- Debt: $45.9B
- Shares Outstanding: 4.3B shares

```bash
dhandho intrinsic -f 9.8 -g 3,0.05,0.0 -g 4,0.03,0.0 -g 3,0.02,0.0 -c 15.8 -d 45.9 -s 4.3 -r 0.12

┌─────────────────────────────────┐   ┌────────────────────────────────┐    ┌─────────────────────────────────┐
│ Assumptions      Value          │   │ Year       FV         PV       │    │ Adjustments      Value          │
╞═════════════════════════════════╡   ╞════════════════════════════════╡    ╞═════════════════════════════════╡
│ Initial Value    $9.80          │   │ 0          0.00       9.80     │    │ NPV              $110.23        │
│ (FCF)                           │   │ 1          10.29      9.19     │    │ - Debt           $45.90         │
│ Discount Rate    12.0%          │   │ 2          10.80      8.61     │    │ + Cash           $15.80         │
│ Growth           Year 1: 5.0%   │   │ 3          11.34      8.07     │    │ = Intrinsic      $80.13         │
│ Assumptions      Year 2: 5.0%   │   │ 4          11.69      7.43     │    │                                 │
│                  Year 3: 5.0%   │   │ 5          12.04      6.83     │    │ ÷ Shares         4.3            │
│                  Year 4: 3.0%   │   │ 6          12.40      6.28     │    │ = per Share      $18.63         │
│                  Year 5: 3.0%   │   │ 7          12.77      5.78     │    └─────────────────────────────────┘
│                  Year 6: 3.0%   │   │ 8          13.02      5.26     │    
│                  Year 7: 3.0%   │   │ 9          13.28      4.79     │    
│                  Year 8: 2.0%   │   │ 10         13.55      4.36     │    
│                  Year 9: 2.0%   │   │ TV         135.50     43.63    │    
│                  Year 10: 2.0%  │   │ NPV                   $110.23  │    
│ Cash             $15.80         │   └────────────────────────────────┘    
│ Debt             $45.90         │                                         
│ Shares           4.3            │                                         
│ Outstanding                     │                                         
└─────────────────────────────────┘
```

The intrinsic value per share is **$18.63**. This calculation demonstrates:
1. **Varying growth assumptions**: Higher growth in near-term (5%), tapering to mature growth (2%)
2. **Realistic discount rate**: 12% WACC for a large-cap company
3. Net Present Value (NPV) of future cash flows: $110.23B
4. After adjustments (debt/cash): $80.13B
5. **Intrinsic value per share: $18.63**

Note: All values can be expressed in billions for simplicity (e.g., 9.8, 15.8, 4.3).

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




