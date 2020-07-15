# Command–line audience calculator for experiments

## Installation

Download the latest [release](https://github.com/DmitryTsepelev/audience/releases/tag/0.1.0) and put to the `/usr/local/bin` folder

## Usage

Calculate required audience size for expected conversion change from 5% to 20%:

```bash
audience --current 5 --expected 20
audience -c 5 -e 20
```

Calculate required audience sizes for expected conversion changes from 5%, 5% + step, ... to 20%:

```bash
audience --current 5 --expected 20 --step 2
audience -c 5 -e 20 -s 2
```
