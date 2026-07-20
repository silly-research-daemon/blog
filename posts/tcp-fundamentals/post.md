## Índice

- [Introdução](#intro)
- [Historicamente o TCP e suas RFCs](#intro-tcp)
- [Referências](#reference)

<a id="intro"></a>

## Introdução

O conteúdo sobre o protocolo TCP é extremamente rico, encontrado com facilidade em diversas páginas web e vídeos no youtube, então o material proposto aqui vem com o objetivo de apoiar todo esse conteúdo fornecido na internet e também servirá para minha documentação própria sobre o meu aprendizado e na produção de um artigo (amador, não acadêmico) sobre "Comunicação com CLPs Siemens da linha S7: fundamentos do protocolo S7comm e integração em C/C++" que venho trabalhando. Neste artigo que citei, tenho como objetivo a documentação de uma etapa técnica para a comunicação com CLPs Siemens, sem dependência de projetos avulsos, acredito quando se comenta sobre comunicação de CLPs é necessário a profundidade técnica sobre o protocolo industrial construído nele e seus fundamentos. Como no artigo a linguagem e objetivo são outros, construí esse blog para me aprofundar sobre temas que acredito ser importante detalhar (como TCP). Como estou escrevendo isso para fundamentar meu artigo, provável que fique enviesado.

Este material não tem a necessidade de requisitos aprofundados sobre o tema, mas será útil se compreender sobre as camadas do modelo OSI, pacotes e Ethernet/IP. De qualquer forma, com o objetivo de nivelamento vamos passar por esses conteúdos. 

<a id="intro-tcp"></a>

## Historicamente o TCP e suas RFCs

Antes de iniciar a parte técnica fundamental ou mesmo antes de pontuar o próprio conteúdo, é importante compreender sobre as RFCs escritas sobre o TCP. 

As Requests For Comments (RFCs) são documentos que descrevem os padrões da internet, métodos, pesquisas e estudos relacionado a sistemas conectados à internet. Como explica o RFC Editor, qualquer pessoa pode participar da criação de RFCs, podendo se juntar a qualquer grupo de trabalho do interesse [\[1\]](#ref-1). Uma vez publicado, uma RFCs não pode ser alterada, se houver alterações ou atualizações uma nova RFC deve ser registrada, deixando a antiga obsoleta. Para nosso estudos, vamos nos basear em duas RFCs, a RFC 793 "Transmission Control Protocol" publicada em 1981 por Jon Postel [\[2\]](#ref-2) e a RFC 9293 "Transmission Control Protocol (TCP)" publicada em 2022 por Wesley Eddy [\[3\]](#ref-3).

A RFC 793 explica que o TCP baseia-se em conceitos descritos por Cerf e Kahn no artigo "A Protocol for Packet Network Intercommunication" [\[4\]](#ref-4). Embora o design proposto ali tenha sido amplamente superado, na época o "TCP" de Cerf e Kahn ainda era um protocolo único, sem a separação entre TCP e IP, que veio só na segunda metade dos anos 1970.

<a id="reference"></a>

## Referências

<a id="ref-1"></a>[1] RFC Editor, "What Is an RFC?" [Online] Disponível em: <https://www.rfc-editor.org/series/rfc/>. Acesso em: 20 Jul. 2026.\
<a id="ref-2"></a>[2] J. Postel, "Transmission Control Protocol" RFC 793, [Online] Disponível em: <https://datatracker.ietf.org/doc/html/rfc793#page-iii>. Acesso em: 20 Jul. 2026.\
<a id="ref-3"></a>[3] W. Eddy, "Transmission Control Protocol (TCP)" RFC 9293, [Online] Disponível em: <https://datatracker.ietf.org/doc/html/rfc9293>. Acesso em: 20 Jul. 2026.\
<a id="ref-4"></a>[4] Cerf, V., and R. Kahn, "A Protocol for Packet Network Intercommunication", IEEE Transactions on Communications, [Online] Disponível em: <https://www.cs.princeton.edu/courses/archive/fall06/cos561/papers/cerf74.pdf>. Acesso em: 20 Jul. 2026.
