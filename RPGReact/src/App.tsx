import React from "react";
import { useEffect } from "react";
import * as C from './App.styles';
import { Character } from "./components/Character";
import { Modelo } from "./components/ModeloStyled/index";
import { useCharacter } from "./hooks/useCharacter";

const App = () => {
    
    const char = useCharacter('Delphos');

    useEffect(()=>{
        window.addEventListener('keydown', handleKeyDown);
    }, []);    

    const handleKeyDown = (e: KeyboardEvent) => {
        switch(e.code) {
            case 'KeyA':
            case 'ArrowLeft':
                char.moveLeft();
            break;
            case 'KeyW':
            case 'ArrowUp':
                char.moveUp();
            break;
            case 'KeyD':
            case 'ArrowRight':
                char.moveRight();
            break;
            case 'KeyS':
            case 'ArrowDown':
                char.moveDown();
            break;
        }
    }

    return (
        <C.Container>
            <C.Map>
                <Character x={char.x} y={char.y} side={char.side} name={char.name} />
        
            </C.Map>
            <Modelo />
        </C.Container>
    );
}

export default App;