# br-validators feature binary sizes

> Measured with `cargo run -p br-data-bot -- measure-features` — release binary per feature, gzip level default.
> Sizes **include embedded JSON** for reference-data features.

| Feature | Raw (release) | Gzip | Notes |
|---------|---------------|------|-------|
| `cep-faixas` | 3132.8 KB | 415.8 KB | includes embed |
| `fiscal` | 1588.4 KB | 414.4 KB | fiscal domain group |
| `ncm` | 1560.2 KB | 408.7 KB | includes embed |
| `tse-municipios` | 1182.0 KB | 344.5 KB | includes embed |
| `full` | 886.4 KB | 344.5 KB | all features including remote and generate |
| `anp-combustiveis` | 1608.1 KB | 341.4 KB | includes embed |
| `portos` | 930.2 KB | 312.2 KB | includes embed |
| `cest` | 878.1 KB | 307.6 KB | includes embed |
| `cbo` | 854.6 KB | 305.3 KB | includes embed |
| `all-data` | 775.5 KB | 303.7 KB | all embedded datasets |
| `ptax` | 875.8 KB | 303.5 KB | includes embed |
| `telefone-ddd` | 1120.1 KB | 300.1 KB | includes embed |
| `cnaes` | 771.4 KB | 293.6 KB | includes embed |
| `iss-municipal` | 778.7 KB | 286.9 KB | includes embed |
| `finance` | 720.5 KB | 286.7 KB | banking and monetary group |
| `bancos` | 718.5 KB | 286.1 KB | includes embed |
| `logistics` | 702.0 KB | 280.7 KB | ports, airports and fuels group |
| `aeroportos` | 697.7 KB | 280.0 KB | includes embed |
| `cfop` | 722.4 KB | 277.8 KB | includes embed |
| `trade` | 657.9 KB | 277.7 KB | foreign trade domain group |
| `lc116` | 656.0 KB | 277.4 KB | includes embed |
| `moedas` | 653.1 KB | 276.1 KB | includes embed |
| `nbs` | 720.9 KB | 275.2 KB | includes embed |
| `payroll` | 650.6 KB | 275.0 KB | labor and tax brackets group |
| `esocial` | 648.4 KB | 274.8 KB | includes embed |
| `irpf` | 644.1 KB | 273.2 KB | includes embed |
| `inss` | 642.9 KB | 272.6 KB | includes embed |
| `paises-bacen` | 644.9 KB | 272.6 KB | includes embed |
| `incoterms` | 635.7 KB | 271.9 KB | includes embed |
| `generate` | 661.9 KB | 267.1 KB | synthetic fixtures generator |
| `transparencia-snapshots` | 622.9 KB | 266.6 KB | includes embed |
| `simples-nacional` | 628.2 KB | 266.3 KB | includes embed |
| `geo` | 622.6 KB | 266.2 KB | geographic data and holidays |
| `ibpt` | 622.2 KB | 265.6 KB | includes embed |
| `selic` | 619.5 KB | 265.6 KB | includes embed |
| `gov` | 617.5 KB | 264.7 KB | transparency and public procurement |
| `ibge` | 617.0 KB | 264.3 KB | includes embed |
| `pncp-reference` | 616.1 KB | 263.6 KB | includes embed |
| `nfe-cuf` | 609.9 KB | 262.9 KB | includes embed |
| `natureza-juridica` | 613.4 KB | 261.8 KB | includes embed |
| `cnpj-motivos` | 610.7 KB | 261.5 KB | includes embed |
| `cst` | 607.1 KB | 260.8 KB | includes embed |
| `csosn` | 604.9 KB | 260.6 KB | includes embed |
| `remote` | 561.7 KB | 231.3 KB | dynamic release client |
| `feriados` | 458.2 KB | 193.7 KB | includes embed |
| `default` | 451.2 KB | 189.7 KB | validator only (zero dependencies) |
