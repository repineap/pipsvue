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

export enum RegionType {
  Blank,
  LessThan,
  GreaterThan,
  SumsTo,
  AllEqual,
  AllDifferent,
}

export interface Region {
  squares: Position[];
  regionType: RegionType;
  regionValue: number;
  regionColor: string;
  isValid?: boolean;
}

export interface GridModel {
  regions: Region[];
  grid: number[][];
}
