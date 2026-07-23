## Índice

- [Introdução](#intro)
- [Historicamente o TCP e suas RFCs](#intro-tcp)
- [Fundamentação](#fundamentals)
- [Referências](#reference)
- [Construindo com Rust](#build-with-rust)

<a id="intro"></a>

## Introdução

O conteúdo sobre o protocolo TCP é extremamente rico, encontrado com facilidade em diversas páginas e vídeos no youtube. Este é um material de estudo sobre o TCP: o que é, de onde veio e como funciona e sua aplicação em rust, partindo das RFCs que o definem. Escrevi enquanto me aprofundava no protocolo, e o organizo aqui tanto como documentação do meu aprendizado quanto para quem queira seguir o mesmo caminho. Cheguei a esse assunto enquanto trabalhava num artigo (amador, não acadêmico) sobre comunicação com CLPs Siemens da linha S7 e o protocolo S7comm, onde o TCP é um pré-requisito que preferi não detalhar lá dentro.

Este material não tem a necessidade de requisitos aprofundados sobre o tema, mas será útil se compreender sobre as camadas do modelo OSI, pacotes e Ethernet/IP. De qualquer forma, com o objetivo de nivelamento vamos passar por esses conteúdos.

<a id="intro-tcp"></a>

## Historicamente o TCP e suas RFCs

Antes de iniciar a parte técnica fundamental ou mesmo antes de pontuar o próprio conteúdo, é importante compreender sobre as RFCs escritas sobre o TCP. 

As Requests For Comments (RFCs) são documentos que descrevem os padrões da internet, métodos, pesquisas e estudos relacionados a sistemas conectados à internet. Como explica o RFC Editor, qualquer pessoa pode participar da criação de RFCs, podendo se juntar a qualquer grupo de trabalho do interesse [\[1\]](#ref-1). Uma vez publicado, uma RFC não pode ser alterada, se houver alterações ou atualizações uma nova RFC deve ser registrada, deixando a antiga obsoleta. Para este material, seguiremos a RFC 9293 "Transmission Control Protocol (TCP)" publicada em 2022 por Wesley Eddy [\[2\]](#ref-2), que obsoletou formalmente a RFC 793 "Transmission Control Protocol" publicada em 1981 por Jon Postel [\[3\]](#ref-3). Esta última é citada por seu valor histórico e por ser a referência de praticamente toda a literatura sobre o TCP até 2022.

A RFC 793 explica que o TCP baseia-se em conceitos descritos por Cerf e Kahn em 1974 no artigo "A Protocol for Packet Network Intercommunication" [\[4\]](#ref-4). Embora o design proposto ali tenha sido amplamente superado, na época o "TCP" de Cerf e Kahn ainda era um protocolo único, sem a separação entre TCP e IP, que veio só na segunda metade dos anos 1970. Em um futuro, espero escrever sobre Cerf e Kahn, mas por hora só iremos citá-los.

Ainda comentando sobre a visão pela RFC 793, o TCP tinha como objetivo principal fornecer uma comunicação confiável entre serviços, projetado para ser um protocolo de host para host. Ao longo dos anos, erratas e deficiências em segurança, performance foram encontradas, o número de melhorias cresceu ao longo do tempo em diversos documentos. Então, com o objetivo de reunir todas as alterações e esclarecimentos feitos sobre a RFC 793, foi construída a RFC 9293.

<a id="fundamentals"></a>

## Fundamentação

Como diz a RFC 9293, "[TCP provides a reliable, in-order, byte-stream service to applications](https://datatracker.ietf.org/doc/html/rfc9293#section-2.2-1)" (em tradução livre: "o TCP fornece às aplicações um serviço de fluxo de bytes confiável e ordenado"). Isso significa que uma conexão é estabelecida entre dois processos. O protocolo em si é simétrico: nenhum dos lados é privilegiado. Os termos cliente e servidor descrevem apenas quem inicia a conexão.

O conjunto de protocolos TCP/IP foi desenvolvido antes do modelo OSI, então o TCP/IP não corresponde exatamente as camadas do modelo OSI. O TCP/IP foi originalmente definido em quatro camadas <strong>(host-rede, internet, transporte e aplicação)</strong>, enquanto o modelo OSI tem sete <strong>(física, enlace de dados, rede, transporte, sessão, apresentação e aplicação)</strong>. Forouzan [\[5\]](#ref-5) propõe uma leitura em cinco camadas, desmembrando host-rede em física e enlace para aproximar a comparação do OSI.

| Camada TCP/IP (Forouzan) | Camada(s) OSI correspondente(s) |
|---|---|
| Física | Física |
| Enlace | Enlace de dados |
| Rede | Rede |
| Transporte | Transporte (+ parte da sessão) |
| Aplicação | Sessão (parte) + Apresentação + Aplicação |

<a id="build-with-rust"></a>

## Construindo com Rust

Após um longo tempo revendo a teoria sobre TCP, compreendemos seu funcionamento. Então para começarmos aplicando isso, vamos desenvolver uma conexão TCP com rust. O objetivo de escolhermos Rust se da pelo fato de eu estar ativamente estudando ele, é uma linguagem que admiro e particularmente gosto (em um momento pretendo também escrever sobre isso). 

Vou partir da ideia que você já tem um script inicial sendo um main.rs (`cargo new your-project-name`). Para você ter um contexto maior, recomendo a leitura do módulo <strong>net</strong> do rust [\[6\]](#ref-6), tem bibliotecas mais recomendadas para construir uma comunicação TCP/IP mas antes de aprofundarmos em alguma abstração quero construir um caminho sólido. No módulo <strong>net</strong> temos duas funcionalidades importantes para a comunicação via TCP: `TcpListener` [\[7\]](#ref-7) e `TcpStream` [\[8\]](#ref-8).

```rust-lang
use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:34254")?;

  stream.write(&[1])?;
  stream.read(&mut [0; 128])?;

  Ok(())
}
```

Ok, isso é o suficiente para conectarmos a uma porta `:34254` em uma conexão TCP. Mas, isso nos entrega um erro:

```
Error: Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }
```

*111 é o errno do <strong>LINUX</strong> para `ECONNREFUSED` em outro sistema operacional o número muda, o erro não.*

Vale apena pararmos um pouco aqui para compreender. Estamos aqui fazendo uma chamada `connect()`, e a pilha TCP monta e envia um segmento SYN para a `127.0.0.1:34254`. Do outro lado o kernel procura um socket em estado <strong>LISTEN</strong> naquela porta `:34254` e não encontra nada (porque ainda não criamos nada que esta <strong>VINCULADA</strong> aquela porta/serviço). A RFC 9293 explica que nesse caso, se a conexão não existe (estado <strong>CLOSED</strong>), um <strong>RESET</strong> é enviado em resposta a qualquer segmento que chegue, exceto outro <strong>RESET</strong>. Veja mais sobre na seção 3.5.2 [\[2\]](#ref-2) da RFC 9293. 

Ou seja: a máquina responde com um <strong>RST</strong>. O `connect()` recebe esse reset e desiste da conexão e então retora com o <strong>erro 111</strong>.

```
No.  Time         Source     Destination  Proto  Len  Info
1    0.000000000  127.0.0.1  127.0.0.1    TCP    74   33208 → 34254 [SYN] Seq=0 Win=43690 Len=0 MSS=65495 SACK_PERM TSval=762580800 TSecr=0 WS=512
2    0.000018228  127.0.0.1  127.0.0.1    TCP    54   34254 → 33208 [RST, ACK] Seq=1 Ack=1 Win=0 Len=0
```

*Listagem 1 — Captura na interface `lo`, filtro `tcp.port == 34254`.*

<a id="reference"></a>

## Referências

<a id="ref-1"></a>[1] RFC Editor, "What Is an RFC?," [Online]. Disponível em: <https://www.rfc-editor.org/series/rfc/>. Acesso em: 20 jul. 2026.\
<a id="ref-2"></a>[2] W. Eddy, "Transmission Control Protocol (TCP)," RFC 9293, 2022. [Online]. Disponível em: <https://datatracker.ietf.org/doc/html/rfc9293>. Acesso em: 20 jul. 2026.\
<a id="ref-3"></a>[3] J. Postel, "Transmission Control Protocol," RFC 793, 1981. [Online]. Disponível em: <https://datatracker.ietf.org/doc/html/rfc793>. Acesso em: 20 jul. 2026.\
<a id="ref-4"></a>[4] V. Cerf e R. Kahn, "A Protocol for Packet Network Intercommunication," *IEEE Transactions on Communications*, vol. COM-22, no. 5, pp. 637–648, maio 1974. [Online]. Disponível em: <https://www.cs.princeton.edu/courses/archive/fall06/cos561/papers/cerf74.pdf>. Acesso em: 20 jul. 2026.\
<a id="ref-5"></a>[5] B. A. Forouzan, *Comunicação de Dados e Redes de Computadores*, 4. ed. Porto Alegre: AMGH, 2008, seç. 2.4, p. 74. \
<a id="ref-6"></a>[6] "Module net," The Rust Standard Library. [Online]. Disponível em: <https://doc.rust-lang.org/std/net/index.html>. Acesso em: 23 jul. 2026.\
<a id="ref-7"></a>[7] "Struct TcpListener," The Rust Standard Library. [Online]. Disponível em: <https://doc.rust-lang.org/std/net/struct.TcpListener.html>. Acesso em: 23 jul. 2026.\
<a id="ref-8"></a>[8] "Struct TcpStream," The Rust Standard Library. [Online]. Disponível em: <https://doc.rust-lang.org/std/net/struct.TcpStream.html>. Acesso em: 23 jul. 2026.\
