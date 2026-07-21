## Índice

- [Introdução](#intro)
- [Historicamente o TCP e suas RFCs](#intro-tcp)
- [Fundamentação](#fundamentals)
- [Referências](#reference)

<a id="intro"></a>

## Introdução

O conteúdo sobre o protocolo TCP é extremamente rico, encontrado com facilidade em diversas páginas e vídeos no youtube. Este é um material de estudo sobre o TCP: o que é, de onde veio e como funciona, partindo das RFCs que o definem. Escrevi enquanto me aprofundava no protocolo, e o organizo aqui tanto como documentação do meu aprendizado quanto para quem queira seguir o mesmo caminho. Cheguei a esse assunto enquanto trabalhava num artigo (amador, não acadêmico) sobre comunicação com CLPs Siemens da linha S7 e o protocolo S7comm, onde o TCP é um pré-requisito que preferi não detalhar lá dentro.

Este material não tem a necessidade de requisitos aprofundados sobre o tema, mas será útil se compreender sobre as camadas do modelo OSI, pacotes e Ethernet/IP. De qualquer forma, com o objetivo de nivelamento vamos passar por esses conteúdos.

<a id="intro-tcp"></a>

## Historicamente o TCP e suas RFCs

Antes de iniciar a parte técnica fundamental ou mesmo antes de pontuar o próprio conteúdo, é importante compreender sobre as RFCs escritas sobre o TCP. 

As Requests For Comments (RFCs) são documentos que descrevem os padrões da internet, métodos, pesquisas e estudos relacionados a sistemas conectados à internet. Como explica o RFC Editor, qualquer pessoa pode participar da criação de RFCs, podendo se juntar a qualquer grupo de trabalho do interesse [\[1\]](#ref-1). Uma vez publicado, uma RFC não pode ser alterada, se houver alterações ou atualizações uma nova RFC deve ser registrada, deixando a antiga obsoleta. Para este material, seguiremos a RFC 9293 "Transmission Control Protocol (TCP)" publicada em 2022 por Wesley Eddy [\[2\]](#ref-2), que obsoletou formalmente a RFC 793 "Transmission Control Protocol" publicada em 1981 por Jon Postel [\[3\]](#ref-3). Esta última é citada por seu valor histórico e por ser a referência de praticamente toda a literatura sobre o TCP até 2022.

A RFC 793 explica que o TCP baseia-se em conceitos descritos por Cerf e Kahn no artigo "A Protocol for Packet Network Intercommunication" [\[4\]](#ref-4). Embora o design proposto ali tenha sido amplamente superado, na época o "TCP" de Cerf e Kahn ainda era um protocolo único, sem a separação entre TCP e IP, que veio só na segunda metade dos anos 1970. Em um futuro, espero escrever sobre Cerf e Kahn, mas por hora só iremos citá-los.

Ainda comentando sobre a visão pela RFC 793, o TCP tinha como objetivo principal fornecer uma comunicação confiável entre serviços, projetado para ser um protocolo de host para host. Ao longo dos anos, erratas e deficiências em segurança, performance foram encontradas, o número de melhorias cresceu ao longo do tempo em diversos documentos. Então, com o objetivo de reunir todas as alterações e esclarecimentos feitos sobre a RFC 793, foi construída a RFC 9293.

<a id="fundamentals"></a>

## Fundamentação

Como diz a RFC 9293, "[TCP provides a reliable, in-order, byte-stream service to applications](https://datatracker.ietf.org/doc/html/rfc9293#section-2.2-1)" (em tradução livre: "o TCP fornece às aplicações um serviço de fluxo de bytes confiável e ordenado"). Isso significa que uma conexão é estabelecida entre dois processos. O protocolo em si é simétrico: nenhum dos lados é privilegiado. Os termos cliente e servidor descrevem apenas quem inicia a conexão (assunto que retomaremos ao tratar do S7comm).

<a id="reference"></a>

## Referências

<a id="ref-1"></a>[1] RFC Editor, "What Is an RFC?" [Online] Disponível em: <https://www.rfc-editor.org/series/rfc/>. Acesso em: 20 Jul. 2026.\
<a id="ref-2"></a>[2] W. Eddy, "Transmission Control Protocol (TCP)" RFC 9293, [Online] Disponível em: <https://datatracker.ietf.org/doc/html/rfc9293>. Acesso em: 20 Jul. 2026.\
<a id="ref-3"></a>[3] J. Postel, "Transmission Control Protocol" RFC 793, [Online] Disponível em: <https://datatracker.ietf.org/doc/html/rfc793>. Acesso em: 20 Jul. 2026.\
<a id="ref-4"></a>[4] Cerf, V., and R. Kahn, "A Protocol for Packet Network Intercommunication", IEEE Transactions on Communications, [Online] Disponível em: <https://www.cs.princeton.edu/courses/archive/fall06/cos561/papers/cerf74.pdf>. Acesso em: 20 Jul. 2026.
