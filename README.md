# **Invaders**

Desenvolvido no curso: **Ultimate Rust Crash Course**
<br>Professor: **Nathan Stocks**

## **Descrição**
**Invaders** é um jogo 2D minimalista jogado diretamente no terminal. Controle uma nave espacial para derrotar ondas de inimigos antes que eles alcancem sua posição. Teste seus reflexos e habilidades estratégicas enquanto enfrenta o desafio de sobreviver em uma batalha intensa no espaço!

---

## **Como Funciona o Jogo**
- **Objetivo**: Eliminar todos os inimigos antes que eles cheguem até você.
- **Derrota**: O jogador perde caso os inimigos alcancem a linha da nave.
- **Vitória**: O jogador vence ao eliminar todos os inimigos da tela.

---

## **Controles**
- **Mover para a esquerda**: `←`  
- **Mover para a direita**: `→`  
- **Atirar**: `Espaço`

---

## **Mecânicas do Jogo**
1. **Inimigos**:
   - Os inimigos aparecem no topo da tela, movendo-se da direita para a esquerda.
   - Quando chegam ao fim da linha, eles descem em direção à nave do jogador.

2. **Jogador**:
   - O jogador controla uma nave que pode se mover lateralmente e atirar para destruir inimigos.

3. **Colisões**:
   - Um inimigo é destruído quando atingido pelo tiro da nave.
   - O jogador perde se algum inimigo alcançar a linha onde a nave está posicionada.

---

## **Instalação**
1. Clone o repositório:
   ```bash
   git clone https://github.com/deusielbrizante/invaders-rust.git
   cd invaders