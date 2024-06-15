
/**
 * 
 * 
 * 
 {
    "name": "Frontend",
    "layer": 1,
    "parent": 0,
    "color": "#3498db",
    "id": 1,
    "children": [ 6, 8, 9, 10, 11, 12 ],
    "silberingCount": 5,
    "silberingOrder": 0,
    "parentChain": [ 0 ],
    "position": {
        "x": 560,
        "y": 80
    }
}
 */


export interface DisplayNodePosition {
  x: number;
  y: number;
}

export interface DisplayNode {
  name: string;
  layer: number;
  parent?: number;
  color: string;
  id: number;
  icon?: string;
  children: number[];
  silberingCount: number;
  silberingOrder: number;
  parentChain: number[];
  position: DisplayNodePosition;
}