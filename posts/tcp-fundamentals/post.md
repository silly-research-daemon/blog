## Índice

- [Introdução](#intro)
- [Historicamente o TCP e suas RFCs](#intro-tcp)
- [Referências](#reference)

<a id="intro"></a>

## Introdução

O conteúdo sobre o protocolo TCP é extremamente rico, encontrado com facilidade em diversas páginas web e vídeos no youtube, então o material proposto aqui vem com o objetivo de apoiar todo esse conteúdo fornecido na internet e também servirá para minha documentação própria sobre o meu aprendizado e na produção de um artigo (amador, não acadêmico) sobre "Comunicação com CLPs Siemens da linha S7: fundamentos do protocolo S7comm e integração em C/C++" que venho trabalhando. Neste artigo que citei, tenho como objetivo a documentação de uma etapa técnica para a comunicação com CLPs Siemens, sem dependência de projetos avulsos, acredito quando se comenta sobre comunicação de CLPs é necessário a profundidade técnica sobre o protocolo industrial construído nele e seus fundamentos. Como no artigo a linguagem e objetivo são outros, construí esse blog para me aprofundar sobre temas que acredito ser importante detalhar (como TCP). Como estou escrevendo isso para fundamentar meu artigo, provável que fique enviesado.

Este material não tem a necessidade de requisitos aprofundados sobre o tema, mas será útil se compreender sobre as camadas do modelo OSI, pacotes e Ethernet/IP. De qualquer forma, com o objetivo de nivelamento vamos passar por esses conteúdos. 

## Historicamente o TCP e suas RFCs

Antes de pontuar a parte técnica exigidade para compreender esse conteúdo, ou mesmo antes de pontuar o próprio conteúdo, é importante compreender sobre as RFCs propostas nelas. Um breve resumo sobre as Request For Comments (RFC):[1](#ref-1) São documentos que descrevem os padrões da internet

<a id="reference"></a>

## Referências

<a id="ref-1"></a>[1] RFC Editor, "What Is an RFC?" [Online] Disponível em: <https://www.rfc-editor.org/series/rfc/>. Acesso em: 20 Jul. 2026.\
<a id="ref-2"></a>[2] J. Postel, "Transmission Control Protocol," RFC 793, IETF, Sep. 1981.
