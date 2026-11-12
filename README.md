# Open DolbyLake Software (v0.0.1)

[![Release](https://img.shields.io/github/v/release/gabrielpgava/open-lake-controller-5.8?include_prereleases&style=flat-square)](https://github.com/gabrielpgava/open-lake-controller-5.8/releases)
[![Version](https://img.shields.io/badge/version-0.0.1--alpha-orange.svg?style=flat-square)](https://github.com/gabrielpgava/open-lake-controller-5.8)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)

> [!WARNING]
> **AVISO IMPORTANTE / SAFETY NOTICE:**  
> Esta versão (**v0.0.1**) é experimental e **AINDA NÃO FOI TESTADA EM EQUIPAMENTO FÍSICO REAL** (Dolby Lake Processor / LM / PLM).  
> Utilize em bancada, simulação ou ambientes de teste até a validação com hardware oficial.

---

## 🎯 Sobre o Projeto

**Open DolbyLake Software** é uma reconstrução moderna, em código aberto e em modo janela, projetada para manter vivos os lendários processadores **Dolby Lake Processor (DLP)**, Lake Mesa Quad EQ, Lake LM 26 / LM 44 e amplificadores Lab.gruppen PLM Series em sistemas operacionais contemporâneos (Windows 10, Windows 11, macOS e Linux).

O software original sofria de congelamentos gráficos graves, travamentos do computador e telas brancas ao ser minimizado ou restaurado no Windows moderno devido a chamadas obsoletas do DirectDraw exclusivo. Esta edição substitui o motor gráfico por uma arquitetura nativa em Rust (`egui`/`eframe`) com aceleração por hardware moderna, garantindo estabilidade absoluta.

---

## 📦 Funcionalidades Principais

- **Modo Janela Nativo**: Redimensionável e estável ao minimizar, alternar de tela ou restaurar.
- **Botões Quadrados Azuis Modernos**: Interface limpa com borda sutil e atalhos táteis F1–F10.
- **Gerenciamento Completo de Dispositivos**: Inserção de frames virtuais, exclusão de módulos obsoletos e listagem de processadores conectados.
- **Lake LoadLibrary Integrada**: Suporte a presets homologados de caixas de som famosas (Adamson, Turbosound, JBL, L-Acoustics, d&b audiotechnik).
- **Auto-Update Integrado**: Verificador automático de lançamentos no GitHub em Configurações.
- **Instalador Autônomo e Portátil para Windows**: Instalador dedicado (`setup.bat` com criação de atalho na Área de Trabalho e Menu Iniciar).

---

## 💾 Instalação no Windows

1. Baixe o instalador no formato `.zip` na aba de **[Releases](https://github.com/gabrielpgava/open-lake-controller-5.8/releases)**:
   - `open-dolbylake-software-v0.0.1-windows-x86_64-installer.zip`
2. Extraia o conteúdo e execute o instalador:
   - Clique duas vezes em `instalar_open_dolbylake.bat` (ou execute como Administrador para todos os usuários).
3. O atalho **Open DolbyLake Software** será criado na sua Área de Trabalho e no Menu Iniciar.

---

## 📄 Regras de Desenvolvimento

Consulte o arquivo [AGENTS.md](AGENTS.md) para conhecer a diretriz obrigatória de incremento contínuo de versão conforme o software evolui.
