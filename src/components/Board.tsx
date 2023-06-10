import { useTetris } from "../tetris";
import { Cell } from "./Cell";

export function Board() {
  const board = useTetris((t) => t.board);

  return (
    <div className="justify-self-center w-fit select-none border border-neutral-700 shadow-neutral-950 shadow-md bg-neutral-900">
      {board.map((row, rowIndex) => (
        <div className="flex" key={rowIndex}>
          {row.map((cell, cellIndex) => (
            <Cell color={cell} key={cellIndex} />
          ))}
        </div>
      ))}
    </div>
  );
}
