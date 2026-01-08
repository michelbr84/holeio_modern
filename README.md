# 🕳️ Hole.io Clone

Um clone moderno do **Hole.io** desenvolvido em Rust com **macroquad** - 100% gráficos procedurais, sem assets externos.

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![macroquad](https://img.shields.io/badge/macroquad-0.4-blue)
![License](https://img.shields.io/badge/license-MIT-green)

![Hole.io_Clone](src/gameplay/game.png)

## 🎮 Sobre o Jogo

Controle um buraco negro que engole objetos da cidade para crescer. Em 2 minutos, seja o maior buraco (Classic) ou seja o último sobrevivente (Battle)!

### Características

- 🏙️ **Cidade Procedural** - Ruas, prédios, parques, carros, árvores e pessoas gerados por código
- 🕳️ **Mecânica de Buraco** - Movimento suave, crescimento, dash com cooldown
- 🎯 **3 Modos de Jogo**:
  - **Classic**: 2 minutos, maior buraco vence
  - **Battle**: Último buraco sobrevivente vence
  - **Solo**: Consuma 100% da cidade
- 🤖 **5 Bots com IA** - Comportamentos de farming, caça e fuga
- 📊 **Leaderboard ao Vivo** - Rankings em tempo real
- ✨ **Efeitos Visuais** - Partículas, ondulações, screen shake

## 🚀 Como Executar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) 1.70 ou superior

### Compilar e Executar

```bash
# Clonar ou navegar para o diretório
cd holeio_modern

# Executar em modo release (recomendado)
cargo run --release

# Ou apenas compilar
cargo build --release
```

O executável será gerado em `target/release/holeio_modern.exe`

## 🎮 Controles

| Tecla | Ação |
|-------|------|
| `W` / `↑` | Mover para cima |
| `S` / `↓` | Mover para baixo |
| `A` / `←` | Mover para esquerda |
| `D` / `→` | Mover para direita |
| `Shift` | Dash (com cooldown) |
| `Enter` | Selecionar opção |
| `Esc` | Pausar / Voltar |

## 📁 Estrutura do Projeto

```
holeio_modern/
├── Cargo.toml              # Dependências (macroquad, rand)
└── src/
    ├── main.rs             # Game loop e gerenciamento de estados
    ├── app/                # Aplicação
    │   ├── state.rs        # Estados: Menu/Playing/Pause/Results
    │   └── settings.rs     # Configurações do jogo
    ├── world/              # Mundo
    │   ├── gen.rs          # Geração procedural da cidade
    │   ├── objects.rs      # Objetos: prédios, carros, árvores
    │   └── spatial.rs      # Grid espacial para colisões
    ├── gameplay/           # Jogabilidade
    │   ├── hole.rs         # Buraco: movimento, crescimento
    │   ├── swallow.rs      # Lógica de captura e animações
    │   ├── modes.rs        # Modos: Classic/Battle/Solo
    │   ├── bots.rs         # IA dos bots
    │   └── scoring.rs      # Leaderboard e pontuação
    ├── render/             # Renderização
    │   ├── theme.rs        # Paletas de cores e estilos
    │   ├── draw_world.rs   # Renderização da cidade
    │   ├── draw_holes.rs   # Renderização dos buracos
    │   ├── draw_ui.rs      # HUD, menus, overlays
    │   └── vfx.rs          # Partículas, ripples, shake
    └── time/               # Tempo
        └── clock.rs        # Timer do jogo
```

## 🎯 Mecânicas de Jogo

### Sistema de Crescimento
- Área = πR²
- Ao engolir objeto: área += massa × multiplicador
- Novo raio = √(área / π)

### Condição de Captura
- Objeto cabe: `tamanho_objeto ≤ raio_buraco × 0.92`
- Objeto no alcance: `distância ≤ raio_buraco × 1.05`

### Combate Entre Buracos
- Pode engolir outro buraco se for 20% maior
- No modo Battle: sem respawn (eliminação permanente)
- No modo Classic: respawn em 3 segundos com invencibilidade

## 🛠️ Tecnologias

- **[macroquad](https://github.com/not-fl3/macroquad)** - Biblioteca gráfica simples para jogos 2D
- **[rand](https://crates.io/crates/rand)** - Geração procedural de números aleatórios

## 📋 Requisitos Não-Funcionais

- ✅ 60 FPS alvo
- ✅ Spatial grid para detecção eficiente de colisões
- ✅ Suporte a centenas de objetos simultâneos

## 📜 Licença

Este projeto é distribuído sob a licença MIT.

---

Desenvolvido com ❤️ em Rust
"# holeio_modern" 
