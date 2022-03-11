import styled from "styled-components";

export const Container = styled.div<{size:number, left:number, top:number, sidePos:number}>`
    width: ${props => props.size}px;
    height: ${props => props.size}px;
    position: absolute;
    left: ${props => props.left}px;
    top: ${props => props.top}px;
    background-image: url('./public/assets/char.png');
    background-position: 0px ${props => props.sidePos}px;

    p {
        background-color: white;
        color: #666;
        font-size: 10px;
        font-family: sans-serif;
        padding: 2px;
        margin-left: -5px;
        margin-top: -18px;
        width: 40px;
        border-radius: 5px;
    }
`;