export enum Rotation {
  Right,
  Down,
  Left,
  Up,
}

export type Domino = {
  id: number;
  left: number;
  right: number;
  rotation: Rotation;
};

export interface Position {
  x: number;
  y: number;
}

export interface DominoPosition {
  domino: Domino;
  position: Position;
  isValid: boolean;
}
