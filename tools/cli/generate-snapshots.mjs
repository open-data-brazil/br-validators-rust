// Executar sobre uma cópia temporária compilada do commit REFERENCE_COMMIT.
import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
const cli = resolve(process.argv[2] ?? 'reference/br-validators/apps/cli/dist/index.js');
const output = resolve('crates/br-validators-cli/tests/snapshots');
mkdirSync(output, { recursive: true });
for (const [name, args] of [
  ['cpf', ['cpf', 'validate', '123.456.789-09', '--json']],
  ['cnpj', ['cnpj', 'validate', '12ABC34501DE35', '--json']],
  ['bancos', ['bancos', 'lookup', '001', '--json']],
]) {
  writeFileSync(resolve(output, `${name}.json`), execFileSync('node', [cli, ...args]));
}
