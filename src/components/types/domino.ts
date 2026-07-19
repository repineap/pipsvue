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
  isValid: boolean;
}

export function regionToString(region: Region): string {
  switch (region.regionType) {
    case RegionType.Blank:
      return "";
    case RegionType.LessThan:
      return `<${region.regionValue}`;
    case RegionType.GreaterThan:
      return `>${region.regionValue}`;
    case RegionType.SumsTo:
      return `${region.regionValue}`;
    case RegionType.AllEqual:
      return `=`;
    case RegionType.AllDifferent:
      return `!=`;
  }
}

export interface GridModel {
  regions: Region[];
  grid: number[][];
}
