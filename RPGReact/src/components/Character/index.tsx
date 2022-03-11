import * as C from './styles';
import {CharacterSides} from '../../types/CharacterSiders';

type Props = {
  name: string;
  x: number;
  y: number;
  side:CharacterSides;
}

export const Character = ({x, y, side, name}:Props) => {
  const size = 30;
  const nome = name;
  const sides = {
    down: 0,
    left: -30,
    right: -60,
    up: -90
  }
  
  return (
        <C.Container
          size={size}
          left={x*size}
          top={y * size}
          sidePos={sides[side]?? 0}
          
        >
          <p>{nome}</p>
        </C.Container>
    )
}
