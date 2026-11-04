# Manutenção das fontes de dados

## Calendário federal

Após a publicação do calendário federal anual pelo MGI, confira os pontos
facultativos da portaria e acrescente em
`crates/br-data-bot/src/fetchers/feriados.rs` as datas específicas do novo ano.
Preserve os registros de anos anteriores e informe o nome e a base legal
publicados para cada data. Atualize o endereço do calendário usado pelo probe.

Execute `cargo run -p br-data-bot -- refresh feriados` e revise o diff gerado
pelo bot. A biblioteca calcula separadamente os feriados nacionais e os pontos
facultativos recorrentes.
