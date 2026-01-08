\# GDD — Hole.io (versão moderna, procedural, sem assets)



\## 1) Visão Geral



\*\*Título:\*\* Hole.io (City Swallow)

\*\*Gênero:\*\* Hyper-casual / Arcade / “battle royale” leve

\*\*Plataforma alvo:\*\* Desktop (Windows/macOS/Linux) + fácil portar para mobile depois

\*\*Restrições:\*\*



\* \*\*Sem imagens externas\*\* (sprites/texturas)

\* \*\*Sem áudio externo\*\* (músicas/efeitos)

\* Tudo gerado por código: \*\*UI, shapes, cores, “materiais”, efeitos visuais\*\*



\*\*Pitch (1 linha):\*\*

Você controla um buraco que engole objetos da cidade para crescer; em 2 minutos, vire o maior (Classic) ou seja o último sobrevivente (Battle). (\[Wikipedia]\[1])



---



\## 2) Pilares do Jogo



1\. \*\*Satisfação visual de “engolir”\*\* (animações, partículas, “colapso” do objeto).

2\. \*\*Leitura clara e moderna\*\* (UI limpa, cores bem escolhidas, zoom suave).

3\. \*\*Progresso rápido\*\* (crescimento constante; rounds curtos ~2 min). (\[Wikipedia]\[1])

4\. \*\*Competição simples\*\* (IA convincente, leaderboard, risco/recompensa).



---



\## 3) Referência de Gameplay (o que o jogador faz)



\* Anda pela cidade engolindo objetos menores.

\* Ao crescer, começa a engolir objetos maiores e eventualmente prédios.

\* Buracos maiores podem engolir buracos menores (eliminação + respawn em alguns modos). (\[Wikipedia]\[1])



---



\## 4) Modos de Jogo



\### 4.1 Classic (Timed Growth)



\*\*Objetivo:\*\* ser o maior buraco quando o tempo acabar (2:00). (\[Wikipedia]\[1])

\*\*Regras:\*\*



\* Mapa cheio de objetos de vários tamanhos.

\* “Players” adversários podem ser \*\*NPCs\*\* (bots). (\[Wikipedia]\[1])

\* Colisão buraco x buraco:



&nbsp; \* Se `R\_inimigo > R\_jogador \* (1 + margem)` → jogador é engolido.

\* Respawn: 3s (com invencibilidade 1s) para manter fluxo.



\### 4.2 Battle (Last Hole Standing)



\*\*Objetivo:\*\* eliminar todos os outros buracos. (\[Wikipedia]\[1])

\*\*Regras:\*\*



\* Mesmo sistema de engolir cidade, mas o foco é “caça”.

\* Respawn: \*\*desligado\*\* (ou limitado a 1 vida, se quiser mais acessível).

\* Opcional “safe shrink”: área segura encolhe levemente para forçar encontros (deixa o modo mais battle royale).



\### 4.3 Solo (100% City Challenge)



\*\*Objetivo:\*\* engolir o máximo possível (ideal 100%) em 2:00. (\[Wikipedia]\[1])

\*\*Regras:\*\*



\* Sem inimigos.

\* UI mostra “% da cidade consumida” + meta de medalhas.



---



\## 5) Controles (desktop e mobile-friendly)



\*\*Desktop\*\*



\* Movimento: WASD / setas

\* Dash curto: Shift (cooldown)

\* Pause: Esc



\*\*Mobile (futuro)\*\*



\* Joystick virtual (procedural, desenhado por shapes)

\* Botão dash



---



\## 6) Câmera e “sensação” (muito importante)



\* Câmera top-down com leve perspectiva fake.

\* \*\*Follow suave\*\* (lerp) para remover tremedeira.

\* \*\*Zoom dinâmico\*\*:



&nbsp; \* Quanto maior o buraco, mais zoom-out.

&nbsp; \* Zoom não pode “pular”: usar interpolação com mola (spring) ou smoothstep.

\* \*\*Camera shake micro\*\* apenas em engolidas grandes (bem sutil).



---



\## 7) Sistema de Crescimento e Regras de Engolir



\### 7.1 Definições principais



\* Buraco tem \*\*raio\*\* `R`.

\* Cada objeto tem:



&nbsp; \* “tamanho” `S` (ex.: raio/circunferência equivalente)

&nbsp; \* “massa” `M` (pode ser proporcional a `S²`)



\### 7.2 Condição de captura (“cabe no buraco”)



Um objeto pode começar a cair se:



\* `S <= R \* k\_fit` (ex.: `k\_fit = 0.92`)

\* E o centro do objeto entrou na \*\*zona de captura\*\*:



&nbsp; \* distância(obj, buraco) <= `R \* k\_capture` (ex.: `k\_capture = 1.05`)



\### 7.3 Processo de “cair” (animação procedural)



Quando captura começa:



1\. Objeto entra em estado \*\*Falling\*\* (não colide mais com outros).

2\. Escala visual reduz (0.8 → 0.0), rotação aumenta, opacidade diminui.

3\. Objeto é puxado para o centro do buraco (ease-in).

4\. Ao concluir, remove do mundo e soma “valor” ao buraco.



\### 7.4 Ganho de tamanho (progressão agradável)



Use curva não-linear para manter crescimento constante:



\* `area = πR²`

\* `area += value(object) \* growth\_multiplier`

\* `R = sqrt(area / π)`



\*\*value(object):\*\* proporcional a `S²` (objetos grandes valem muito).



\### 7.5 Objetos grandes demais “bloqueiam”



Se `S > R \* k\_fit`:



\* O buraco \*\*não engole\*\*.

\* O objeto pode “parar na borda” (colisão), criando o comportamento de “bloqueio” que o jogador precisa contornar. (\[Wikipedia]\[1])



---



\## 8) Mundo: Cidade procedural (sem assets)



\### 8.1 Geração do mapa (rápida e rejogável)



\* Terreno base: retângulo grande com gradiente sutil.

\* Ruas:



&nbsp; \* grade com variação (avenidas mais largas)

&nbsp; \* faixas desenhadas com linhas/retângulos

\* Quarteirões:



&nbsp; \* prédios como retângulos com alturas fake (sombras)

&nbsp; \* parques como áreas verdes (retângulos arredondados)

\* Props:



&nbsp; \* postes, árvores, carros, pessoas como \*\*shapes\*\* simples (capsules/circles/rects)



\### 8.2 “3D fake” (visual premium sem textura)



Tudo é 2D, mas com:



\* \*\*Sombra\*\* (shape escuro deslocado)

\* \*\*Highlight\*\* (borda clara no topo/esquerda)

\* \*\*Camada\*\* (z-index): objetos sobre rua, UI acima de tudo



---



\## 9) Buracos (player e inimigos)



\### 9.1 Visual do buraco (procedural)



\* Corpo: círculo escuro com \*\*gradiente radial\*\* (centro mais escuro)

\* Borda: anel com cor do jogador + brilho sutil

\* “Profundidade”: segunda camada interna menor (shadow ring)



\### 9.2 Skins sem imagens



Cosméticos 100% por código:



\* Padrões na borda (listras, pontos, checker, wave)

\* Cores neon / pastel / monocromático

\* Partículas (pequenos círculos) ao engolir itens grandes

\* “Trail” no chão (bem sutil) ao usar dash



---



\## 10) IA (bots) — “parece multiplayer”



Como é comum o jogo ser percebido como multiplayer, mas com NPCs, a IA precisa ser “crível” (não perfeita). (\[Wikipedia]\[1])



\### 10.1 Objetivos da IA (por prioridade)



1\. \*\*Se estiver pequeno\*\*: farmar objetos próximos e seguros.

2\. \*\*Se estiver médio\*\*: otimizar rota por densidade de itens.

3\. \*\*Se estiver grande\*\*: caçar buracos menores.

4\. \*\*Se ameaçado\*\*: fugir (vetor oposto ao maior inimigo visível).



\### 10.2 Steering simples (sem pathfinding pesado)



\* “Seek” para alvo + “avoid” paredes/objetos grandes

\* “Threat field”: buracos maiores criam repulsão

\* Um pouco de aleatoriedade para parecer humano



---



\## 11) UI/UX Moderna (sem assets)



\### 11.1 HUD (Playing)



\* Timer grande (2:00)

\* Leaderboard top 5 (nome + tamanho)

\* Seu rank e tamanho

\* Barra de progresso (próximo “tier” de tamanho — opcional)

\* Botão pause (mobile) / “Esc”



\### 11.2 Telas



\*\*Menu\*\*



\* Play (Classic / Battle / Solo)

\* Settings

\* Cosmetics (procedural)

\* Quit



\*\*Pause\*\*



\* Overlay translúcido + card central

\* Resume / Restart / Exit



\*\*Game Over / Results\*\*



\* Rank final

\* “Best size”

\* Botões: Play again / Change mode



\### 11.3 Animações de UI



\* Cards entram com slide + fade

\* Contadores (score/size) com tween suave

\* Leaderboard “pop” quando muda posição



---



\## 12) Efeitos visuais (o que substitui som)



Sem áudio, então os \*\*VFX\*\* precisam carregar a sensação:



\* Partículas ao engolir (10–50 círculos pequenos, lifetime 0.4s)

\* “Ripple” na borda do buraco (anel expandindo com alpha)

\* Screen shake leve em engolidas grandes

\* Flash rápido na linha do leaderboard quando você sobe de posição

\* “Respawn bubble” (círculo protetor pulsando)



---



\## 13) Progressão e Retenção (sem depender de ads no protótipo)



\* XP por partida (tempo vivo + objetos + eliminações)

\* Unlocks:



&nbsp; \* paletas

&nbsp; \* padrões de borda

&nbsp; \* trails

&nbsp; \* efeitos de engolir (cor/forma)



Tudo procedural → zero assets.



---



\## 14) Requisitos Funcionais (Checklist)



\* Mundo procedural com centenas/milhares de objetos.

\* Movimento suave + zoom dinâmico.

\* Captura de objetos com animação de queda.

\* Objetos grandes bloqueiam quando não cabem. (\[Wikipedia]\[1])

\* Buraco cresce com curva agradável (sem ficar lento demais).

\* Classic: timer 2:00 + vencedor por tamanho. (\[Wikipedia]\[1])

\* Battle: eliminações; last standing.

\* Solo: % consumido.

\* Bots com comportamento crível. (\[Wikipedia]\[1])

\* UI moderna: menu, pause, results.



---



\## 15) Requisitos Não-Funcionais (Performance)



\* 60 FPS alvo.

\* Spatial partition (grid / quadtree) para:



&nbsp; \* detectar objetos próximos do buraco

&nbsp; \* evitar checar colisão com “o mundo inteiro”

\* Object pooling (reusar structs/entidades) para partículas e props.



---



\## 16) Stack recomendada em Rust (para esse estilo)



\### Recomendada (equilíbrio “bonito + simples”)



\* \*\*macroquad\*\*: shapes + texto + animação fácil (ótimo pra hyper-casual)

\* ou \*\*Bevy\*\*: se você quer crescer para algo maior (ECS, plugins, pós-efeitos)



\*\*Sem assets\*\* fica perfeito em macroquad: retângulos, círculos, linhas, gradientes fake, UI desenhada.



---



\## 17) Estrutura de Projeto sugerida (alto nível)



```

holeio\_modern/

├── Cargo.toml

└── src/

&nbsp;   ├── main.rs

&nbsp;   ├── app/

&nbsp;   │   ├── state.rs        // Menu/Playing/Pause/Results

&nbsp;   │   └── settings.rs     // toggles, acessibilidade, tema

&nbsp;   ├── world/

&nbsp;   │   ├── gen.rs          // cidade procedural

&nbsp;   │   ├── objects.rs      // props + prédios

&nbsp;   │   └── spatial.rs      // grid/quadtree

&nbsp;   ├── gameplay/

&nbsp;   │   ├── hole.rs         // buraco, crescimento, dash

&nbsp;   │   ├── swallow.rs      // lógica de captura + animação

&nbsp;   │   ├── modes.rs        // classic/battle/solo

&nbsp;   │   ├── bots.rs         // IA

&nbsp;   │   └── scoring.rs      // rank, leaderboard, xp

&nbsp;   ├── render/

&nbsp;   │   ├── theme.rs        // paletas, estilos, tamanhos

&nbsp;   │   ├── draw\_world.rs

&nbsp;   │   ├── draw\_holes.rs

&nbsp;   │   ├── draw\_ui.rs

&nbsp;   │   └── vfx.rs          // partículas, ripple, trails

&nbsp;   └── time/

&nbsp;       └── clock.rs

