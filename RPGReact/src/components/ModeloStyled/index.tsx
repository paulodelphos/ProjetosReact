import React from "react";
import styled, { createGlobalStyle } from "styled-components";
import { GlobalStyleComponent } from "styled-components";

const GlobalStyle = createGlobalStyle`
    * {
        font-family: sans-serif;
    }
`;

const tamanhoTexto = 30;
const Title = styled.h3`
    font-size: ${tamanhoTexto}px;
    color : orange;
   
    &:hover {
        color: #666;
    }

    a {
        color: inherit;
        text-decoration: none;
    }
`

const PostCardWrapper = styled.article`
    border: 1px solid orange;
    padding: 15px;
    margin-top: 15px;
    margin-left: 5px;
    width: 435px;
    box-shadow: 2px;
`;

export const Modelo = ()=> {
    return (
    <PostCardWrapper>
        <GlobalStyle/>
        <header>
            <Title>
                <a rel="bookmark" href="/the-wet-codebase">RPG Game in React...</a>
                
            </Title>
            <small> 10 de Março de 2022, 1m de visualização</small>
        </header>
            <p> Modelo simplificado para treinamento em Reack Js</p>
    </PostCardWrapper>
        );

}

