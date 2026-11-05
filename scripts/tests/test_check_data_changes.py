"""Cobre a guarda de arquivos sem acessar a rede."""

import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("guard", Path(__file__).resolve().parents[1] / "check-data-changes.py")
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)


class DataChangesTest(unittest.TestCase):
    def test_distingue_dados_e_codigo_antes_e_depois_do_bump(self):
        for path in ["crates/br-validators/data/bancos/bancos.json", "data/refresh-reports/daily/2026-11-05.json", "docs/DATA-FRESHNESS.md"]:
            self.assertTrue(guard.allowed(path))
        for path in ["Cargo.toml", "Cargo.lock", "CHANGELOG.md"]:
            self.assertFalse(guard.allowed(path))
            self.assertTrue(guard.allowed(path, after_bump=True))
        for path in ["crates/br-validators/src/cpf.rs", ".github/workflows/release.yml", "docs/DATA-FRESHNESS.md.bak", "crates/br-validators/data/../../src/cpf.rs", "/data/refresh-reports/a.json"]:
            self.assertFalse(guard.allowed(path, after_bump=True))

    def test_inclui_origem_de_rename_e_nomes_com_espaco_ou_quebra_de_linha(self):
        payload = b"R  data/refresh-reports/nome novo.json\0crates/br-validators/src/cpf.rs\0?? arquivo com\nlinha\0 D data/refresh-reports/antigo.json\0"
        paths = list(guard.changed_paths(payload))
        self.assertEqual(paths, ["data/refresh-reports/nome novo.json", "crates/br-validators/src/cpf.rs", "arquivo com\nlinha", "data/refresh-reports/antigo.json"])
        self.assertFalse(all(guard.allowed(path) for path in paths))

    def test_rejeita_status_incompleto(self):
        for payload in [b"R  data/novo.json\0", b"invalido\0"]:
            with self.assertRaises(ValueError):
                list(guard.changed_paths(payload))


if __name__ == "__main__":
    unittest.main()
