import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const root = fileURLToPath(new URL('../../', import.meta.url));
const reference = resolve(root, process.argv[2] ?? 'reference/br-validators');
const core = await import(pathToFileURL(resolve(reference, 'packages/br-validators/dist/index.js')));
const vectors = resolve(root, 'crates/br-validators/tests/vectors');
const output = resolve(root, 'crates/br-validators/tests/parity');
const filenames = (await readdir(vectors)).sort();
const border = [
  '', '   ', '.- /', 'abc', '１２３', '1'.repeat(200), '0'.repeat(11), '1'.repeat(14),
  '1_2+3', '1\u00a02', '\ufeff123', '🦀123', '12\n3',
];
const ufs = core.IE_SUPPORTED_UFS;
const generationCount = 500;
const firstSeed = 0x42520000;
const metadata = /^(source|url|note|code|message|issuer|tipo|format|brand|uf|type)$|Url$/i;
const documents = [
  { id: 'cpf', suffix: 'Cpf' },
  { id: 'cnpj', suffix: 'Cnpj', generateOpts: [{ format: 'numeric' }, { format: 'alphanumeric' }] },
  { id: 'cep', suffix: 'Cep' },
  { id: 'pis-pasep', suffix: 'PisPasep' },
  { id: 'cnh', suffix: 'Cnh' },
  { id: 'renavam', suffix: 'Renavam' },
  { id: 'titulo-eleitor', suffix: 'TituloEleitor', generateOpts: ufs.map(uf => ({ uf })) },
  { id: 'processo-judicial', suffix: 'ProcessoJudicial', generate: false },
  { id: 'nfe-chave', suffix: 'NfeChave' },
  { id: 'placa', suffix: 'Placa', generateOpts: [{ format: 'legacy' }, { format: 'mercosul' }] },
  { id: 'telefone', suffix: 'Telefone', generateOpts: [{ format: 'fixo' }, { format: 'celular' }] },
  { id: 'pix', suffix: 'PixKey' },
  { id: 'brcode', suffix: 'BrCode' },
  { id: 'boleto', suffix: 'Boleto' },
  { id: 'boleto-arrecadacao', suffix: 'Arrecadacao' },
  { id: 'cartao-credito', suffix: 'CartaoCredito', generateOpts: core.GENERATABLE_CARD_BRANDS.map(brand => ({ brand })) },
  { id: 'ean', suffix: 'Ean', generate: false },
  { id: 'inscricao-estadual', suffix: 'InscricaoEstadual', ufPrefix: 'ie' },
  { id: 'inscricao-estadual-produtor-rural', suffix: 'IeProdutorRural', stripSuffix: 'IeSpRural', uf: 'SP', ufFirst: true },
  { id: 'rg', suffix: 'Rg', ufPrefix: 'rg', generate: false },
  { id: 'cnis', suffix: 'Nit', generate: 'pis-pasep' },
];

function* inputsFrom(value) {
  if (typeof value === 'string') {
    yield value;
  } else if (value !== null && typeof value === 'object') {
    for (const [key, nested] of Object.entries(value)) {
      if (!metadata.test(key)) yield* inputsFrom(nested);
    }
  }
}

async function readVector(name) {
  return JSON.parse(await readFile(resolve(vectors, name), 'utf8'));
}

async function goldenInputs(document, uf) {
  const prefix = document.ufPrefix ? `${document.ufPrefix}.${uf.toLowerCase()}` : document.id;
  const names = filenames.filter(name => name.startsWith(`${prefix}.`));
  const inputs = [];
  for (const name of names) {
    inputs.push(...inputsFrom(await readVector(name)));
  }
  if (document.ufPrefix === 'ie') {
    const { cases } = await readVector('ie.negative.official.json');
    for (const [name, input] of Object.entries(cases)) {
      if (name.startsWith(uf.toLowerCase())) inputs.push(input);
    }
  }
  return inputs;
}

function alterDigit(input) {
  return input.replace(/[0-9](?=[^0-9]*$)/, digit => String((Number(digit) + 1) % 10));
}

function addCase(cases, input, opts, expected) {
  const key = JSON.stringify([input, opts]);
  cases.set(key, { input, opts, expected });
}

async function save(name, cases) {
  // Um caso por linha facilita comparar e revisar a regeneração.
  const lines = [...cases.values()].map(value => `  ${JSON.stringify(value)}`);
  await writeFile(resolve(output, `${name}.json`), `[\n${lines.join(',\n')}\n]\n`);
}

await mkdir(output, { recursive: true });
const format = new Map();
const strip = new Map();
const detect = new Map();

for (const document of documents) {
  const { id, suffix, uf } = document;
  const validateValue = core[`validate${suffix}`];
  const validate = document.ufFirst
    ? (input, opts) => validateValue(opts.uf, input)
    : validateValue;
  const formatValue = core[`format${suffix}`];
  const stripValue = core[`strip${document.stripSuffix ?? suffix}`];
  const states = document.ufPrefix ? ufs : [uf];
  const cases = new Map();
  for (const state of states) {
    const opts = state === undefined ? {} : { uf: state };
    for (const input of new Set([...await goldenInputs(document, state), ...border])) {
      addCase(cases, input, opts, validate(input, opts));
      if (formatValue) addCase(format, input, { type: id, ...opts }, formatValue(input, opts));
      if (stripValue) addCase(strip, input, { type: id, ...opts }, stripValue(input, opts));
      addCase(detect, input, opts, core.detect(input, opts));
    }
  }

  if (document.generate !== false) {
    const generator = typeof document.generate === 'string' ? document.generate : id;
    const options = document.ufPrefix === 'ie'
      ? ufs.map(uf => ({ uf }))
      : document.generateOpts ?? [{}];
    for (let i = 0; i < generationCount; i++) {
      const generateOpts = { seed: firstSeed + i, ...options[i % options.length] };
      const opts = document.ufPrefix ? { uf: generateOpts.uf } : uf ? { uf } : {};
      const input = core.generate(generator, generateOpts);
      const expected = validate(input, opts);
      if (!expected.ok) {
        throw new Error(`Gerador ${generator} produziu ${input} com opções ${JSON.stringify(generateOpts)}: ${expected.message}`);
      }
      addCase(cases, input, opts, expected);
      const altered = alterDigit(input);
      addCase(cases, altered, opts, validate(altered, opts));
    }
  }

  await save(id, cases);
  console.log(`${id}: ${cases.size}`);
}

const formatted = await readVector('format.official.json');
for (const { type, raw } of formatted.cases) {
  addCase(format, raw, { type }, core.formatDocument(type, raw));
}

await save('format', format);
await save('strip', strip);
await save('detect', detect);
console.log(`format: ${format.size}\nstrip: ${strip.size}\ndetect: ${detect.size}`);
