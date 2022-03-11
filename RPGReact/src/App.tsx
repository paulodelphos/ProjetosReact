import React from "react";
import * as C from './App.styles';
import { Character } from "./components/Character";
import { Modelo } from "./components/ModeloStyled/index"

const App = () => {
    return (
        <C.Container>
            <C.Map>

                <Character x={5} y={4}/>
                
            </C.Map>
            <Modelo />
        </C.Container>
    );
}

export default App;