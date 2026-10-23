# Integração Headless com o Sunshine 📡🎮

Este documento descreve como o **Kame House Launcher** estende o [Sunshine](https://github.com/LizardByte/Sunshine) como um serviço de streaming headless de alto desempenho, substituindo a interface web convencional por uma experiência integrada e nativa no launcher.

---

## 1. Filosofia de Integração: Extensão sem Apropriação

O Sunshine é a referência mundial em captura de tela de ultra-baixa latência e codificação de vídeo em tempo real (NVENC, AMF, QuickSync, VAAPI, software). O Kame House Launcher adota os seguintes princípios:
1. **Zero Modificação no Motor C++:** Toda a infraestrutura de codificação de áudio/vídeo, RTSP e emulação de entrada permanece 100% sob responsabilidade do código upstream do Sunshine.
2. **Submódulo Git e Rastreamento Upstream:** O Sunshine é mantido como submódulo oficial apontando para o repositório oficial da organização LizardByte, possibilitando atualizações contínuas de versão.
3. **Substituição da Camada de Apresentação:** O frontend web legado do Sunshine (que exigia abrir navegadores e aceitar avisos de certificados SSL) é substituído pela nossa interface integrada orientada a controle.

---

## 2. Comunicação via API REST HTTPS Local

O Sunshine expõe internamente uma API REST na porta `47990` (`confighttp.cpp`). A crate `launcher-sunshine` interage diretamente com esses endpoints:

### 2.1 Principais Endpoints Consumidos

| Endpoint | Método | Descrição | Uso no Launcher |
| :--- | :---: | :--- | :--- |
| `/api/csrf-token` | `GET` | Recupera o token CSRF da sessão | Autenticação para requisições seguras |
| `/api/pin` | `POST` | Envia o PIN de 4 dígitos digitado pelo usuário | Pareamento instantâneo do Moonlight na tela |
| `/api/config` | `GET`/`POST` | Lê e aplica alterações de configuração | Ajustes de encoders, taxa de quadros e portas |
| `/api/apps` | `GET`/`POST` | Gerencia os aplicativos transmitidos | Sincronização do catálogo de jogos |
| `/api/apps/close` | `POST` | Encerra a transmissão do aplicativo atual | Interrupção segura de sessões ativas |
| `/api/restart` | `POST` | Reinicia o daemon de streaming | Aplicação a quente de mudanças de hardware |
| `/api/logs` | `GET` | Recupera as últimas mensagens de log | Monitoramento e diagnóstico de conexão |

---

## 3. Fluxo de Pareamento de Clientes Moonlight

1. O usuário abre o aplicativo **Moonlight** em uma TV da sala, Steam Deck, portátil ou smartphone e seleciona "Add Host".
2. O Moonlight gera um **código PIN de 4 dígitos** na tela remota.
3. No mesmo instante, o Kame House Launcher detecta o pedido ou exibe o botão **"Parear Dispositivo Moonlight"** na aba Sunshine.
4. O usuário digita o PIN de 4 dígitos diretamente no popup do launcher usando o controle ou o teclado virtual.
5. O `launcher-sunshine` envia o payload:
   ```json
   {
     "pin": "1234",
     "name": "Kame House Launcher"
   }
   ```
6. O Sunshine valida o handshake criptográfico e autoriza o cliente Moonlight imediatamente, sem abrir qualquer navegador externo.

---

## 4. Sincronização de Jogos para o `apps.json`

O `launcher-sunshine` mantém o arquivo `apps.json` sincronizado com a biblioteca de jogos do launcher:

```json
{
  "apps": [
    {
      "name": "Kame House Launcher (Console)",
      "cmd": "\"C:\\Program Files\\KameHouse\\kame-house-launcher.exe\" --fullscreen",
      "index": -1,
      "auto-detach": "true",
      "wait-all": "true",
      "exit-timeout": "5"
    },
    {
      "name": "Super Mario 64",
      "cmd": "\"C:\\Games\\sm64.z64\"",
      "index": -1,
      "image-path": "C:\\Program Files\\KameHouse\\art\\sm64_cover.png",
      "auto-detach": "true",
      "wait-all": "true",
      "exit-timeout": "5"
    }
  ]
}
```

Dessa forma, ao abrir o Moonlight na TV da sala, o usuário pode escolher:
- Iniciar diretamente a experiência completa do **Kame House Launcher em tela cheia**; OU
- Lançar títulos específicos individualmente a partir do menu do Moonlight.

---

## 5. Rastreamento Upstream e Atualizações

Para manter o Sunshine sempre atualizado com as últimas melhorias de infraestrutura e correções de segurança lançadas pelo LizardByte:

```bash
# Atualizar o submódulo do Sunshine para a versão upstream mais recente
git submodule update --remote --merge Sunshine

# Compilar e validar a integração
cargo test -p launcher-sunshine
```

### Créditos e Licença
Conforme a licença **GPL-3.0**, todos os créditos do projeto Sunshine pertencem à organização **LizardByte e aos Sunshine Contributors**. O Kame House Launcher não reivindica direitos autorais sobre a infraestrutura do Sunshine, apenas a estende como plano de controle nativo.

