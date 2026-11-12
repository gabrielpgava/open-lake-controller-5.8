# Open Lake Controller v5.8 (Modern Windowed Edition)

Reconstrução moderna e em código aberto do **Lake Controller v5.8**, desenvolvida para manter o software vivo e 100% funcional em sistemas operacionais modernos (Windows 10, Windows 11), com suporte dedicado aos processadores clássicos e modernos, incluindo o **Dolby Lake Processor (DLP)**, Lake Mesa Quad EQ, LM 26, LM 44 e amplificadores Lab.gruppen PLM Series.

---

## 🎯 Objetivo do Projeto

O software original do Lake Controller foi desenvolvido na era do Windows XP e Windows 7 com renderização exclusiva via DirectDraw/DirectX da época. Em versões recentes do Windows, o aplicativo frequentemente trava a interface gráfica, congela ao ser minimizado ou restaurado e gera telas em branco.

A finalidade deste projeto é:
1. **Manter o software vivo**: Garantir a operação contínua e confiável dos consoles e sistemas baseados no **Dolby Lake Processor**, permitindo que engenheiros de áudio e técnicos de som continuem operando esses processadores sem depender de máquinas legadas ou máquinas virtuais instáveis.
2. **Modo Janela Nativo**: Substituir o modo fullscreen exclusivo problemático por uma janela nativa moderna, redimensionável e estável.
3. **Bibliotecas Gráficas Modernas**: Reconstrução com aceleração por hardware moderna (Rust + egui/eframe), eliminando o congelamento de tela ao minimizar ou restaurar.
4. **Fidelidade Visual e Operacional**: Preservação estrita da estética clássica que os operadores já conhecem (molduras amarelas arredondadas, módulos esféricos 3D com brilho, botões gel F1–F10 e medidores de LED segmentados).
5. **Comunicação em Rede UDP 6004**: Suporte a seleção de interface de rede (NIC) dedicada para controle dos processadores na rede de áudio.

---

## 🚀 Tecnologias Utilizadas

- **Linguagem**: [Rust](https://www.rust-lang.org/)
- **Interface e Gráficos**: `egui` / `eframe` (com suporte a rendering via hardware moderno)
- **Protocolo de Rede**: UDP broadcast (porta padrão `6004`) para descoberta e telemetria de processadores Lake / Dolby Lake.

---

## 🛠️ Como Compilar e Executar

### Pré-requisitos
- Rust (versão estável recente com target `x86_64-pc-windows-gnu` ou `x86_64-pc-windows-msvc`)

### Compilando
```bash
cargo build --release
```

### Executando
```bash
cargo run
```

Para capturar um screenshot automatizado para testes:
```bash
cargo run -- --screenshot captura.png
```

---

## 📄 Licença e Propósito Comunitário
Este projeto foi desenvolvido com propósitos de preservação de hardware de áudio profissional e interoperabilidade para manter os equipamentos **Dolby Lake** plenamente operacionais no mercado de áudio profissional.
